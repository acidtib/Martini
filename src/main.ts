import { createApp } from 'vue'
import router from './lib/router'
import App from './App.vue'
import { listen } from '@tauri-apps/api/event'
import { Window } from '@tauri-apps/api/window'
import { Webview, getAllWebviews } from '@tauri-apps/api/webview'
import { checkForUpdates } from './lib/updater';

// Check for updates when app starts
checkForUpdates().catch(console.error);

// Create and mount the Vue app
createApp(App).use(router).mount('#app')

// Listen for the open-viewer event
listen('open-viewer', async (event) => {
    console.log(event.payload);

    try {
        // Try to get existing window first
        let viewerWindow = await Window.getByLabel('screenshot-viewer');
        
        if (!viewerWindow) {
            // Create new window if it doesn't exist
            viewerWindow = new Window('screenshot-viewer', {
                title: 'Martini - Screenshot Viewer',
                width: 800,
                height: 600,
                resizable: true,
                center: true
            });
        }

        // Check for existing viewer webview and close it
        const existingWebviews = await getAllWebviews();
        let viewerWebview = existingWebviews.find(w => w.label === 'screenshot-viewer-view');

        console.log('Existing webviews:', viewerWebview);
        
        if (!viewerWebview) {
            // Create webview and set up event listeners
            viewerWebview = new Webview(viewerWindow, 'screenshot-viewer-view', {
                url: '/screenshot',
                x: 0,
                y: 0,
                width: 800,
                height: 600
            });
        }

        viewerWebview.once('tauri://created', () => {
            console.log('Viewer webview created successfully');
            viewerWebview.setFocus();
        });

        viewerWebview.once('tauri://error', (e) => {
            console.error('Error creating viewer webview:', e);
        });

        await viewerWindow.setVisibleOnAllWorkspaces(true);
        await viewerWindow.setFocus();
        
        console.log('Viewer window ready');
    } catch (error) {
        console.error('Failed to create or show viewer window:', error);
    }
});
