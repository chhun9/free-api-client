<template>
    <div class="item-list">
        <div v-for="(collection, index) in collections" :key="index" class="item"
            @contextmenu.prevent="$emit('showContextMenu', $event, collection)">
            <div @click="$emit('selectCollection', collection)"
                :class="{ highlighted: activeCollection?.id === collection.id }">
                {{ activeCollection?.id === collection.id ? '▼' : '►' }} {{ collection.name }}
            </div>
            <div v-if="activeCollection?.id === collection.id" class="api-list">
                <AddingItem v-if="isAddingItemInCollection" :activeTab="`api in collection`"
                    @saveNewItem="(newItem) => $emit('saveNewItem', newItem)" />
                <slot :collection="collection" />
                <button class="btn-in-collection" @click="$emit('createNewApiInCollection', e, true)">
                    New API in {{ activeCollection.name }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>
import AddingItem from '@/components/sidebar/AddingItem.vue';

defineProps(['collections', 'activeCollection', 'isAddingItemInCollection']);
</script>

<style scoped>
.item-list {
    margin-top: 10px;
}

.item:hover {
    cursor: pointer;
}

.item .highlighted {
    background-color: #4d66b5;
    font-weight: bold;
    overflow: hidden;
    color: white;
    text-overflow: ellipsis;
}

.api-list button {
    margin-top: 10px;
    padding: 10px;
    background-color: #7cc97f;
    color: white;
    border: none;
    cursor: pointer;
}

.api-list button:hover {
    background-color: #78bd7a;
}
</style>