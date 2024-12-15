<template>
    <div class="request-editor">
        <!-- Selected API Name -->
        <div class="api-name">
            <h2> {{ selectedApi?.name }}</h2>
            <h5 v-if="selectedCollection"> collection name : {{ selectedCollection?.name }}</h5>
        </div>

        <Methods :url="url" :method="method" @selectMethod="selectMethod" @analyzeUrl="analyzeUrl"
            @sendRequest="sendRequest" />

        <div class="request-settings">
            <Headers :headers="headers" @updateHeaders="updateHeaders" @addHeader="addHeader"
                @removeHeader="removeHeader" />
            <Parameters :parameters="parameters" @updateParameters="updateParameters" @addParameter="addParameter"
                @removeParameter="removeParameter" />
        </div>

        <!-- Request Body -->
        <div v-if="method !== 'GET'" class="request-body">
            <h3>Request Body</h3>
            <JsonEditorVue class="body-input" v-model="body" mode="text" :main-menu-bar="false" />
        </div>
    </div>
</template>

<script setup>
import { ref, defineProps, watch, defineEmits } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import Methods from '@/components/requestEditor/Methods.vue';
import Headers from '@/components/requestEditor/Headers.vue';
import Parameters from '@/components/requestEditor/Parameters.vue';
import JsonEditorVue from 'json-editor-vue'

// Props: receive selectedApi from the parent (sidebar)
const props = defineProps({
    selectedApi: {
        type: Object,
        default: null,
    },
    selectedCollection: {
        type: Object,
        default: null,
    },
});

// Request State
const method = ref('');
const url = ref('');
const headers = ref([]);
const parameters = ref([]);
const body = ref('');
const emit = defineEmits();


watch(() => props.selectedApi, () => {
    method.value = props.selectedApi.method
    url.value = props.selectedApi.url
    headers.value = props.selectedApi.headers
    parameters.value = props.selectedApi.parameters
    body.value = props.selectedApi.body
    analyzeUrl();
});

watch(() => method.value, () => {
    props.selectedApi.method = method.value;
    if (method.value !== 'GET') {
        if (headers.value.findIndex(h => h.key.toLowerCase() === 'content-type') < 0) {
            headers.value.push({ key: 'Content-Type', value: 'application/json' })
        }
    }
    saveApi(props.selectedApi);
});
watch(() => url.value, () => {
    props.selectedApi.url = url.value;
    saveApi(props.selectedApi);
});
watch(() => headers.value, () => {
    props.selectedApi.headers = headers.value;
    saveApi(props.selectedApi);
}, { deep: true });
watch(() => parameters.value, () => {
    props.selectedApi.parameters = parameters.value;
    saveApi(props.selectedApi);
}, { deep: true });
watch(() => body.value, () => {
    props.selectedApi.body = body.value;
    saveApi(props.selectedApi);
});

// Save api to backend
const saveApi = async (api) => {
    const serializedData = JSON.stringify(api);
    await invoke('save_api', { data: serializedData });
};

const selectMethod = (newMethod) => { method.value = newMethod }

const updateHeaders = (newHeaders) => { headers.value = newHeaders }
const addHeader = () => headers.value.push({ key: '', value: '' });
const removeHeader = (index) => headers.value.splice(index, 1);

const updateParameters = (newParameters) => { parameters.value = newParameters }
const addParameter = () => parameters.value.push({ parameter_type: 'QUERY', key: '', value: '' });
const removeParameter = (index) => parameters.value.splice(index, 1);

// Analyze URL to extract query and path parameters
const analyzeUrl = (inputUrl) => {
    if (!inputUrl) return;
    url.value = inputUrl.trim()
    const currentParams = [...parameters.value];
    const urlWithoutQuery = url.value.split('?')[0];
    const queryString = url.value.split('?')[1];

    parameters.value = [];

    // Extract Path Params
    const pathParams = urlWithoutQuery.match(/{([^}]+)}/g);
    if (pathParams) {
        pathParams.forEach((param) => {
            const key = param.replace(/[{}]/g, '')
            const existingParam = currentParams.find(p => p.key === key && p.parameter_type === 'PATH')
            parameters.value.push(existingParam || { parameter_type: 'PATH', key, value: '' })
        });
    }

    // Extract Query Params
    if (queryString) {
        const queries = queryString.split('&');
        queries.forEach((query) => {
            const [key, value] = query.split('=');
            const decodeKey = decodeURIComponent(key)
            const decodeValue = decodeURIComponent(value || '')
            const existingParam = currentParams.find(p => p.key === decodeKey && p.parameter_type === 'QUERY')
            parameters.value.push(existingParam || { parameter_type: 'QUERY', key: decodeKey, value: decodeValue })
        });
    }

    currentParams.forEach(param => {
        if (!parameters.value.find(p => p.key === param.key && p.parameter_type === param.parameter_type)) {
            parameters.value.push(param)
        }
    })
};

// Send Request
const sendRequest = async () => {
    try {
        const fullUrl = buildUrlWithParameters(url.value, parameters.value);
        const requestHeaders = headers.value.reduce((header, current) => {
            if (current.key && current.value) {
                header.push({ key: current.key, value: current.value })
            }
            return header
        }, []);
        emit('request', {
            method: method.value,
            url: fullUrl,
            headers: requestHeaders,
            body: body.value
        })
    } catch (error) {
        console.error('Request failed:', error);
    }
};

// Helper: Build URL with Parameters
const buildUrlWithParameters = (baseUrl, params) => {
    let finalUrl = baseUrl;

    // Replace Path Params
    params
        .filter((param) => param.parameter_type === 'PATH' && param.key)
        .forEach((param) => {
            finalUrl = finalUrl.replace(`{${param.key}}`, encodeURIComponent(param.value));
        });

    // Append Query Params
    const queryParams = params
        .filter((param) => param.parameter_type === 'QUERY' && param.key)
        .map((param) => `${encodeURIComponent(param.key)}=${encodeURIComponent(param.value)}`)
        .join('&');
    return queryParams ? `${finalUrl.split('?')[0]}?${queryParams}` : finalUrl;
};
</script>

<style scoped>
.request-editor {
    min-width: 520px;
    width: 50%;
    padding: 10px;
    border-right: 1px solid #ddd;
}

.api-name h2 {
    margin: 0 0 0px;
    font-size: 24px;
}

.api-name h5 {
    margin: 0 0 0px;
    font-size: 15px;
}

.request-settings {
    display: flex;
    width: 100%;
}

.request-settings h3,
.request-body h3 {
    margin: 0;
    font-size: 15px;
    display: flex;
}
</style>