<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { Settings } from '../lib/database'

interface SettingData {
  key: string
  value: string
}

const settings = ref<SettingData[]>([])
const loading = ref(true)
const error = ref<string | null>(null)

// System settings that should be read-only
const SYSTEM_SETTINGS = ['bootstrapped', 'installed_on', 'system_cpu', 'system_memory', 'system_os']

const formatSettingName = (key: string): string => {
  return key
    .split('_')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

const formatSettingValue = (key: string, value: string): string => {
  if (key === 'bootstrapped') {
    return value === 'true' ? 'Yes' : 'No'
  }
  if (key === 'installed_on') {
    try {
      return new Date(value).toLocaleString()
    } catch {
      return value
    }
  }
  return value === '-' ? 'Not Available' : value
}

const systemSettings = computed(() => {
  return settings.value.filter(setting => SYSTEM_SETTINGS.includes(setting.key))
})

const loadSettings = async () => {
  try {
    loading.value = true
    error.value = null
    const results = await Settings.findAll()
    settings.value = results.map(setting => ({
      key: setting.getAttributes().key,
      value: setting.getAttributes().value
    }))
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to load settings'
    console.error('Error loading settings:', err)
  } finally {
    loading.value = false
  }
}

const updateSetting = async (key: string, value: string) => {
  if (SYSTEM_SETTINGS.includes(key)) return // Prevent updating system settings
  
  try {
    // Find existing setting
    const results = await Settings.findAll({ where: { key }, limit: 1 })
    const setting = results[0]
    
    if (setting) {
      // Update existing setting
      setting.getAttributes().value = value
      await setting.save()
    } else {
      // Create new setting
      const newSetting = new Settings({ key, value })
      await newSetting.save()
    }
    
    await loadSettings() // Reload to get updated data
  } catch (err) {
    console.error('Error updating setting:', err)
  }
}

onMounted(() => {
  loadSettings()
})
</script>

<template>
  <div class="settings-view">
    <h2 class="text-2xl font-bold mb-6">Settings</h2>

    <div v-if="loading" class="text-gray-600">
      Loading settings...
    </div>

    <div v-else-if="error" class="text-red-600">
      {{ error }}
    </div>

    <div v-else>
      <!-- System Information -->
      <div class="mb-8">
        <h3 class="text-xl font-semibold mb-4">System Information</h3>
        <div class="settings-grid">
          <div v-for="setting in systemSettings" :key="setting.key" class="setting-item">
            <div class="setting-content">
              <div class="setting-label">{{ formatSettingName(setting.key) }}</div>
              <div class="setting-value" :class="{ 'text-gray-500': setting.value === '-' }">
                {{ formatSettingValue(setting.key, setting.value) }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Other Settings -->
      <div v-if="settings.length > systemSettings.length">
        <h3 class="text-xl font-semibold mb-4">Other Settings</h3>
        <div class="settings-grid">
          <div 
            v-for="setting in settings.filter(s => !SYSTEM_SETTINGS.includes(s.key))" 
            :key="setting.key" 
            class="setting-item"
          >
            <div class="setting-content">
              <div class="setting-label">{{ formatSettingName(setting.key) }}</div>
              <div class="setting-value">
                <input
                  type="text"
                  :value="setting.value"
                  @change="e => updateSetting(setting.key, (e.target as HTMLInputElement).value)"
                  class="bg-transparent border-b border-gray-300 focus:border-blue-500 outline-none px-2 py-1 w-full"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-view {
  padding: 1.5rem;
}

.settings-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

.setting-item {
  background-color: rgba(255, 255, 255, 0.05);
  border-radius: 0.5rem;
  padding: 1rem;
}

.setting-content {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.setting-label {
  font-weight: 500;
  color: #9ca3af;
}

.setting-value {
  font-size: 1.125rem;
}
</style>