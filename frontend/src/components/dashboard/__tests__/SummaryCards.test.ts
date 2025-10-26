import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import SummaryCards from '../SummaryCards.vue'
import type { DashboardSummary, PowerStats } from '@/api/types'

describe('SummaryCards.vue', () => {
  const createSummary = (overrides: Partial<DashboardSummary> = {}): DashboardSummary => ({
    total_factories: 5,
    total_production_lines: 12,
    total_logistics_lines: 8,
    total_power_consumption: 250.5,
    total_power_generation: 300.0,
    net_power: 49.5,
    ...overrides,
  })

  const createPowerStats = (overrides: Partial<PowerStats> = {}): PowerStats => ({
    total_generation: 300.0,
    total_consumption: 250.5,
    power_balance: 49.5,
    has_surplus: true,
    has_deficit: false,
    is_balanced: false,
    factory_stats: [],
    ...overrides,
  })

  const mountComponent = (options: {
    summary?: DashboardSummary | null
    powerStats?: PowerStats | null
    loading?: boolean
  } = {}) => {
    const {
      summary = createSummary(),
      powerStats = createPowerStats(),
      loading = false,
    } = options

    return mount(SummaryCards, {
      props: { summary, powerStats, loading },
      global: {
        stubs: {
          LoadingSpinner: {
            name: 'LoadingSpinner',
            template: '<div class="loading-spinner-stub" />',
          },
        },
      },
    })
  }

  it('renders core summary metrics', () => {
    const wrapper = mountComponent()

    expect(wrapper.find('.summary-cards').exists()).toBe(true)
    expect(wrapper.text()).toContain('5')
    expect(wrapper.text()).toContain('12')
    expect(wrapper.text()).toContain('8')
  })

  it('displays loading indicators when loading', () => {
    const wrapper = mountComponent({ loading: true })

    const placeholders = wrapper.findAll('.loading-spinner-stub')
    expect(placeholders.length).toBeGreaterThan(0)
  })

  it('applies surplus and deficit styling based on power stats', () => {
    const surplusWrapper = mountComponent({
      powerStats: createPowerStats({ has_surplus: true, has_deficit: false, is_balanced: false, power_balance: 25 }),
    })
    expect(surplusWrapper.find('.summary-card.surplus').exists()).toBe(true)

    const deficitWrapper = mountComponent({
      powerStats: createPowerStats({ has_surplus: false, has_deficit: true, is_balanced: false, power_balance: -20 }),
    })
    expect(deficitWrapper.find('.summary-card.deficit').exists()).toBe(true)

    const balancedWrapper = mountComponent({
      powerStats: createPowerStats({ has_surplus: false, has_deficit: false, is_balanced: true, power_balance: 0 }),
    })
    expect(balancedWrapper.find('.summary-card.balanced').exists()).toBe(true)
  })

  it('formats large power values as gigawatts', () => {
    const wrapper = mountComponent({
      summary: createSummary({ total_power_generation: 5000 }),
      powerStats: createPowerStats({ total_generation: 5000 }),
    })

    expect(wrapper.text()).toContain('5.0 GW')
  })
})
