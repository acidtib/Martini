import { createWebHistory, createRouter } from 'vue-router'

import AppLayout from '../layouts/AppLayout.vue'
import BlankLayout from '../layouts/BlankLayout.vue'

const routes = [
  { 
    path: '/', 
    name: 'home', 
    meta: { layout: AppLayout },
    component: () => import('../views/HomeView.vue')
  },
  { 
    path: '/settings', 
    name: 'settings', 
    meta: { layout: AppLayout },
    component: () => import('../views/SettingsView.vue')
  },
  { 
    path: '/screenshot', 
    name: 'screenshot', 
    meta: { layout: BlankLayout },
    component: () => import('../views/ScreenshotView.vue')
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router