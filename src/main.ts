import { createApp } from 'vue'
import App from './App.vue'
import { listen } from '@tauri-apps/api/event'
import { Window } from '@tauri-apps/api/window'
import { checkForUpdates } from './lib/updater';

// Check for updates when app starts
checkForUpdates().catch(console.error);

// Create and mount the Vue app
createApp(App).mount('#app')

// Listen for the open-viewer event
listen('open-viewer', () => {
    const appWindow = new Window('screenshot-viewer', {
        url: 'viewer.html',
        title: 'Screenshot Viewer',
        center: true
    });

    appWindow.once('tauri://created', () => {
        console.log('Viewer window created successfully');
    });

    appWindow.once('tauri://error', (e) => {
        console.error('Error creating viewer window:', e);
    });
});
