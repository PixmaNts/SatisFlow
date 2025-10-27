import { createRouter, createWebHistory } from 'vue-router'
import DashboardView from '@/views/DashboardView.vue'
import FactoryView from '@/views/FactoryView.vue'
import LogisticsView from '@/views/LogisticsView.vue'
import BlueprintLibraryView from '@/views/BlueprintLibraryView.vue'
import ValidationDemo from '@/components/forms/ValidationDemo.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: DashboardView,
    },
    {
      path: '/factory',
      name: 'factory',
      component: FactoryView,
    },
    {
      path: '/logistics',
      name: 'logistics',
      component: LogisticsView,
    },
    {
      path: '/validation-demo',
      name: 'validation-demo',
      component: ValidationDemo,
    },
    {
      path: '/blueprints',
      name: 'blueprints',
      component: BlueprintLibraryView,
      meta: {
        title: 'Blueprint Library',
      },
    },
  ],
})

export default router
