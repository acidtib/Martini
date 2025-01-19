import {
  memoryInfo,
  staticInfo,
  cpuInfo
} from "tauri-plugin-system-info-api";
import { Settings } from './database'

function formatMemorySize(bytes: number): string {
  const gigabytes = bytes / (1024 * 1024 * 1024)
  return `${Math.round(gigabytes * 100) / 100} GB`
}

export async function updateSystemInfo() {
  try {
    const { total_memory } = await memoryInfo();
    const { name, os_version } = await staticInfo();
    const { cpu_count } = await cpuInfo();

    const systemInfo = {
      system_os: `${name} ${os_version}`,
      system_cpu: `${cpu_count} Cores`,
      system_memory: formatMemorySize(total_memory)
    };

    // Update each system info setting
    for (const [key, value] of Object.entries(systemInfo)) {
      // Create or update setting
      const setting = new Settings({ key, value });
      await setting.save();
    }

    console.log('System information updated successfully');
  } catch (error) {
    console.error('Failed to update system information:', error);
  }
}
