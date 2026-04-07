import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import ProductionLineForm from '../ProductionLineForm.vue'
import { useFactoryStore } from '@/stores/factory'
import { useGameDataStore } from '@/stores/gameData'
import type {
  ProductionLineResponse,
  MachineGroup,
  RecipeInfo,
  MachineInfo,
  FactoryResponse
} from '@/api/types'

// Mock API endpoints
vi.mock('@/api/endpoints', async () => {
  const actual = await vi.importActual('@/api/endpoints')
  return {
    ...actual,
    factories: {
      ...(actual as any).factories,
      preview: {
        ...(actual as any).factories?.preview,
        productionLine: vi.fn().mockResolvedValue({
          total_power_consumption: 15.5,
          total_machines: 2,
          total_somersloop: 0,
          input_rate: [],
          output_rate: [],
        }),
      },
    },
    blueprintTemplates: {
      ...(actual as any).blueprintTemplates,
      getAll: vi.fn().mockResolvedValue([]),
      createInstanceInFactory: vi.fn().mockResolvedValue({ success: true }),
    },
  }
})

// Stub child components
vi.mock('@/components/ui/Modal.vue', () => ({
  default: {
    name: 'Modal',
    template: '<div class="modal-stub"><slot /></div>',
    props: ['show', 'title'],
    emits: ['close'],
  },
}))
vi.mock('@/components/ui/Button.vue', () => ({
  default: {
    name: 'Button',
    template: '<button :class="variant" :disabled="disabled" @click="$emit(\'click\')"><slot /></button>',
    props: ['variant', 'size', 'loading', 'disabled', 'type'],
    emits: ['click'],
  },
}))
vi.mock('../RecipeAutocomplete.vue', () => ({
  default: {
    name: 'RecipeAutocomplete',
    template: '<div class="recipe-autocomplete-stub" />',
    props: ['modelValue', 'recipes', 'placeholder', 'disabled'],
    emits: ['update:modelValue', 'selected', 'cleared'],
  },
}))
vi.mock('@/components/forms/SearchableSelect.vue', () => ({
  default: {
    name: 'SearchableSelect',
    template: '<div class="searchable-select-stub" />',
    props: ['modelValue', 'options', 'placeholder', 'disabled'],
    emits: ['update:modelValue'],
  },
}))

describe('ProductionLineForm', () => {
  let pinia: ReturnType<typeof createPinia>
  let factoryStore: ReturnType<typeof useFactoryStore>
  let gameDataStore: ReturnType<typeof useGameDataStore>
  const mockFactoryId = '550e8400-e29b-41d4-a716-446655440000'

  const mockRecipe1: RecipeInfo = {
    name: 'Iron Plate',
    machine: 'Constructor',
    inputs: [{ item: 'Iron Ingot', quantity: 30 }],
    outputs: [{ item: 'Iron Plate', quantity: 20 }],
  }

  const mockRecipe2: RecipeInfo = {
    name: 'Copper Ingot',
    machine: 'Smelter',
    inputs: [{ item: 'Copper Ore', quantity: 30 }],
    outputs: [{ item: 'Copper Ingot', quantity: 30 }],
  }

  const mockMachineInfo: MachineInfo = {
    name: 'Constructor',
    base_power: 15,
    max_somersloop: 1,
  }

  const mockFactory: FactoryResponse = {
    id: mockFactoryId,
    name: 'Test Factory',
    description: 'Test Description',
    notes: null,
    production_lines: [],
    raw_inputs: [],
    power_generators: [],
    items: [],
    total_power_consumption: 0,
    total_power_generation: 0,
    power_balance: 0,
  }

  beforeEach(async () => {
    pinia = createPinia()
    setActivePinia(pinia)

    gameDataStore = useGameDataStore()
    gameDataStore.recipes = [mockRecipe1, mockRecipe2]
    vi.spyOn(gameDataStore, 'getRecipeByName').mockImplementation((name: string) => {
      if (name === 'Iron Plate') return mockRecipe1
      if (name === 'Copper Ingot') return mockRecipe2
      return null
    })
    vi.spyOn(gameDataStore, 'getMachineByName').mockImplementation((name: string) => {
      if (name === 'Constructor') return mockMachineInfo
      return null
    })

    factoryStore = useFactoryStore()
    vi.clearAllMocks()
  })

  // =========================================================================
  // 1. Initialization
  // =========================================================================
  describe('Initialization', () => {
    it('Form initializes empty in create mode (no productionLine prop)', async () => {
      const wrapper = mount(ProductionLineForm, {
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
      expect(vm.formData.name).toBe('')
      expect(vm.formData.description).toBe('')
      expect(vm.formData.type).toBe('recipe')
      expect(vm.formData.recipe).toBe('')
      expect(vm.formData.blueprint_template_id).toBe('')
      expect(vm.formData.machine_groups).toEqual([])
      expect(vm.isEditing).toBe(false)
    })

    it('Form loads production line recipe data in edit mode', async () => {
      const machineGroups: MachineGroup[] = [
        { number_of_machine: 2, oc_value: 150, somersloop: 0 },
      ]

      const productionLine: ProductionLineResponse = {
        ProductionLineRecipe: {
          id: 'line-123',
          name: 'Iron Plate Line',
          description: 'Produces iron plates',
          recipe: 'Iron Plate',
          machine_groups: machineGroups,
        },
        total_power_consumption: 45,
        total_machines: 2,
        total_somersloop: 0,
        input_rate: [],
        output_rate: [],
      }

      const wrapper = mount(ProductionLineForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
          productionLine,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      const vm = wrapper.vm as any
      expect(vm.isEditing).toBe(true)
      expect(vm.formData.name).toBe('Iron Plate Line')
      expect(vm.formData.description).toBe('Produces iron plates')
      expect(vm.formData.type).toBe('recipe')
      expect(vm.formData.recipe).toBe('Iron Plate')
      expect(vm.formData.machine_groups).toHaveLength(1)
      expect(vm.formData.machine_groups[0].number_of_machine).toBe(2)
      expect(vm.formData.machine_groups[0].oc_value).toBe(150)
    })

    it('Form loads production line blueprint data in edit mode', async () => {
      const productionLine: ProductionLineResponse = {
        ProductionLineBlueprint: {
          id: 'line-456',
          name: 'My Blueprint',
          description: 'A blueprint template',
          production_lines: [],
        },
        total_power_consumption: 0,
        total_machines: 0,
        total_somersloop: 0,
        input_rate: [],
        output_rate: [],
      }

      const wrapper = mount(ProductionLineForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
          productionLine,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      const vm = wrapper.vm as any
      expect(vm.isEditing).toBe(true)
      expect(vm.formData.name).toBe('My Blueprint')
      expect(vm.formData.description).toBe('A blueprint template')
      expect(vm.formData.type).toBe('blueprint')
    })
  })

  // =========================================================================
  // 2. Validation — canSubmit
  // =========================================================================
  describe('Validation — canSubmit', () => {
    it('canSubmit false when name is empty', async () => {
      const wrapper = mount(ProductionLineForm, {
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
      vm.formData.name = ''
      vm.formData.recipe = 'Iron Plate'
      vm.formData.machine_groups = [
        { number_of_machine: 1, oc_value: 100, somersloop: 0 },
      ]
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBe(false)
    })

    it('canSubmit false when recipe type has no recipe selected', async () => {
      const wrapper = mount(ProductionLineForm, {
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
      vm.formData.name = 'Test Line'
      vm.formData.recipe = ''
      vm.formData.machine_groups = [
        { number_of_machine: 1, oc_value: 100, somersloop: 0 },
      ]
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBeFalsy()
    })

    it('canSubmit false when machine groups have invalid oc_value', async () => {
      const wrapper = mount(ProductionLineForm, {
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
      vm.formData.name = 'Test Line'
      vm.formData.recipe = 'Iron Plate'
      vm.formData.machine_groups = [
        { number_of_machine: 1, oc_value: 300, somersloop: 0 }, // oc_value > 250
      ]
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBe(false)

      // Also test negative oc_value
      vm.formData.machine_groups = [
        { number_of_machine: 1, oc_value: -10, somersloop: 0 },
      ]
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBe(false)
    })

    it('canSubmit true with valid recipe form', async () => {
      const wrapper = mount(ProductionLineForm, {
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
      vm.formData.name = 'Test Line'
      vm.formData.recipe = 'Iron Plate'
      vm.formData.machine_groups = [
        { number_of_machine: 1, oc_value: 100, somersloop: 0 },
      ]
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBe(true)
    })

    it('canSubmit true with valid blueprint form', async () => {
      const wrapper = mount(ProductionLineForm, {
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
      vm.formData.name = 'Blueprint Line'
      vm.formData.type = 'blueprint'
      vm.formData.blueprint_template_id = 'template-123'
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBe(true)
    })
  })

  // =========================================================================
  // 3. Form submission — recipe type
  // =========================================================================
  describe('Form submission — recipe type', () => {
    it('handleSubmit calls factoryStore.createProductionLine with correct payload for new line', async () => {
      const createSpy = vi.spyOn(factoryStore, 'createProductionLine').mockResolvedValue(mockFactory)

      const wrapper = mount(ProductionLineForm, {
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
      vm.formData.name = 'New Iron Line'
      vm.formData.description = 'Making iron plates'
      vm.formData.recipe = 'Iron Plate'
      vm.formData.machine_groups = [
        { number_of_machine: 2, oc_value: 150, somersloop: 0 },
      ]
      await wrapper.vm.$nextTick()

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createSpy).toHaveBeenCalledWith(mockFactoryId, {
        name: 'New Iron Line',
        description: 'Making iron plates',
        type: 'recipe',
        recipe: 'Iron Plate',
        machine_groups: [
          { number_of_machine: 2, oc_value: 150, somersloop: 0 },
        ],
      })
    })

    it('handleSubmit calls factoryStore.updateProductionLine for editing existing line', async () => {
      const updateSpy = vi.spyOn(factoryStore, 'updateProductionLine').mockResolvedValue(mockFactory)

      const productionLine: ProductionLineResponse = {
        ProductionLineRecipe: {
          id: 'line-123',
          name: 'Existing Line',
          description: null,
          recipe: 'Iron Plate',
          machine_groups: [
            { number_of_machine: 1, oc_value: 100, somersloop: 0 },
          ],
        },
        total_power_consumption: 15,
        total_machines: 1,
        total_somersloop: 0,
        input_rate: [],
        output_rate: [],
      }

      const wrapper = mount(ProductionLineForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
          productionLine,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      const vm = wrapper.vm as any
      vm.formData.name = 'Updated Line'
      await wrapper.vm.$nextTick()

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(updateSpy).toHaveBeenCalledWith(
        mockFactoryId,
        'line-123',
        expect.objectContaining({
          name: 'Updated Line',
          type: 'recipe',
          recipe: 'Iron Plate',
        })
      )
    })

    it('handleSubmit emits saved and closes on success', async () => {
      vi.spyOn(factoryStore, 'createProductionLine').mockResolvedValue(mockFactory)

      const wrapper = mount(ProductionLineForm, {
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
      vm.formData.name = 'New Line'
      vm.formData.recipe = 'Iron Plate'
      vm.formData.machine_groups = [
        { number_of_machine: 1, oc_value: 100, somersloop: 0 },
      ]
      await wrapper.vm.$nextTick()

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(wrapper.emitted('saved')).toBeTruthy()
      expect(wrapper.emitted('update:show')).toBeTruthy()
      const closeEvents = wrapper.emitted('update:show')
      expect(closeEvents![0]).toEqual([false])
    })
  })

  // =========================================================================
  // 4. Form submission — blueprint type
  // =========================================================================
  describe('Form submission — blueprint type', () => {
    it('handleSubmit with blueprint type calls blueprintTemplates.createInstanceInFactory', async () => {
      const { blueprintTemplates } = await import('@/api/endpoints')
      const createInstanceSpy = vi.spyOn(blueprintTemplates, 'createInstanceInFactory')

      const wrapper = mount(ProductionLineForm, {
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
      vm.formData.name = 'Blueprint Line'
      vm.formData.type = 'blueprint'
      vm.formData.blueprint_template_id = 'template-789'
      await wrapper.vm.$nextTick()

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createInstanceSpy).toHaveBeenCalledWith(
        mockFactoryId,
        'template-789',
        { name: 'Blueprint Line' }
      )
    })
  })

  // =========================================================================
  // 5. Type change and machine groups
  // =========================================================================
  describe('Type change and machine groups', () => {
    it('handleTypeChange resets recipe fields when switching to blueprint', async () => {
      const wrapper = mount(ProductionLineForm, {
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
      vm.formData.name = 'Test Line'
      vm.formData.recipe = 'Iron Plate'
      vm.formData.machine_groups = [
        { number_of_machine: 2, oc_value: 150, somersloop: 0 },
      ]
      await wrapper.vm.$nextTick()

      vm.formData.type = 'blueprint'
      await wrapper.vm.$nextTick()

      vm.handleTypeChange()
      await wrapper.vm.$nextTick()

      // handleTypeChange resets recipe fields when switching to blueprint
      // (type change is done via v-model on radio, handleTypeChange only resets fields)
      expect(vm.formData.type).toBe('blueprint')
      expect(vm.formData.recipe).toBe('')
      expect(vm.formData.machine_groups).toEqual([])
    })

    it('handleTypeChange resets blueprint fields when switching to recipe', async () => {
      const wrapper = mount(ProductionLineForm, {
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
      // Set initial blueprint state
      vm.formData.type = 'blueprint'
      vm.formData.blueprint_template_id = 'template-123'
      await wrapper.vm.$nextTick()

      // Switch to recipe (via v-model), then handleTypeChange clears blueprint fields
      vm.formData.type = 'recipe'
      await wrapper.vm.$nextTick()

      vm.handleTypeChange()
      await wrapper.vm.$nextTick()

      // handleTypeChange resets blueprint fields when switching to recipe
      expect(vm.formData.type).toBe('recipe')
      expect(vm.formData.blueprint_template_id).toBe('')
    })

    it('addMachineGroup adds a default group, removeMachineGroup removes at index', async () => {
      const wrapper = mount(ProductionLineForm, {
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
      expect(vm.formData.machine_groups).toHaveLength(0)

      vm.addMachineGroup()
      await wrapper.vm.$nextTick()

      expect(vm.formData.machine_groups).toHaveLength(1)
      expect(vm.formData.machine_groups[0]).toEqual({
        number_of_machine: 1,
        oc_value: 100,
        somersloop: 0,
      })

      // Add another group
      vm.addMachineGroup()
      await wrapper.vm.$nextTick()

      expect(vm.formData.machine_groups).toHaveLength(2)

      // Remove first group
      vm.removeMachineGroup(0)
      await wrapper.vm.$nextTick()

      expect(vm.formData.machine_groups).toHaveLength(1)
    })
  })

  // =========================================================================
  // 6. Close behavior
  // =========================================================================
  describe('Close behavior', () => {
    it('handleClose emits update:show with false', async () => {
      const wrapper = mount(ProductionLineForm, {
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
      vm.handleClose()
      await wrapper.vm.$nextTick()

      expect(wrapper.emitted('update:show')).toBeTruthy()
      const events = wrapper.emitted('update:show')
      expect(events![0]).toEqual([false])
    })

    it('handleClose resets form data to initial empty state', async () => {
      const wrapper = mount(ProductionLineForm, {
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
      vm.formData.name = 'Changed Name'
      vm.formData.description = 'Changed Description'
      vm.formData.recipe = 'Iron Plate'
      vm.formData.machine_groups = [
        { number_of_machine: 2, oc_value: 150, somersloop: 0 },
      ]
      await wrapper.vm.$nextTick()

      vm.handleClose()
      await wrapper.vm.$nextTick()

      expect(vm.formData.name).toBe('')
      expect(vm.formData.description).toBe('')
      expect(vm.formData.type).toBe('recipe')
      expect(vm.formData.recipe).toBe('')
      expect(vm.formData.machine_groups).toEqual([])
    })
  })
})
