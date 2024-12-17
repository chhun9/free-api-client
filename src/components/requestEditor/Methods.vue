<template>
    <div class="method-url">
        <select v-model="selectMethod" class="method-select">
            <option v-for="method in methods" :key="method" :value="method">
                {{ method }}
            </option>
        </select>
        <input v-model="inputUrl" type="text" class="url-input" placeholder="Enter API URL"
            @blur="$emit('analyzeUrl', inputUrl)" />
        <button @click="$emit('sendRequest')" class="send-btn" :disabled="isLoading">Send</button>
    </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useHttpUtils } from '@/composables/useHttpUtils';
const props = defineProps(['url', 'method', 'isLoading']);
const emit = defineEmits();

const methods = ref([]);
const selectMethod = ref('')
const inputUrl = ref();
const { getAllMethods } = useHttpUtils();

onMounted(async () => {
    methods.value = await getAllMethods();
});
watch(() => props.url, () => {
    inputUrl.value = props.url
});
watch(() => selectMethod.value, () => {
    emit('selectMethod', selectMethod.value);
});
watch(() => props.method, () => {
    selectMethod.value = props.method
})
</script>

<style scoped>
.method-url {
    display: flex;
    gap: 10px;
    margin-bottom: 10px;
}

.method-select,
.url-input,
.send-btn {
    padding: 5px;
    font-size: 14px;
}

.url-input {
    width: 100%;
}

.send-btn {
    background-color: #4caf50;
    color: white;
    border: none;
    cursor: pointer;
}

.send-btn:hover {
    background-color: #45a049;
}
</style>
