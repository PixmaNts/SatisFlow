import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import LogisticsLineForm from '../LogisticsLineForm.vue'
import { useLogisticsStore } from '@/stores/logistics'
import { useFactoryStore } from '@/stores/factory'
import type { LogisticsResponse, FactoryResponse } from '@/api/types'

// Mock the API endpoints
vi.mock('@/api/endpoints', () => ({
  logistics: {
    create: vi.fn().mockResolvedValue({}),
    update: vi.fn().mockResolvedValue({}),
  },
  factories: {
    getAll: vi.fn().mockResolvedValue([]),
  },
}))

// Mock child components
vi.mock('../TransportSelector.vue', () => ({
  default: { name: 'TransportSelector', template: '<div class="transport-selector"></div>' }
}))
vi.mock('../BusEditor.vue', () => ({
  default: { name: 'BusEditor', template: '<div class="bus-editor"></div>' }
}))
vi.mock('../TrainEditor.vue', () => ({
  default: { name: 'TrainEditor', template: '<div class="train-editor"></div>' }
}))
vi.mock('../TruckEditor.vue', () => ({
  default: { name: 'TruckEditor', template: '<div class="truck-editor"></div>' }
}))
vi.mock('../DroneEditor.vue', () => ({
  default: { name: 'DroneEditor', template: '<div class="drone-editor"></div>' }
}))

describe('LogisticsLineForm', () => {
  let pinia: ReturnType<typeof createPinia>

  const mockFactories: FactoryResponse[] = [
    {
      id: 'factory-1',
      name: 'Factory 1',
      description: null,
      notes: null,
      production_lines: [],
      raw_inputs: [],
      power_generators: [],
      items: [],
      total_power_consumption: 0,
      total_power_generation: 0,
      power_balance: 0,
    },
    {
      id: 'factory-2',
      name: 'Factory 2',
      description: null,
      notes: null,
      production_lines: [],
      raw_inputs: [],
      power_generators: [],
      items: [],
      total_power_consumption: 0,
      total_power_generation: 0,
      power_balance: 0,
    },
  ]

  const mockLogisticsLine: LogisticsResponse = {
    id: 'logistics-1',
    from_factory: 'factory-1',
    to_factory: 'factory-2',
    transport_type: 'Truck',
    transport_id: 'TRUCK-001',
    transport_name: 'Test Transport',
    transport_details: JSON.stringify({ fuel_type: 'Coal', round_trip_time: 120 }),
    items: [{ item: 'IronPlate', quantity_per_min: 60 }],
    total_quantity_per_min: 60,
  }

  beforeEach(async () => {
    pinia = createPinia()
    setActivePinia(pinia)

    // Populate factories in the store so the form can use them
    const factoryStore = useFactoryStore()
    factoryStore.factories = mockFactories

    vi.clearAllMocks()
  })

  describe('Mode detection', () => {
    it('should be in create mode when no logisticsLine prop is provided', async () => {
      const wrapper = mount(LogisticsLineForm, {
        props: { show: true },
        global: { plugins: [pinia] },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.isEditing).toBe(false)
    })

    it('should be in edit mode when logisticsLine prop is provided', async () => {
      const wrapper = mount(LogisticsLineForm, {
        props: { show: true, logisticsLine: mockLogisticsLine },
        global: { plugins: [pinia] },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.isEditing).toBe(true)
    })
  })

  describe('Edit mode initialization', () => {
    it('should load logistics line data into form', async () => {
      const wrapper = mount(LogisticsLineForm, {
        props: { show: true, logisticsLine: mockLogisticsLine },
        global: { plugins: [pinia] },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.formData.from_factory).toBe('factory-1')
      expect(vm.formData.to_factory).toBe('factory-2')
    })

    it('should parse transport_details JSON into transportConfig', async () => {
      const wrapper = mount(LogisticsLineForm, {
        props: { show: true, logisticsLine: mockLogisticsLine },
        global: { plugins: [pinia] },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.selectedTransportType).toBe('Truck')
      expect(vm.transportConfig).toEqual({ fuel_type: 'Coal', round_trip_time: 120 })
    })

    it('should reset form when show becomes false', async () => {
      const wrapper = mount(LogisticsLineForm, {
        props: { show: true, logisticsLine: mockLogisticsLine },
        global: { plugins: [pinia] },
      })

      await wrapper.vm.$nextTick()

      let vm = wrapper.vm as any
      expect(vm.formData.from_factory).toBe('factory-1')

      // Close the form
      await wrapper.setProps({ show: false })
      await wrapper.vm.$nextTick()

      vm = wrapper.vm as any
      expect(vm.formData.from_factory).toBe('')
      expect(vm.formData.to_factory).toBe('')
    })
  })

  describe('Save behavior — the critical edit vs create branch', () => {
    it('should call logisticsStore.update (NOT create) when editing', async () => {
      const logisticsStore = useLogisticsStore()
      const updateSpy = vi.spyOn(logisticsStore, 'update').mockResolvedValue(mockLogisticsLine)
      const createSpy = vi.spyOn(logisticsStore, 'create').mockResolvedValue(mockLogisticsLine)

      const wrapper = mount(LogisticsLineForm, {
        props: { show: true, logisticsLine: mockLogisticsLine },
        global: { plugins: [pinia] },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any

      // Simulate what loadLogisticsLine would set: transportConfig + formData.transport_config
      vm.selectedTransportType = 'Truck'
      vm.transportConfig = {
        transport_type: 'Truck',
        item: 'IronPlate',
        quantity_per_min: 60,
        truck_id: 'TRUCK-001',
      }
      vm.formData.transport_config = {
        transport_type: 'Truck',
        item: 'IronPlate',
        quantity_per_min: 60,
        truck_id: 'TRUCK-001',
      }
      await wrapper.vm.$nextTick()

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      // CRITICAL: update must be called, create must NOT be called
      expect(updateSpy).toHaveBeenCalledWith('logistics-1', expect.any(Object))
      expect(createSpy).not.toHaveBeenCalled()
    })

    it('should call logisticsStore.create (NOT update) when creating new', async () => {
      const logisticsStore = useLogisticsStore()
      const updateSpy = vi.spyOn(logisticsStore, 'update').mockResolvedValue(mockLogisticsLine)
      const createSpy = vi.spyOn(logisticsStore, 'create').mockResolvedValue(mockLogisticsLine)

      const wrapper = mount(LogisticsLineForm, {
        props: { show: true },
        global: { plugins: [pinia] },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any

      // Set up valid form data for creation
      vm.formData.from_factory = 'factory-1'
      vm.formData.to_factory = 'factory-2'
      vm.selectedTransportType = 'Truck'
      vm.transportConfig = {
        transport_type: 'Truck',
        item: 'IronPlate',
        quantity_per_min: 60,
        truck_id: 'TRK-NEW',
      }
      vm.formData.transport_config = {
        transport_type: 'Truck',
        item: 'IronPlate',
        quantity_per_min: 60,
        truck_id: 'TRK-NEW',
      }
      await wrapper.vm.$nextTick()

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      // CRITICAL: create must be called, update must NOT be called
      expect(createSpy).toHaveBeenCalledWith(expect.any(Object))
      expect(updateSpy).not.toHaveBeenCalled()
    })

    it('should not submit when form is invalid', async () => {
      const logisticsStore = useLogisticsStore()
      const updateSpy = vi.spyOn(logisticsStore, 'update').mockResolvedValue(mockLogisticsLine)
      const createSpy = vi.spyOn(logisticsStore, 'create').mockResolvedValue(mockLogisticsLine)

      const wrapper = mount(LogisticsLineForm, {
        props: { show: true },
        global: { plugins: [pinia] },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      // Form is invalid (no factories selected, no transport config)
      expect(vm.isFormValid).toBe(false)

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createSpy).not.toHaveBeenCalled()
      expect(updateSpy).not.toHaveBeenCalled()
    })
  })

  describe('Close behavior', () => {
    it('should emit close when handleCancel is called', async () => {
      const wrapper = mount(LogisticsLineForm, {
        props: { show: true },
        global: { plugins: [pinia] },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.handleCancel()
      await wrapper.vm.$nextTick()

      expect(wrapper.emitted('close')).toBeTruthy()
    })
  })
})
