<template>
    <div v-if="visible" :style="{ top: y + 'px', left: x + 'px' }" class="context-menu">
        <button class="menu-item-name">{{ `${deleteItem.name} (${deleteItem.apis ? 'collection' : 'api'})` }}</button>
        <button class="menu-item" @click="$emit('edit')">Edit</button>
        <button class="menu-item" @click="$emit('delete')">Delete</button>
        <button class="menu-item" @click="$emit('duplicate')">Duplicate</button>
    </div>
</template>

<script setup>
defineProps(['visible', 'x', 'y', 'deleteItem']);
</script>

<style scoped>
.context-menu {
    position: absolute;
    background-color: #fff;
    border-radius: 8px;
    box-shadow: 0 0 15px rgba(0, 0, 0, 0.3);
    padding: 10px;
    z-index: 1000;
    min-width: 120px;
    overflow: hidden;
}


.menu-item-name,
.menu-item {
    padding: 8px 12px;
    background: transparent;
    border: none;
    text-align: left;
    cursor: pointer;
    font-weight: 500;
    font-size: 14px;
    width: 100%;
    display: block;
    text-align: center;
    transition: background 0.2s ease;
}

.menu-item-name {
    color: gray;
    border-bottom: 1px solid #eaeaea;
}

.menu-item:hover {
    background-color: #f5f5f5;
}

.menu-item:active {
    background-color: #eaeaea;
}

.menu-item:focus {
    outline: none;
}
</style>
