<template>
    <input class="new-item-input" v-model="inputItem" @keydown.enter="$event.target.blur()"
        @blur="$emit('saveNewItem', inputItem)" :placeholder="`Enter ${activeTab} name`" />
</template>

<script setup>
import { ref } from 'vue';

defineProps(['activeTab', 'inputClassName']);
const inputItem = ref('');

</script>

<style scoped>
.new-item-input {
    width: 100%;
    box-sizing: border-box;
    padding: 8px 12px;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 14px;
    background-color: #f9f9f9;
    outline: none;
    transition: border-color 0.2s ease-in-out, box-shadow 0.2s ease-in-out;
}

.new-item-input:focus {
    border-color: #5894f5;
    box-shadow: 0 0 5px rgba(88, 148, 245, 0.5);
    background-color: #fff;
}
</style>