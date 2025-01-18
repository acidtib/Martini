import { listen } from '@tauri-apps/api/event'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
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
        let viewerWindow = await WebviewWindow.getByLabel('screenshot-viewer');
            
        if (viewerWindow) {
            // Close existing window
            await viewerWindow.close();
        }

        viewerWindow = new WebviewWindow('screenshot-viewer', {
            title: 'Screenshot Viewer - Martini',
            url: '/screenshot',
            width: 920,
            height: 580,
            minWidth: 920,
            minHeight: 580,
            resizable: true,
            center: true
          });

        viewerWindow.once('tauri://created', async () => {
            console.log('Viewer window created successfully');
            await viewerWindow.setFocus();
        });

        viewerWindow.once('tauri://error', (e) => {
            console.error('Error creating viewerWindow:', e);
        });
        
        console.log('Viewer window ready');
    });
}
