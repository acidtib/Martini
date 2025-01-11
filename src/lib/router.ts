import { createWebHistory, createRouter } from 'vue-router'

import HomeView from '../views/HomeView.vue'
import SettingsView from '../views/SettingsView.vue'
import ScreenshotView from '../views/ScreenshotView.vue'

const routes = [
  { path: '/', name: 'home', component: HomeView },
  { path: '/settings', name: 'settings', component: SettingsView },
  { path: '/screenshot', name: 'screenshot', component: ScreenshotView },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router