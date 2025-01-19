import { createApp } from 'vue'
import { checkForUpdates } from './lib/updater';
import { updateSystemInfo } from './lib/system_info'
import { initializeEventListeners } from './lib/events'
import router from './lib/router'
import App from './App.vue'

// Check for updates when app starts
checkForUpdates().catch(console.error);

// Update system info when app starts
updateSystemInfo().catch(console.error);

// Initialize event listeners
initializeEventListeners();

// Create and mount the Vue app
createApp(App).use(router).mount('#app')