import { mount, VueWrapper } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import type { ComponentPublicInstance } from 'vue'
import { expect } from 'vitest'

/**
 * Mount a component with Pinia store
 */
export function mountWithStore(component: any, options: any = {}) {
  const pinia = createPinia()
  setActivePinia(pinia)

  return mount(component, {
    global: {
      plugins: [pinia],
      ...options.global,
    },
    ...options,
  })
}

/**
 * Create a mock factory
 */
export function createMockFactory(overrides: any = {}) {
  return {
    id: 1,
    name: 'Test Factory',
    description: 'Test Description',
    notes: null,
    production_lines: [],
    raw_inputs: [],
    power_generators: [],
    items: [],
    total_power_consumption: 100,
    total_power_generation: 150,
    power_balance: 50,
    ...overrides,
  }
}

/**
 * Create a mock production line
 */
export function createMockProductionLine(overrides: any = {}) {
  return {
    ProductionLineRecipe: {
      id: 1,
      name: 'Iron Plate Production',
      description: 'Produces iron plates',
      recipe: 'IronPlate',
      machine_groups: [
        {
          number_of_machine: 2,
          oc_value: 100,
          somersloop: 0,
        },
      ],
    },
    ...overrides,
  }
}

/**
 * Create a mock logistics line
 */
export function createMockLogistics(overrides: any = {}) {
  return {
    id: 1,
    from_factory: 1,
    to_factory: 2,
    transport_type: 'Truck',
    transport_id: 'TRUCK-001',
    transport_name: 'Test Transport',
    transport_details: 'Test logistics line',
    items: [
      {
        item: 'IronPlate',
        quantity_per_min: 60,
      },
    ],
    total_quantity_per_min: 60,
    ...overrides,
  }
}

/**
 * Create a mock item balance
 */
export function createMockItemBalance(overrides: any = {}) {
  return {
    item: 'IronPlate',
    balance: 30,
    state: 'overflow' as const,
    ...overrides,
  }
}

/**
 * Create a mock dashboard summary
 */
export function createMockDashboardSummary(overrides: any = {}) {
  return {
    total_factories: 2,
    total_production_lines: 5,
    total_logistics_lines: 3,
    total_power_consumption: 150.5,
    total_power_generation: 200.0,
    net_power: 49.5,
    ...overrides,
  }
}

/**
 * Wait for next tick
 */
export function nextTick() {
  return new Promise(resolve => setTimeout(resolve, 0))
}

/**
 * Flush all pending promises
 */
export async function flushPromises() {
  await nextTick()
  await nextTick()
}

/**
 * Type assertion helper
 */
export function expectType<T>(value: T): T {
  return value
}

/**
 * Mock API response helper
 */
export function createApiResponse<T>(data: T, status = 200) {
  return Promise.resolve({
    ok: status >= 200 && status < 300,
    status,
    json: () => Promise.resolve(data),
    text: () => Promise.resolve(JSON.stringify(data)),
  } as Response)
}

/**
 * Mock API error helper
 */
export function createApiError(message: string, status = 400) {
  return Promise.resolve({
    ok: false,
    status,
    json: () => Promise.resolve({ error: message }),
    text: () => Promise.resolve(JSON.stringify({ error: message })),
  } as Response)
}

/**
 * Common test data factory
 */
export const testData = {
  factories: [createMockFactory(), createMockFactory({ id: 2, name: 'Second Factory' })],
  logistics: [createMockLogistics()],
  itemBalances: [
    createMockItemBalance(),
    createMockItemBalance({ item: 'Wire', balance: -20, state: 'underflow' as const }),
  ],
  dashboardSummary: createMockDashboardSummary(),
}

/**
 * Helper to test reactive properties
 */
export async function testReactiveProperty<T>(
  wrapper: VueWrapper<ComponentPublicInstance>,
  property: string,
  initialValue: T,
  newValue: T,
  selector?: string
) {
  // Test initial value
  if (selector) {
    expect(wrapper.find(selector).exists()).toBe(true)
    expect(wrapper.find(selector).text()).toContain(String(initialValue))
  } else {
    expect((wrapper.vm as any)[property]).toBe(initialValue)
  }

  // Set new value
  await wrapper.setData({ [property]: newValue })
  await flushPromises()

  // Test new value
  if (selector) {
    expect(wrapper.find(selector).text()).toContain(String(newValue))
  } else {
    expect((wrapper.vm as any)[property]).toBe(newValue)
  }
}

/**
 * Helper to test form validation
 */
export async function testFormValidation(
  wrapper: VueWrapper<ComponentPublicInstance>,
  fieldSelector: string,
  errorSelector: string,
  invalidValue: any,
  validValue: any,
  expectedError: string
) {
  const field = wrapper.find(fieldSelector)
  expect(field.exists()).toBe(true)

  // Set invalid value
  await field.setValue(invalidValue)
  await field.trigger('blur')
  await flushPromises()

  // Check for error
  const errorElement = wrapper.find(errorSelector)
  expect(errorElement.exists()).toBe(true)
  expect(errorElement.text()).toContain(expectedError)

  // Set valid value
  await field.setValue(validValue)
  await field.trigger('blur')
  await flushPromises()

  // Check error is gone
  expect(wrapper.find(errorSelector).exists()).toBe(false)
}
