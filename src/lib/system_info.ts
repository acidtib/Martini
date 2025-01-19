import {
  memoryInfo,
  staticInfo,
  cpuInfo
} from "tauri-plugin-system-info-api";

export async function updateSystemInfo() {
  const { total_memory } = await memoryInfo();
  const { name, os_version } = await staticInfo();
  const { cpu_count } = await cpuInfo();

  console.log("os_name", name + " " + os_version);
  console.log("cpu_count", cpu_count);
  console.log("total_memory", total_memory);
}
