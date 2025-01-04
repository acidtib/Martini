import { createApp } from "vue";
import App from "./App.vue";
import { checkForUpdates } from './lib/updater';

// Check for updates when app starts
checkForUpdates().catch(console.error);

createApp(App).mount("#app");
