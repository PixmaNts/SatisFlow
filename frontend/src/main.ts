import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

// Import global CSS variables and typography
import './assets/styles/variables.css'
import './assets/styles/typography.css'
import './assets/styles/industrial-overrides.css'

// Apply industrial dark theme immediately, before app mounts
if (typeof document !== 'undefined') {
  const root = document.documentElement
  root.classList.remove('light-theme', 'dark-theme')
  root.classList.add('dark-theme')
  root.setAttribute('data-theme', 'dark')
}

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')
