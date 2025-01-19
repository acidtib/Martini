<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Screenshot } from '../lib/database'
import type { ScreenshotAttributes } from '../lib/database'

const latestScreenshot = ref<ScreenshotAttributes | null>(null)

const loadLatestScreenshot = async () => {
  try {
    latestScreenshot.value = (await Screenshot.latest())?.attributes || null
  } catch (error) {
    console.error('Error loading latest screenshot:', error)
  }
}

// Load screenshot when component mounts
onMounted(() => {
  loadLatestScreenshot()
})
</script>

<template>
  <div class="screenshot-viewer">
    <div v-if="latestScreenshot" class="screenshot-container">
      <img :src="`data:image/png;base64,${latestScreenshot.image}`" :alt="latestScreenshot.name" />
      <div class="screenshot-info">
        <p>Name: {{ latestScreenshot.name }}</p>
        <p>Taken: {{ new Date(latestScreenshot.createdAt).toLocaleString() }}</p>
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
}

.screenshot-container {
  margin-top: 1rem;
}

.screenshot-container img {
  max-width: 100%;
  height: auto;
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.screenshot-info {
  margin-top: 1rem;
  padding: 1rem;
  background-color: #f5f5f5;
  border-radius: 4px;
}

.no-screenshot {
  margin-top: 1rem;
  padding: 2rem;
  text-align: center;
  background-color: #f5f5f5;
  border-radius: 4px;
  color: #666;
}
</style>