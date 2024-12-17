<template>
    <div v-if="visible" :style="{ top: y + 'px', left: x + 'px' }" class="context-menu">
        <button class="menu-item-name">
            <template v-if="isRenaming">
                <input class="menu-item-input" v-model="editedName" @keydown.enter="saveRename"
                    placeholder="Enter name" />
                <button @click="saveRename">save</button>
            </template>
            <template v-else>
                {{ `${selectItem.name} (${selectItem.apis ? 'collection' : 'api'})` }}
            </template>
        </button>
        <button class="menu-item" @click="renameEvent">Rename</button>
        <button class="menu-item" @click="$emit('delete')">Delete</button>
        <button class="menu-item" @click="$emit('duplicate')">Duplicate</button>
    </div>
</template>

<script setup>
import { ref, defineEmits, defineProps, watch } from 'vue';
const props = defineProps(['visible', 'x', 'y', 'selectItem']);
const emit = defineEmits(['saveRename']);

const isRenaming = ref(false)
const editedName = ref('')

watch(() => props.visible, () => {
    saveRename()
});

const renameEvent = () => {
    isRenaming.value = true

    nextTick(() => {
        const input = document.querySelector('.menu-item-input');
        if (input) input.focus();
    });
}

const saveRename = () => {
    isRenaming.value = false
    if (editedName.value?.trim() === '')
        return;
    emit('saveRename', editedName.value)
    editedName.value = ''
}

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
    max-width: 250px;
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
    display: flex;
    align-items: center;
    justify-content: space-between;
    /* Adjust spacing between input and button */
    color: gray;
    border-bottom: 1px solid #eaeaea;
    padding: 8px;
}

.menu-item-input {
    padding: 4px 8px;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 13px;
    /* Slightly smaller font size */
    background-color: #f9f9f9;
    outline: none;
    transition: border-color 0.2s ease-in-out, box-shadow 0.2s ease-in-out;
}

.menu-item-input:focus {
    border-color: #5894f5;
    box-shadow: 0 0 5px rgba(88, 148, 245, 0.5);
}

button {
    padding: 4px 8px;
    font-size: 13px;
    border: 1px solid #ccc;
    border-radius: 4px;
    background-color: #fff;
    cursor: pointer;
    transition: background-color 0.2s ease;
}

button:hover {
    background-color: #f5f5f5;
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
