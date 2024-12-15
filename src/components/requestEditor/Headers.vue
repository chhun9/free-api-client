<template>
    <div class="headers-settings">
        <h3 class="header">Headers
            <div @click="$emit('addHeader')" class="add-btn">
                + Add Header
            </div>
        </h3>
        <div v-for="(header, index) in inputHeaders" :key="index" class="key-value-pair">
            <input v-model="header.key" type="text" placeholder="Key" class="key-input" @blur="
                $emit('updateHeaders', inputHeaders)" />
            <input v-model="header.value" type="text" placeholder="Value" class="value-input" @blur="
                $emit('updateHeaders', inputHeaders)" />
            <div @click="$emit('removeHeader', index)" class="remove-btn">X</div>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useHttpUtils } from '@/composables/useHttpUtils';
const { getAllHeaders } = useHttpUtils();
const props = defineProps(['headers']);
const emit = defineEmits();

const inputHeaders = ref([]);
const headerKeys = ref([])

onMounted(async () => {
    headerKeys.value = await getAllHeaders()
});
watch(() => props.headers, () => {
    inputHeaders.value = props.headers
});

</script>

<style scoped>
.headers-settings {
    width: 100%;
}

.header {
    margin: 0;
    font-size: 15px;
    display: flex;
}

.key-value-pair {
    display: flex;
    gap: 5px;
    margin-bottom: 5px;
}

.key-input {
    width: 35%;
}

.value-input {
    width: 45%;
}

.add-btn,
.remove-btn {
    color: gray;
    font-size: 11px;
    cursor: pointer;
}

.add-btn {
    margin-left: 15px;
}

.add-btn:hover {
    background-color: #e3e2e2;
}

.remove-btn {
    width: 10px;
}

.remove-btn:hover {
    background-color: #e3e2e2;
}
</style>
