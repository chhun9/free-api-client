<script setup>
import { ref, defineProps, defineEmits, computed } from 'vue';
import VueJsonPretty from 'vue-json-pretty';
import 'vue-json-pretty/lib/styles.css';

const props = defineProps({
    response: [String, Object],
    isLoading: Boolean,
});
const emit = defineEmits(['cancel']);

const tabs = ref(['Body', 'Headers']);
const activeTab = ref('Body');

const statusColor = computed(() => {
    if (!props.response?.status_code) return '';
    if (props.response?.status_code >= 200 && props.response?.status_code < 300) return 'green';
    if (props.response?.status_code >= 300 && props.response?.status_code < 400) return 'blue';
    if (props.response?.status_code >= 400 && props.response?.status_code < 500) return 'orange';
    if (props.response?.status_code >= 500) return 'red';
    return '';
});

const copyStatus = ref('');

const copyToClipboard = async () => {
    try {
        const textToCopy = typeof props.response?.body === 'object' ? JSON.stringify(props.response?.body, null, 2) : props.response?.body;
        await navigator.clipboard.writeText(textToCopy);
        copyStatus.value = 'Copied to clipboard!';
        setTimeout(() => {
            copyStatus.value = '';
        }, 2000);
    } catch (err) {
        copyStatus.value = 'Failed to copy';
        setTimeout(() => {
            copyStatus.value = '';
        }, 2000);
    }
};
</script>

<template>
    <div class="response-viewer">
        <h2>
            <p>Response</p>
            <p v-if="response?.status_code" :style="{ backgroundColor: statusColor, color: 'white' }"
                class="status-code">
                Status Code: {{ response?.status_code }}
            </p>
        </h2>
        <div v-if="isLoading" class="loading-overlay">
            <p>Loading...</p>
            <button @click="emit('cancel')">Cancel</button>
        </div>
        <div v-else>
            <div class="tabs">
                <button v-for="tab in tabs" :key="tab" :class="['tab', { active: activeTab === tab }]"
                    @click="activeTab = tab">
                    {{ tab }}
                </button>
            </div>
            <div class="tab-content">
                <div v-if="activeTab === 'Body'" class="tab-body">
                    <button :class="`copy-button${copyStatus ? '-copied' : ''}`" @click="copyToClipboard">
                        {{ copyStatus ? copyStatus : 'Copy to Clipboard' }}
                    </button>

                    <VueJsonPretty v-if="response?.body" :data="response?.body" showIcon showLineNumber showLine
                        showLength showDoubleQuotes showKeyValueSpace collapseOnClickBrackets virtual :height="600" />
                </div>
                <div v-if="activeTab === 'Headers'" class="tab-headers">
                    <pre>{{ response?.headers }}</pre>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.response-viewer {
    width: 50%;
    padding: 10px;
    border-right: 1px solid #ddd;
}

.request-editor {
    min-width: 520px;
    width: 50%;
    padding: 10px;
    border-right: 1px solid #ddd;
}

h2 {
    display: flex;
    margin: 0 0 0px;
    font-size: 24px;
}

h2 p {
    margin: 0 0 0px;
}

.tabs {
    display: flex;
    border-bottom: 1px solid #ddd;
}

.tab {
    padding: 0.5rem 1rem;
    cursor: pointer;
    border: none;
    background: none;
    font-size: 1rem;
    color: #555;
}

.tab.active {
    font-weight: bold;
    color: #000;
    border-bottom: 2px solid #007bff;
}

.tab-body {
    position: relative;
}

.copy-button {
    padding: 5px 10px;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.copy-button-copied {
    padding: 5px 10px;
    background-color: #57b365;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.copy-button:hover {
    background-color: #0056b3;
}

.copy-status {
    margin-top: 5px;
    font-size: 0.9rem;
    color: #28a745;
}

.status-code {
    margin-left: auto;
    text-align: right;
}

.loading-overlay {
    position: relative;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
    background: rgba(255, 255, 255, 0.8);
    padding: 1rem;
    border: 1px solid #ccc;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
    z-index: 10;
}

.loading-overlay button {
    margin-top: 1rem;
    padding: 0.5rem 1rem;
    background: #e74c3c;
    color: #fff;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.loading-overlay button:hover {
    background: #c0392b;
}
</style>
