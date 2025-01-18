import { listen } from '@tauri-apps/api/event'
import { Window } from '@tauri-apps/api/window'
import { Webview, getAllWebviews } from '@tauri-apps/api/webview'
import Database from '@tauri-apps/plugin-sql'

// Define the type for the screenshot payload
interface ScreenshotPayload {
    image: string;
    name?: string;
}

interface ScreenshotEvent {
    payload: ScreenshotPayload;
}

export const initializeEventListeners = () => {
    // when shortcut is pressed and screenshot is taken
    listen('new-screenshot', async (event: ScreenshotEvent) => {
        console.log(event);

        try {
            // Connect to the database
            const db = await Database.load('sqlite:app.db');
            
            // Save screenshot to database
            const { payload } = event;
            await db.execute(
                'INSERT INTO screenshots (image, name) VALUES ($1, $2)',
                [payload.image, payload.name || 'screenshot.png']
            );

            console.log('Screenshot saved to database');
        } catch (error) {
            console.error('Error saving screenshot:', error);
        }

        // Try to get existing window first
        let viewerWindow = await Window.getByLabel('screenshot-viewer');
            
        if (viewerWindow) {
            // Close existing window
            await viewerWindow.close();
        }

        viewerWindow = new Window('screenshot-viewer', {
            title: 'Screenshot Viewer - Martini',
            width: 800,
            height: 600,
            resizable: true,
            center: true
        });

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
    });
}
