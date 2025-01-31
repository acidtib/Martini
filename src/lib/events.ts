import { listen } from '@tauri-apps/api/event'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { Screenshots } from './database'

// Define the type for the screenshot payload
interface ScreenshotPayload {
    image: string;
    name?: string;
}

interface ScreenshotEvent {
    payload: ScreenshotPayload;
}

export const initializeEventListeners = () => {
    // open screenshot viewer
    const handleOpenScreenshotViewer = async () => {
        // Try to get existing window first
        let viewerWindow: WebviewWindow | null = await WebviewWindow.getByLabel('screenshot-viewer');
                
        if (viewerWindow) {
            // Emit an event to the existing window to trigger a refresh
            await viewerWindow.emit('refresh-screenshot-viewer');
            await (viewerWindow as WebviewWindow).setFocus();
        } else {
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

            // After creating the window, we know it's not null
            (viewerWindow).once('tauri://created', async () => {
                console.log('Viewer window created successfully');
                await (viewerWindow as WebviewWindow).setFocus();
            });
    
            (viewerWindow).once('tauri://error', (e) => {
                console.error('Error creating viewerWindow:', e);
            });
        }
    };

    // save screenshot
    const handleSaveScreenshot = async (event: ScreenshotEvent) => {
        console.log('Saving screenshot...');

        const { image, name = 'screenshot.jpg' } = event.payload
        const screenshot = new Screenshots({ name, image })
        await screenshot.save()
    }

    // close screenshot viewer
    const handleCloseScreenshotViewer = async () => {
        const viewerWindow = await WebviewWindow.getByLabel('screenshot-viewer');
        if (viewerWindow) {
            viewerWindow.close();
        }
    }

    
    listen('save-screenshot', handleSaveScreenshot);
    listen('open-screenshot-viewer', handleOpenScreenshotViewer)
    listen('close-screenshot-viewer', handleCloseScreenshotViewer)
}
