import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'

// Import shared CSS system
import './assets/css/main.css'

// Import views
import HomeView from './views/HomeView.vue'
import DashboardView from './views/DashboardView.vue'
import FactoriesView from './views/FactoriesView.vue'
import LogisticsView from './views/LogisticsView.vue'

const routes = [
  { path: '/', component: HomeView },
  { path: '/dashboard', component: DashboardView },
  { path: '/factories', component: FactoriesView },
  { path: '/logistics', component: LogisticsView },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

const pinia = createPinia()

createApp(App)
  .use(pinia)
  .use(router)
  .mount('#app')