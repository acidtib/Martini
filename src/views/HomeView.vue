<script>
import { listen } from '@tauri-apps/api/event'

export default {
  data() {
    return {
      screenshotStatus: '',
      unlisten: null,
      statusTimer: null
    }
  },
  async mounted() {
    this.unlisten = await listen('screenshot-status', (event) => {
      this.screenshotStatus = event.payload
      
      // Clear any existing timer
      if (this.statusTimer) {
        clearTimeout(this.statusTimer)
        this.statusTimer = null
      }

      // Set timer for final states
      if (event.payload === 'detected' || event.payload === 'not-detected') {
        this.statusTimer = setTimeout(() => {
          this.screenshotStatus = ''
        }, 10000) // 10 seconds
      }
    })
  },
  unmounted() {
    if (this.unlisten) {
      this.unlisten()
    }
    if (this.statusTimer) {
      clearTimeout(this.statusTimer)
    }
  },
  methods: {
    goToSettings() {
      this.$router.push('/settings')
    },
  },
}
</script>

<template>
  <div class="home">
    <h2>HomeView</h2>
    <div v-if="screenshotStatus" class="status-banner">
      <p>
        <span v-if="screenshotStatus === 'capturing'">📸 Taking screenshot...</span>
        <span v-else-if="screenshotStatus === 'cropping'">✂️ Cropping image...</span>
        <span v-else-if="screenshotStatus === 'recognizing'">🔍 Analyzing image...</span>
        <span v-else-if="screenshotStatus === 'detected'">✅ Mission Summary detected!</span>
        <span v-else-if="screenshotStatus === 'not-detected'">❌ No Mission Summary found</span>
      </p>
    </div>
    <button @click="goToSettings">Go to Settings</button>
  </div>
</template>

<style scoped>
.home {
  padding: 1rem;
}

.status-banner {
  margin: 1rem 0;
  padding: 0.75rem;
  border-radius: 0.5rem;
  background-color: #f3f4f6;
  text-align: center;
}

.status-banner p {
  margin: 0;
  font-size: 0.95rem;
  color: #374151;
}
</style>