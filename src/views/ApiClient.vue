<template>
  <div class="api-client">
    <Sidebar @selectApi="handleApiFromSidebar" @selectCollection="handleCollectionFromSidebar" />

    <RequestEditor :style="{ width: editorWidth + '%' }" :selectedApi="selectedApi"
      :selectedCollection="selectedCollection" @request="handleRequest" />

    <div class="request-resizer" @mousedown="startResize" />

    <ResponseViewer :isLoading="isLoading" :response="response" @cancel="cancelRequest" />


  </div>
</template>

<script setup>
import { ref, toRaw } from 'vue'
import Sidebar from '@/views/Sidebar.vue';
import RequestEditor from '@/views/RequestEditor.vue';
import ResponseViewer from '@/views/ResponseViewer.vue';
import { useApiHandler } from '@/composables/useApiHandler';

const { sendRequest, cancelRequest, isLoading, response } = useApiHandler();
const selectedApi = ref(null);
const selectedCollection = ref(null);
const editorWidth = ref(50);
const startX = ref(0);
const startWidth = ref(0);

const isEmptyString = str => {
  return !str || str.trim() === '';
}

const handleRequest = async (request) => {
  await sendRequest(request);
};

const handleApiFromSidebar = (selectedApiFromSidebar) => {
  if (!selectedApiFromSidebar) return;

  selectedApi.value = selectedApiFromSidebar;
  selectedApi.value.headers = selectedApi.value.headers?.filter(h=>!isEmptyString(h.key) || !isEmptyString(h.value))
  selectedApi.value.parameters = selectedApi.value.parameters?.filter(p=>!isEmptyString(p.key) || !isEmptyString(p.value))
};

const handleCollectionFromSidebar = (selectedCollectionFromSidebar) => {
  selectedCollection.value = selectedCollectionFromSidebar;
};

const startResize = (e) => {
  startX.value = e.clientX;
  startWidth.value = editorWidth.value;
  window.addEventListener('mousemove', doResize);
  window.addEventListener('mouseup', stopResize);
};

const doResize = (e) => {
  const diff = e.clientX - startX.value;
  editorWidth.value = Math.min(Math.max(startWidth.value + (diff / window.innerWidth * 100), 10), 90);
};

const stopResize = (e) => {
  window.removeEventListener('mousemove', doResize);
  window.removeEventListener('mouseup', stopResize)
};
</script>

<style>
.api-client {
  display: flex;
  height: 100vh;
  width: 100vw;
}

.sidebar,
.request-editor,
.response-viewer {
  overflow-y: auto;
  overflow-x: auto;
}

.request-resizer {
  width: 3px;
  border-left: solid 1px;
  border-right: solid 1px;
  cursor: ew-resize;
  transition: top ease-out 0.5s;
}

.request-resizer:hover {
  background-color: #aaa;
}
</style>
