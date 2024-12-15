<template>
    <div class="tabs">
        <div :class="['tab', { active: activeTab === 'collections' }]" @click="$emit('changeTab', 'collections')">
            Collections
        </div>
        <div :class="['tab', { active: activeTab === 'apis' }]" @click="$emit('changeTab', 'apis')">
            APIs
        </div>
    </div>
</template>

<script setup>
defineProps(['activeTab']);
</script>

<style scoped>
.tabs {
    display: flex;
    margin-bottom: 10px;
    gap: 10px;
}

.tab {
    flex: 1;
    padding: 10px;
    text-align: center;
    cursor: pointer;
    background-color: #e4e4e4;
    border-radius: 5px;
}

.tab:hover {
    background-color: #b3b3b3;
}

.tab.active {
    background-color: #a1c4fd;
    font-weight: bold;
}

.tab.active:hover {
    background-color: #8fb7f7;
    font-weight: bold;
}
</style>