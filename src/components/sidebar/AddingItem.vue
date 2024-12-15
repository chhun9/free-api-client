<template>
    <input class="new-item-input" v-model="inputItem" @keydown.enter="$emit('saveNewItem', inputItem)"
        @blur="$emit('saveNewItem', inputItem)" :placeholder="`Enter ${activeTab} name`" />
</template>

<script setup>
import { ref } from 'vue';

defineProps(['activeTab', 'inputClassName']);
const inputItem = ref('');

</script>

<style scoped>
input {
    width: 100%;
}
</style>