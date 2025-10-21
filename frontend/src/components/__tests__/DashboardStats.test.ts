import { describe, it, expect, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import DashboardStats from '../DashboardStats.vue'
import type { DashboardSummary } from '@/api/types'

describe('DashboardStats.vue', () => {
  let wrapper: ReturnType<typeof mount>
  let pinia: ReturnType<typeof createPinia>

  beforeEach(() => {
    pinia = createPinia()
    setActivePinia(pinia)
  })

  it('renders correctly with data', () => {
    const summary: DashboardSummary = {
      total_factories: 5,
      total_production_lines: 12,
      total_logistics_lines: 8,
      total_power_consumption: 250.5,
      total_power_generation: 300.0,
      net_power: 49.5,
    }

    wrapper = mount(DashboardStats, {
      props: { summary },
      global: {
        plugins: [pinia],
      },
    })

    expect(wrapper.find('.dashboard-stats').exists()).toBe(true)
    expect(wrapper.text()).toContain('5')
    expect(wrapper.text()).toContain('12')
    expect(wrapper.text()).toContain('8')
    expect(wrapper.text()).toContain('250.5')
    expect(wrapper.text()).toContain('300.0')
    expect(wrapper.text()).toContain('49.5')
  })

  it('renders loading state', () => {
    wrapper = mount(DashboardStats, {
      props: { loading: true },
      global: {
        plugins: [pinia],
      },
    })

    expect(wrapper.find('.loading-indicator').exists()).toBe(true)
    expect(wrapper.text()).toContain('Loading')
  })

  it('renders error state', () => {
    wrapper = mount(DashboardStats, {
      props: { error: 'Failed to load data' },
      global: {
        plugins: [pinia],
      },
    })

    expect(wrapper.find('.error-message').exists()).toBe(true)
    expect(wrapper.text()).toContain('Failed to load data')
  })

  it('formats numbers correctly', () => {
    const summary: DashboardSummary = {
      total_factories: 1234,
      total_production_lines: 5678,
      total_logistics_lines: 9012,
      total_power_consumption: 1234.56,
      total_power_generation: 5678.90,
      net_power: 1234.56,
    }

    wrapper = mount(DashboardStats, {
      props: { summary },
      global: {
        plugins: [pinia],
      },
    })

    // Check if numbers are formatted with commas
    expect(wrapper.text()).toContain('1,234')
    expect(wrapper.text()).toContain('5,678')
    expect(wrapper.text()).toContain('9,012')

    // Check if decimals are formatted correctly
    expect(wrapper.text()).toContain('1,234.6')
    expect(wrapper.text()).toContain('5,678.9')
    expect(wrapper.text()).toContain('1,234.6')
  })

  it('emits refresh event when refresh button is clicked', async () => {
    const summary: DashboardSummary = {
      total_factories: 5,
      total_production_lines: 12,
      total_logistics_lines: 8,
      total_power_consumption: 250.5,
      total_power_generation: 300.0,
      net_power: 49.5,
    }

    wrapper = mount(DashboardStats, {
      props: { summary },
      global: {
        plugins: [pinia],
      },
    })

    const refreshButton = wrapper.find('.refresh-button')
    expect(refreshButton.exists()).toBe(true)

    await refreshButton.trigger('click')
    expect(wrapper.emitted().refresh).toBeTruthy()
    expect(wrapper.emitted().refresh).toHaveLength(1)
  })

  it('applies correct CSS classes based on power balance', () => {
    const positiveSummary: DashboardSummary = {
      total_factories: 5,
      total_production_lines: 12,
      total_logistics_lines: 8,
      total_power_consumption: 250.5,
      total_power_generation: 300.0,
      net_power: 49.5,
    }

    const negativeSummary: DashboardSummary = {
      total_factories: 5,
      total_production_lines: 12,
      total_logistics_lines: 8,
      total_power_consumption: 300.0,
      total_power_generation: 250.5,
      net_power: -49.5,
    }

    // Test positive power balance
    wrapper = mount(DashboardStats, {
      props: { summary: positiveSummary },
      global: {
        plugins: [pinia],
      },
    })

    const netPowerElement = wrapper.find('.net-power')
    expect(netPowerElement.exists()).toBe(true)
    expect(netPowerElement.classes()).toContain('positive')

    // Test negative power balance
    wrapper = mount(DashboardStats, {
      props: { summary: negativeSummary },
      global: {
        plugins: [pinia],
      },
    })

    const netPowerElement2 = wrapper.find('.net-power')
    expect(netPowerElement2.exists()).toBe(true)
    expect(netPowerElement2.classes()).toContain('negative')
  })
})
