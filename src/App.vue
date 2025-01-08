<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { listen } from '@tauri-apps/api/event';

const screenshotData = ref<string | null>(null);

onMounted(() => {
  listen('screenshot-taken', (event: any) => {
    screenshotData.value = event.payload as string;
  });
});
</script>

<template>
  <main class="container">
    <div class="welcome-content">
      <h1 class="title">Welcome to <span class="highlight">Martini</span></h1>
      <div class="card">
        <p class="description">
          This is a tech demo that showcases screen capture capabilities.
        </p>
        <div class="shortcut-container">
          <p class="shortcut-text">To try it out, press</p>
          <div class="shortcut-keys">
            <kbd>Control</kbd>+<kbd>Shift</kbd>+<kbd>M</kbd>
          </div>
        </div>
      </div>

      <!-- Screenshot display area -->
      <div v-if="screenshotData" class="screenshot-container">
        <img :src="screenshotData" alt="Screenshot" class="screenshot" />
      </div>
    </div>
  </main>
</template>

<style scoped>
.container {
  min-height: 100vh;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  text-align: center;
  padding: 2rem;
  background: linear-gradient(135deg, #f5f7fa 0%, #e4e8eb 100%);
}

.welcome-content {
  max-width: 800px;
  animation: fadeIn 0.8s ease-out;
  width: 100%;
}

.title {
  font-size: 3rem;
  margin-bottom: 2rem;
  color: #2c3e50;
  font-weight: 700;
  letter-spacing: -0.5px;
}

.highlight {
  background: linear-gradient(120deg, #3498db, #2980b9);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  padding: 0 0.2em;
}

.card {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 16px;
  padding: 2rem;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.08);
  backdrop-filter: blur(5px);
  transition: transform 0.3s ease;
  margin-bottom: 2rem;
}

.card:hover {
  transform: translateY(-5px);
}

.description {
  font-size: 1.25rem;
  line-height: 1.6;
  color: #34495e;
  margin-bottom: 1.5rem;
}

.shortcut-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.75rem;
}

.shortcut-text {
  font-size: 1.1rem;
  color: #666;
}

.shortcut-keys {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

kbd {
  background-color: #ffffff;
  border: 1px solid #e1e4e8;
  border-radius: 6px;
  box-shadow: 0 2px 5px rgba(0,0,0,0.05);
  display: inline-block;
  font-family: 'SF Mono', 'Segoe UI Mono', monospace;
  font-size: 1rem;
  font-weight: 600;
  line-height: 1;
  padding: 0.5em 0.75em;
  color: #0366d6;
  transition: all 0.2s ease;
}

kbd:hover {
  transform: translateY(-1px);
  box-shadow: 0 3px 6px rgba(0,0,0,0.1);
}

.screenshot-container {
  margin-top: 2rem;
  background: rgba(255, 255, 255, 0.95);
  border-radius: 16px;
  padding: 1rem;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.08);
  backdrop-filter: blur(5px);
  animation: fadeIn 0.5s ease-out;
}

.screenshot {
  max-width: 100%;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@media (max-width: 640px) {
  .title {
    font-size: 2.5rem;
  }
  
  .card {
    padding: 1.5rem;
  }
  
  .description {
    font-size: 1.1rem;
  }
}
</style>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}
</style>