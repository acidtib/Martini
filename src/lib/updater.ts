import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'

export async function checkForUpdates() {
  try {
    const update = await check()
    
    if (update) {
      console.log(
        `Found update ${update.version} from ${update.date} with notes: ${update.body}`
      )
      
      let downloaded = 0
      let contentLength = 0
      
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            contentLength = event.data.contentLength || 0
            console.log(`Started downloading ${event.data.contentLength} bytes`)
            break
          case 'Progress':
            downloaded += event.data.chunkLength
            const progress = contentLength > 0 ? (downloaded / contentLength) * 100 : 0
            console.log(`Downloaded ${progress.toFixed(2)}%`)
            break
          case 'Finished':
            console.log('Download finished')
            break
        }
      })

      console.log('Update installed, relaunching...')
      await relaunch()
    } else {
      console.log('No updates available')
    }
  } catch (error) {
    console.error('Error checking for updates:', error)
  }
}
