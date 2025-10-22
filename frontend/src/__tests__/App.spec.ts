import { describe, it, expect, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'
import App from '../App.vue'

// Mock router
const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', name: 'Dashboard', component: { template: '<div>Dashboard</div>' } },
    { path: '/factory', name: 'Factory', component: { template: '<div>Factory</div>' } },
    { path: '/logistics', name: 'Logistics', component: { template: '<div>Logistics</div>' } },
  ],
})

describe('App', () => {
  beforeEach(() => {
    // Create and set active Pinia instance for each test
    const pinia = createPinia()
    setActivePinia(pinia)
  })

  it('mounts renders properly', async () => {
    const wrapper = mount(App, {
      global: {
        plugins: [createPinia(), router],
      },
    })

    // Wait for router to be ready
    await router.isReady()

    expect(wrapper.text()).toContain('Satisflow')
  })
})
