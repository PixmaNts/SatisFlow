import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import PowerGeneratorForm from '../PowerGeneratorForm.vue'
import { useFactoryStore } from '@/stores/factory'
import { useGameDataStore } from '@/stores/gameData'
import type { PowerGeneratorResponse } from '@/api/types'

// Mock the API endpoints
vi.mock('@/api/endpoints', async () => {
  const actual = await vi.importActual('@/api/endpoints')
  return {
    ...actual,
    factories: {
      ...(actual as any).factories,
      preview: {
        ...(actual as any).factories?.preview,
        powerGenerator: vi.fn().mockResolvedValue({
          total_power_generation: 50.0,
          total_fuel_consumption: 15.0,
          waste_production_rate: 0,
          waste_product: null,
        }),
      },
    },
  }
})

// Mock the useItemIcon composable
vi.mock('@/composables/useItemIcon', () => ({
  useItemIcon: () => ({
    formatItemName: vi.fn((name: string) => name),
    getIconPath: vi.fn(() => '/icons/test.webp'),
  }),
}))

// Mock Modal component
vi.mock('@/components/ui/Modal.vue', () => ({
  default: {
    name: 'Modal',
    template: '<div class="modal-stub"><slot /></div>',
    props: ['show', 'title'],
    emits: ['close'],
  },
}))

// Mock Button component
vi.mock('@/components/ui/Button.vue', () => ({
  default: {
    name: 'Button',
    template: '<button :class="variant" :disabled="disabled" @click="$emit(\'click\')"><slot /></button>',
    props: ['variant', 'size', 'loading', 'disabled', 'type'],
    emits: ['click'],
  },
}))

describe('PowerGeneratorForm', () => {
  let pinia: ReturnType<typeof createPinia>
  let factoryStore: ReturnType<typeof useFactoryStore>
  let gameDataStore: ReturnType<typeof useGameDataStore>
  const mockFactoryId = '550e8400-e29b-41d4-a716-446655440000'

  const allTestItems = [
    'Biomass', 'Leaves', 'Mycelia', 'Wood', 'AlienProtein', 'FlowerPetals',
    'BaconAgaric', 'BerylNut', 'Paleberry',
    'Coal', 'CompactedCoal',
    'Fuel', 'Turbofuel', 'LiquidBiofuel', 'PackagedLiquidBiofuel',
    'UraniumFuelRod', 'PlutoniumFuelRod', 'FicsoniumFuelRod',
    'IronOre',
  ]

  beforeEach(async () => {
    pinia = createPinia()
    setActivePinia(pinia)

    factoryStore = useFactoryStore()
    gameDataStore = useGameDataStore()

    // Pre-seed items in gameDataStore
    gameDataStore.items = allTestItems as any

    vi.clearAllMocks()
  })

  describe('Initialization', () => {
    it('Form initializes empty on mount (no generator_type, no fuel_type, no groups)', async () => {
      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.formData.generator_type).toBe('')
      expect(vm.formData.fuel_type).toBeNull()
      expect(vm.formData.groups).toEqual([])
    })

    it('Form loads power generator data for editing (pre-fills from powerGenerator prop)', async () => {
      const mockPowerGenerator: PowerGeneratorResponse = {
        id: 'generator-123',
        generator_type: 'Coal',
        fuel_type: 'Coal',
        groups: [
          { number_of_generators: 2, clock_speed: 100 },
          { number_of_generators: 1, clock_speed: 150 },
        ],
        total_power_generation: 100,
        total_fuel_consumption: 30,
        waste_production_rate: 0,
        waste_product: null,
      }

      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
          powerGenerator: mockPowerGenerator,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.formData.generator_type).toBe('Coal')
      expect(vm.formData.fuel_type).toBe('Coal')
      expect(vm.formData.groups).toHaveLength(2)
      expect(vm.formData.groups[0].number_of_generators).toBe(2)
      expect(vm.formData.groups[0].clock_speed).toBe(100)
      expect(vm.formData.groups[1].number_of_generators).toBe(1)
      expect(vm.formData.groups[1].clock_speed).toBe(150)
    })

    it('Fetches items on mount (gameDataStore.fetchItems called)', async () => {
      vi.spyOn(gameDataStore, 'fetchItems').mockResolvedValue()

      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      expect(gameDataStore.fetchItems).toHaveBeenCalled()
    })
  })

  describe('Validation — canSubmit', () => {
    it('canSubmit false when no generator_type selected', async () => {
      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      // Add a group to satisfy that part of validation
      vm.formData.groups = [{ number_of_generators: 1, clock_speed: 100 }]
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBeFalsy()
    })

    it('canSubmit false when fuel type missing for fuel-requiring generator (e.g. Coal)', async () => {
      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.generator_type = 'Coal'
      vm.formData.groups = [{ number_of_generators: 1, clock_speed: 100 }]
      // fuel_type is null by default
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBeFalsy()
    })

    it('canSubmit true for Geothermal without fuel (showFuelType=false, no fuel required)', async () => {
      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.generator_type = 'Geothermal'
      vm.formData.groups = [{ number_of_generators: 1, clock_speed: 100 }]
      await wrapper.vm.$nextTick()

      expect(vm.showFuelType).toBe(false)
      expect(vm.canSubmit).toBe(true)
    })

    it('canSubmit true with valid Coal generator + fuel + groups', async () => {
      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.generator_type = 'Coal'
      vm.formData.fuel_type = 'Coal'
      vm.formData.groups = [{ number_of_generators: 1, clock_speed: 100 }]
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBeTruthy()
    })

    it('canSubmit false with invalid groups (number_of_generators = 0 or clock_speed > 250)', async () => {
      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.generator_type = 'Coal'
      vm.formData.fuel_type = 'Coal'
      // Invalid: number_of_generators = 0
      vm.formData.groups = [{ number_of_generators: 0, clock_speed: 100 }]
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBe(false)

      // Invalid: clock_speed > 250
      vm.formData.groups = [{ number_of_generators: 1, clock_speed: 300 }]
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBe(false)

      // Invalid: clock_speed < 0
      vm.formData.groups = [{ number_of_generators: 1, clock_speed: -10 }]
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBe(false)
    })
  })

  describe('Fuel filtering', () => {
    it('Biomass generator shows only biomass-compatible items', async () => {
      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.generator_type = 'Biomass'
      await wrapper.vm.$nextTick()

      const expectedBiomassFuels = [
        'Biomass', 'Leaves', 'Mycelia', 'Wood', 'AlienProtein',
        'FlowerPetals', 'BaconAgaric', 'BerylNut', 'Paleberry',
      ]
      expect(vm.availableFuels).toEqual(expectedBiomassFuels)
    })

    it('Coal generator shows only Coal + CompactedCoal', async () => {
      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.generator_type = 'Coal'
      await wrapper.vm.$nextTick()

      expect(vm.availableFuels).toEqual(['Coal', 'CompactedCoal'])
    })

    it('Fuel generator shows Fuel + Turbofuel + LiquidBiofuel + PackagedLiquidBiofuel', async () => {
      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.generator_type = 'Fuel'
      await wrapper.vm.$nextTick()

      expect(vm.availableFuels).toEqual([
        'Fuel', 'Turbofuel', 'LiquidBiofuel', 'PackagedLiquidBiofuel',
      ])
    })

    it('Nuclear generator shows fuel rods (UraniumFuelRod, PlutoniumFuelRod, FicsoniumFuelRod)', async () => {
      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.generator_type = 'Nuclear'
      await wrapper.vm.$nextTick()

      expect(vm.availableFuels).toEqual([
        'UraniumFuelRod', 'PlutoniumFuelRod', 'FicsoniumFuelRod',
      ])
    })

    it('Geothermal generator: showFuelType=false (no fuel dropdown)', async () => {
      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.generator_type = 'Geothermal'
      await wrapper.vm.$nextTick()

      expect(vm.showFuelType).toBe(false)
      expect(vm.availableFuels).toEqual([])
    })
  })

  describe('Form submission', () => {
    it('handleSubmit calls factoryStore.createPowerGenerator with correct payload for new generator', async () => {
      const createSpy = vi.spyOn(factoryStore, 'createPowerGenerator').mockResolvedValue({
        id: mockFactoryId,
        name: 'Test Factory',
        description: '',
        notes: '',
        production_lines: [],
        raw_inputs: [],
        power_generators: [],
        items: [],
        total_power_consumption: 0,
        total_power_generation: 0,
        power_balance: 0,
      })

      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.generator_type = 'Coal'
      vm.formData.fuel_type = 'Coal'
      vm.formData.groups = [{ number_of_generators: 2, clock_speed: 100 }]
      await wrapper.vm.$nextTick()

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createSpy).toHaveBeenCalledWith(mockFactoryId, {
        generator_type: 'Coal',
        fuel_type: 'Coal',
        groups: [{ number_of_generators: 2, clock_speed: 100 }],
      })
    })

    it('handleSubmit calls factoryStore.updatePowerGenerator for editing (passes powerGenerator.id)', async () => {
      const updateSpy = vi.spyOn(factoryStore, 'updatePowerGenerator').mockResolvedValue({
        id: mockFactoryId,
        name: 'Test Factory',
        description: '',
        notes: '',
        production_lines: [],
        raw_inputs: [],
        power_generators: [],
        items: [],
        total_power_consumption: 0,
        total_power_generation: 0,
        power_balance: 0,
      })

      const mockPowerGenerator: PowerGeneratorResponse = {
        id: 'generator-123',
        generator_type: 'Coal',
        fuel_type: 'Coal',
        groups: [{ number_of_generators: 1, clock_speed: 100 }],
        total_power_generation: 50,
        total_fuel_consumption: 15,
        waste_production_rate: 0,
        waste_product: null,
      }

      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
          powerGenerator: mockPowerGenerator,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      // Update the groups
      vm.formData.groups = [{ number_of_generators: 3, clock_speed: 150 }]
      await wrapper.vm.$nextTick()

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(updateSpy).toHaveBeenCalledWith(mockFactoryId, 'generator-123', {
        generator_type: 'Coal',
        fuel_type: 'Coal',
        groups: [{ number_of_generators: 3, clock_speed: 150 }],
      })
    })

    it('handleSubmit deletes fuel_type from payload when showFuelType is false (Geothermal)', async () => {
      const createSpy = vi.spyOn(factoryStore, 'createPowerGenerator').mockResolvedValue({
        id: mockFactoryId,
        name: 'Test Factory',
        description: '',
        notes: '',
        production_lines: [],
        raw_inputs: [],
        power_generators: [],
        items: [],
        total_power_consumption: 0,
        total_power_generation: 0,
        power_balance: 0,
      })

      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.generator_type = 'Geothermal'
      vm.formData.fuel_type = null
      vm.formData.groups = [{ number_of_generators: 1, clock_speed: 100 }]
      await wrapper.vm.$nextTick()

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      // Verify fuel_type is NOT in the payload (deleted, not undefined)
      expect(createSpy).toHaveBeenCalledWith(mockFactoryId, {
        generator_type: 'Geothermal',
        groups: [{ number_of_generators: 1, clock_speed: 100 }],
      })
      const calledPayload = createSpy.mock.calls[0][1]
      expect('fuel_type' in calledPayload).toBe(false)
    })
  })

  describe('Generator type change', () => {
    it('handleGeneratorTypeChange resets fuel_type and groups when changing type', async () => {
      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      // Set up initial Coal generator
      vm.formData.generator_type = 'Coal'
      vm.formData.fuel_type = 'Coal'
      vm.formData.groups = [{ number_of_generators: 2, clock_speed: 150 }]
      await wrapper.vm.$nextTick()

      expect(vm.formData.fuel_type).toBe('Coal')
      expect(vm.formData.groups).toHaveLength(1)

      // Change to Biomass
      vm.formData.generator_type = 'Biomass'
      await vm.handleGeneratorTypeChange()
      await wrapper.vm.$nextTick()

      // fuel_type should be reset (Biomass requires fuel but the first available is set)
      expect(vm.formData.groups).toHaveLength(1)
      expect(vm.formData.groups[0].number_of_generators).toBe(1)
      expect(vm.formData.groups[0].clock_speed).toBe(100)

      // Change to Geothermal - fuel should be null
      vm.formData.generator_type = 'Geothermal'
      await vm.handleGeneratorTypeChange()
      await wrapper.vm.$nextTick()

      expect(vm.formData.fuel_type).toBeNull()
      expect(vm.formData.groups).toHaveLength(1)
    })
  })

  describe('Close behavior', () => {
    it('handleClose emits update:show(false) and resets form', async () => {
      const wrapper = mount(PowerGeneratorForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      // Set some form data
      vm.formData.generator_type = 'Coal'
      vm.formData.fuel_type = 'Coal'
      vm.formData.groups = [{ number_of_generators: 2, clock_speed: 100 }]
      await wrapper.vm.$nextTick()

      vm.handleClose()
      await wrapper.vm.$nextTick()

      // Check emit
      expect(wrapper.emitted('update:show')).toBeTruthy()
      const updateShowEvents = wrapper.emitted('update:show')
      expect(updateShowEvents![0]).toEqual([false])

      // Check form reset
      expect(vm.formData.generator_type).toBe('')
      expect(vm.formData.fuel_type).toBeNull()
      expect(vm.formData.groups).toEqual([])
    })
  })
})
