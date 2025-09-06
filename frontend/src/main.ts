import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'

// Import views
import HomeView from './views/HomeView.vue'
import FactoriesView from './views/FactoriesView.vue'
import LogisticsView from './views/LogisticsView.vue'

const routes = [
  { path: '/', component: HomeView },
  { path: '/factories', component: FactoriesView },
  { path: '/logistics', component: LogisticsView },
  // Redirect old dashboard route to new factories route
  { path: '/dashboard', redirect: '/factories' },
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