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

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

// Debounce function to prevent multiple rapid executions
function debounce<T extends (...args: any[]) => any>(
    func: T,
    wait: number
): (...args: Parameters<T>) => void {
    let timeout: NodeJS.Timeout | null = null;
    
    return (...args: Parameters<T>) => {
        if (timeout) {
            clearTimeout(timeout);
        }
        
        timeout = setTimeout(() => {
            func(...args);
            timeout = null;
        }, wait);
    };
}

export const initializeEventListeners = () => {
    const handleOpenScreenshotViewer = debounce(async () => {
        // Try to get existing window first
        let viewerWindow = await WebviewWindow.getByLabel('screenshot-viewer');
                
        if (viewerWindow) {
            // Destroy existing window
            await viewerWindow.destroy();
            // Wait for window to be fully destroyed
            await sleep(500);
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
    }, 1000);

    // Create a debounced version of the screenshot handler
    const handleScreenshot = debounce(async (event: ScreenshotEvent) => {
        console.log(event);

        try {
            const { image, name = 'screenshot.jpg' } = event.payload
            const screenshot = new Screenshots({ name, image })
            await screenshot.save()

            console.log('Screenshot saved to database');

            // Try to get existing window first
            let viewerWindow = await WebviewWindow.getByLabel('screenshot-viewer');
                
            if (viewerWindow) {
                // Destroy existing window
                await viewerWindow.destroy();
                // Wait for window to be fully destroyed
                await sleep(500);
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
        } catch (error) {
            console.error('Error saving screenshot:', error);
        }
    }, 1000); // 1000ms debounce time


    // Open screenshot viewer
    listen('open-screenshot-viewer', handleOpenScreenshotViewer)

    // Handle new screenshot
    listen('new-screenshot', handleScreenshot);
}
