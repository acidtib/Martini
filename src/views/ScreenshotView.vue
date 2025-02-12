<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, emit } from '@tauri-apps/api/event';
import { Screenshots } from '../lib/database'

listen('refresh-screenshot-viewer', () => {
  console.log('Refreshing viewer window');
  loadLatestScreenshot();
});

interface Screenshot {
  id: number
  mission_type: string
  name: string
  image: string
  recognized: boolean
  ocr: boolean
  created_at: string
}

const latestScreenshot = ref<Screenshot | null>(null)

const loadLatestScreenshot = async () => {
  try {
    const screenshots = await Screenshots.findAll({
      orderBy: { column: 'created_at', direction: 'DESC' },
      limit: 1
    })
    latestScreenshot.value = (screenshots[0]?.getAttributes() as Screenshot) || null
  } catch (error) {
    console.error('Error loading latest screenshot:', error)
  }
}

// Load screenshot when component mounts
onMounted(() => {
  loadLatestScreenshot()
})

const handleSubmit = () => {
  console.log('this is submit');
  invoke('submit_screenshot', { screenshotId: latestScreenshot.value?.id });
};

const handleNeverMind = async () => {
  await emit('close-screenshot-viewer');
};
</script>

<template>
  <div class="screenshot-viewer">
    <div v-if="latestScreenshot" class="screenshot-container">
      <img :src="`data:image/jpeg;base64,${latestScreenshot.image}`" :alt="latestScreenshot.name" />
      <div class="screenshot-info">
        <p>ID: {{ latestScreenshot.id }}</p>
        <p>Mission Type: {{ latestScreenshot.mission_type }}</p>
        <p>Name: {{ latestScreenshot.name }}</p>
        <p>Recognized: {{ latestScreenshot.recognized }}</p>
        <p>OCR: {{ latestScreenshot.ocr }}</p>
        <p>Taken: {{ new Date(latestScreenshot.created_at).toLocaleString() }}</p>
      </div>
      <div class="button-container">
        <button @click="handleSubmit">Submit</button>
        <button @click="handleNeverMind">Never Mind</button>
      </div>
    </div>
    <div v-else class="no-screenshot">
      No screenshots available
    </div>
  </div>
</template>

<style scoped>
.screenshot-viewer {
  padding: 1rem;
  height: 100vh;
  overflow-y: auto;
  background-color: rgba(255, 255, 255, 0.05);
}

.screenshot-container {
  margin-top: 1rem;
}

.screenshot-container img {
  max-width: 100%;
  height: auto;
  border-radius: 0.5rem;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.screenshot-info {
  margin-top: 1rem;
  padding: 1rem;
  background-color: rgba(255, 255, 255, 0.1);
  border-radius: 0.5rem;
  color: #9ca3af;
}

.screenshot-info p {
  margin: 0.5rem 0;
}

.no-screenshot {
  margin-top: 1rem;
  padding: 2rem;
  text-align: center;
  background-color: rgba(255, 255, 255, 0.1);
  border-radius: 0.5rem;
  color: #9ca3af;
}

.button-container {
  margin-top: 1rem;
  display: flex;
  gap: 1rem;
  padding: 1rem 0;
}

button {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 0.25rem;
  cursor: pointer;
  background-color: #3b82f6;
  color: white;
}

button:hover {
  background-color: #2563eb;
}

button:last-child {
  background-color: #6b7280;
}

button:last-child:hover {
  background-color: #4b5563;
}
</style>