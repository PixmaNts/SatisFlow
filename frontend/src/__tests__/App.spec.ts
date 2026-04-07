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

  it('renders navigation and main content area', async () => {
    const wrapper = mount(App, {
      global: {
        plugins: [createPinia(), router],
      },
    })

    // Wait for router to be ready
    await router.isReady()

    // Verify navigation bar exists with brand title
    const nav = wrapper.find('nav.main-nav')
    expect(nav.exists()).toBe(true)

    // Verify brand title renders
    const title = wrapper.find('h1.nav-title')
    expect(title.exists()).toBe(true)
    expect(title.text()).toBe('Satisflow')

    // Verify navigation links exist
    const navLinks = wrapper.findAll('router-link-stub, a.nav-link')
    expect(navLinks.length).toBeGreaterThanOrEqual(3)

    // Verify main content area with router-view exists
    const mainContent = wrapper.find('main.main-content')
    expect(mainContent.exists()).toBe(true)
  })
})
