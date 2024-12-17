<template>
    <div class="sidebar">
        <SidebarTabs :activeTab="activeTab" @changeTab="setActiveTab" />

        <div class="create-new" @click="createNewItem">
            {{ isActiveCollectionTab ? 'New Collection' : 'New API' }}
        </div>

        <AddingItem v-if="isAddingItem" :activeTab="activeTab" @saveNewItem="saveNewItem" />

        <Collections v-if="isActiveCollectionTab" :collections="collections"
            :isAddingItemInCollection="isAddingItemInCollection" :activeCollection="activeCollection"
            @saveNewItem="saveNewItem" @selectCollection="selectCollection" @createNewApiInCollection="createNewItem"
            @showContextMenu="showContextMenu">

            <template v-slot:default="{ collection }">
                <Apis :apis="collection.apis" :selectedApi="selectedApi" @selectApi="selectApi"
                    @showContextMenu="showContextMenu" />
            </template>
        </Collections>

        <Apis v-if="isActiveApiTab" :apis="apis" :selectedApi="selectedApi" @selectApi="selectApi"
            @showContextMenu="showContextMenu" />

        <SidebarContextMenu :visible="contextMenu.visible" :x="contextMenu.x" :y="contextMenu.y"
            :selectItem="contextMenu.item" @delete="deleteItem" @saveRename="saveRename" @duplicate="duplicateItem" />
    </div>
</template>

<script setup>
import SidebarTabs from '@/components/sidebar/SidebarTabs.vue'
import AddingItem from '@/components/sidebar/AddingItem.vue'
import Collections from '@/components/sidebar/Collections.vue'
import Apis from '@/components/sidebar/Apis.vue'
import SidebarContextMenu from '@/components/sidebar/SidebarContextMenu.vue'
import { ref, onMounted, nextTick, defineEmits, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Local state
const collections = ref([])
const apis = ref([])
const activeTab = ref('collections')
const activeCollection = ref(null)
const isAddingItem = ref(false)
const isAddingItemInCollection = ref(null)
const newItemName = ref('')
const contextMenu = ref({ visible: false, x: 0, y: 0, item: null })
const selectedApi = ref(null)
const emit = defineEmits()

onMounted(() => {
    loadData()
    document.addEventListener('click', handleOutsideClick)
})

onUnmounted(() => {
    document.removeEventListener('click', handleOutsideClick)
})

const isActiveCollectionTab = computed(() => activeTab.value === 'collections')
const isActiveApiTab = computed(() => activeTab.value === 'apis')
const isAddingToCollections = computed(() => isActiveCollectionTab.value && !activeCollection.value)
const isAddingToApiInCollections = computed(() => isActiveCollectionTab.value && activeCollection.value)

const loadData = async () => {
    try {
        const data = await invoke('read_data')
        collections.value = data.collections
        apis.value = data.apis

        if (activeCollection.value) {
            activeCollection.value = collections.value.find(c => c.id === activeCollection.value.id)
        }

        if (selectedApi.value) {
            if (activeCollection.value) {
                selectedApi.value = activeCollection.value.apis.find(a => a.id === selectedApi.value.id)
            } else {
                selectedApi.value = apis.value.find(a => a.id === selectedApi.value.id)
            }
        }
        selectApi(selectedApi.value)
    } catch (err) {
        console.error('Error loading data:', err)
    }
}

const setActiveTab = (tab) => {
    activeTab.value = tab
    activeCollection.value = null
}

const createNewItem = (e, isInCollection = false) => {
    newItemName.value = ''
    isAddingItem.value = !isInCollection
    isAddingItemInCollection.value = isInCollection
    activeCollection.value = isInCollection ? activeCollection.value : null

    focusNewItemInput()
}

const focusNewItemInput = () => {
    nextTick(() => {
        const input = document.querySelector('.new-item-input')
        if (input) input.focus()
    })
}

const saveNewItem = async (inputItem) => {
    newItemName.value = inputItem?.trim()

    if (!newItemName.value) {
        resetAddingState()
        return
    }

    const newItem = createItem(newItemName.value)
    const serializedData = JSON.stringify(newItem)
    if (isAddingToCollections.value) {
        activeCollection.value = newItem
        await invoke('save_new_collection', { data: serializedData }).then(retId => {
            activeCollection.value.id = retId
            loadData()
        })
    } else {
        selectedApi.value = newItem
        if (isAddingToApiInCollections.value) {
            await invoke('save_new_api', { data: serializedData, collectionId: activeCollection.value.id }).then(retId => {
                selectedApi.value.id = retId
                loadData()
            })
        } else {
            await invoke('save_new_api', { data: serializedData, collectionId: '' }).then(retId => {
                selectedApi.value.id = retId
                loadData()
            })
        }
    }

    resetAddingState()
}

const createItem = (name) => {
    return isActiveCollectionTab.value && !isAddingItemInCollection.value
        ? { id: '', name, apis: [] }
        : { id: '', name, method: 'GET', url: '', headers: [], parameters: [], body: '' }
}

const resetAddingState = () => {
    newItemName.value = ''
    isAddingItem.value = false
    isAddingItemInCollection.value = false
}

const selectCollection = (collection) => {
    if (collection) {
        activeCollection.value = collection
        selectedApi.value = null
        emit('selectApi', selectedApi.value)
        emit('selectCollection', activeCollection.value)
    }
}

const selectApi = (api) => {
    if (api) {
        selectedApi.value = api
        emit('selectApi', selectedApi.value)
        if (isActiveApiTab.value) {
            activeCollection.value = null
            emit('selectCollection', activeCollection.value)
        }
    }
}

const showContextMenu = (event, item) => {
    event.preventDefault()
    contextMenu.value = { visible: true, x: event.clientX, y: event.clientY, item }
}

const handleOutsideClick = (event) => {
    const contextMenuElement = document.querySelector('.context-menu')

    if (contextMenuElement && !contextMenuElement.contains(event.target)) {
        hideContextMenu()
    }
}

const hideContextMenu = () => {
    contextMenu.value.visible = false
}

const deleteItem = async () => {
    await invoke('delete_data', { id: contextMenu.value.item.id }).then(() => {
        loadData()
        hideContextMenu()
    })
}

const saveRename = async (itemName) => {
    let selectId = contextMenu.value.item.id

    await invoke('rename_data', { id: contextMenu.value.item.id, newName: itemName }).then(() => {
        loadData()
        hideContextMenu()
    })
}

const duplicateItem = async () => {
    let selectId = contextMenu.value.item.id

    await invoke('duplicate_data', { id: contextMenu.value.item.id }).then(() => {
        loadData()
        hideContextMenu()
    })
}
</script>

<style scoped>
.sidebar {
    min-width: 250px;
    max-width: 250px;
    padding: 10px;
    border-right: 1px solid #ddd;
}

.create-new {
    flex: 1;
    padding: 10px;
    text-align: center;
    cursor: pointer;
    background-color: #5894f5;
    font-weight: bold;
    border-radius: 5px;
}

.create-new:hover {
    background-color: #4b8cf4;
}
</style>