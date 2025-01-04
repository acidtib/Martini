import { check } from '@tauri-apps/plugin-updater'

export async function checkForUpdates() {
  try {
    const update = await check()
    
    if (update) {
      console.log(
        `Found update ${update.version} from ${update.date} with notes: ${update.body}`
      )
    } else {
      console.log('No updates available')
    }
  } catch (error) {
    console.error('Error checking for updates:', error)
  }
}
