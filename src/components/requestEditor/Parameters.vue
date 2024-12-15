<template>
    <div class="parameters-settings">
        <h3 class="parameter">Parameters
            <div @click="$emit('addParameter')" class="add-btn">
                + Add Parameter
            </div>
        </h3>
        <div v-for="(param, index) in inputParameters" :key="index" class="key-value-pair">
            <select v-model="param.parameter_type" class="param-type-select">
                <option v-for="parameter in parameterOptions" :key="parameter" :value="parameter">{{ parameter }}
                </option>
            </select>
            <input v-model="param.key" type="text" placeholder="Key" class="key-input" @blur="
                $emit('updateParameters', inputParameters)" />
            <input v-model="param.value" type="text" placeholder="Value" class="value-input" @blur="
                $emit('updateParameters', inputParameters)" />
            <div @click="$emit('removeParameter', index)" class="remove-btn">X</div>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useHttpUtils } from '@/composables/useHttpUtils';
const { getAllPrameters } = useHttpUtils();
const props = defineProps(['parameters']);
const emit = defineEmits();

const inputParameters = ref([]);
const parameterOptions = ref([])

onMounted(async () => {
    parameterOptions.value = await getAllPrameters()
});
watch(() => props.parameters, () => {
    inputParameters.value = props.parameters
});
</script>

<style scoped>
.parameters-settings {
    width: 100%;
}

.parameter {
    margin: 0;
    font-size: 15px;
    display: flex;
}

.param-type-select {
    padding: 6px;
    font-size: 14px;
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
