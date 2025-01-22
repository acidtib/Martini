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
    }, 1000);

    // save screenshot
    listen('save-screenshot', async (event: ScreenshotEvent) => {
        console.log('Saving screenshot...');

        const { image, name = 'screenshot.jpg' } = event.payload
        const screenshot = new Screenshots({ name, image })
        await screenshot.save()
    });

    // Open screenshot viewer
    listen('open-screenshot-viewer', handleOpenScreenshotViewer)

    // Close screenshot viewer
    listen('close-screenshot-viewer', async () => {
        const viewerWindow = await WebviewWindow.getByLabel('screenshot-viewer');
        if (viewerWindow) {
            viewerWindow.close();
        }
    })

}
