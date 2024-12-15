<template>
    <div>
        <div v-for="(api, index) in apis" :key="index" :class="['item-api', { highlighted: selectedApi.id === api.id }]"
            @click="$emit('selectApi', api)" @contextmenu.stop="$emit('showContextMenu', $event, api)">
            <div class="item-api-method" :style="{ backgroundColor: getMethodColor(api.method) }">
                {{ api.method }}
            </div>
            {{ api.name }}
        </div>
    </div>
</template>

<script setup>
import { useHttpUtils } from '@/composables/useHttpUtils';
const { getMethodColor } = useHttpUtils();
defineProps(['apis', 'selectedApi']);
</script>

<style scoped>
.item-api {
    margin-top: 3px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    display: flex;
    gap: 5px;
}

.item-api:hover {
    background-color: #7371d5;
    font-weight: bold;
    cursor: pointer;
}

.item-api.highlighted {
    background-color: #4d66b5;
    font-weight: bold;
    overflow: hidden;
    color: white;
    text-overflow: ellipsis;
}

.item-api-method {
    width: 42px;
    font-weight: bold;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 10px;
    color: black;
    opacity: 0.8;
}
</style>