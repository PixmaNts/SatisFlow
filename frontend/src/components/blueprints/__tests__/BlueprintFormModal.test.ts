import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import BlueprintFormModal from '../BlueprintFormModal.vue'
import type { BlueprintTemplateResponse } from '@/api/types'

// Mock the API endpoints
vi.mock('@/api/endpoints', () => ({
  gameData: {
    getRecipes: vi.fn().mockResolvedValue([]),
  },
}))

// Mock child components that have complex dependencies
vi.mock('@/components/ui/BaseInput.vue', () => ({
  default: {
    name: 'BaseInput',
    template: '<input class="base-input" :value="modelValue" @input="$emit(\'update:modelValue\', $event.target.value)" />',
    props: ['modelValue', 'placeholder', 'error', 'required'],
    emits: ['update:modelValue'],
  }
}))
vi.mock('@/components/ui/Collapsible.vue', () => ({
  default: {
    name: 'Collapsible',
    template: '<div class="collapsible"><slot name="header" /><slot /></div>',
    props: ['defaultOpen'],
  }
}))
vi.mock('@/components/ui/ConfirmDialog.vue', () => ({
  default: {
    name: 'ConfirmDialog',
    template: '<div class="confirm-dialog" v-if="show"><slot /><button @click="$emit(\'confirm\')">Confirm</button></div>',
    props: ['show', 'title', 'message', 'variant', 'confirmText'],
    emits: ['confirm', 'cancel'],
  }
}))
vi.mock('@/components/ui/Button.vue', () => ({
  default: {
    name: 'Button',
    template: '<button class="btn" :disabled="disabled" @click="$emit(\'click\')"><slot /></button>',
    props: ['variant', 'size', 'disabled', 'loading'],
    emits: ['click'],
  }
}))
vi.mock('@/components/forms/FormNumber.vue', () => ({
  default: {
    name: 'FormNumber',
    template: '<input class="form-number" type="number" :value="modelValue" @input="$emit(\'update:modelValue\', $event.target.value)" />',
    props: ['modelValue', 'min', 'max', 'placeholder', 'error', 'required'],
    emits: ['update:modelValue'],
  }
}))
vi.mock('@/components/forms/RangeSlider.vue', () => ({
  default: {
    name: 'RangeSlider',
    template: '<input class="range-slider" type="range" :value="modelValue" @input="$emit(\'update:modelValue\', $event.target.value)" />',
    props: ['modelValue', 'min', 'max', 'step', 'unit', 'presets', 'error', 'required', 'compact', 'showQuickPresets'],
    emits: ['update:modelValue'],
  }
}))
vi.mock('@/components/factory/RecipeAutocomplete.vue', () => ({
  default: {
    name: 'RecipeAutocomplete',
    template: '<input class="recipe-autocomplete" :value="modelValue" @input="$emit(\'update:modelValue\', $event.target.value)" />',
    props: ['modelValue', 'recipes', 'placeholder', 'disabled'],
    emits: ['update:modelValue'],
  }
}))
vi.mock('@/components/ui/Modal.vue', () => ({
  default: {
    name: 'Modal',
    template: '<div class="modal" v-if="show"><slot /><slot name="footer" /></div>',
    props: ['show', 'title', 'size'],
    emits: ['close'],
  }
}))

describe('BlueprintFormModal', () => {
  let pinia: ReturnType<typeof createPinia>

  const mockTemplate: BlueprintTemplateResponse = {
    id: 'template-1',
    name: 'Test Blueprint',
    description: 'Test Description',
    total_machines: 2,
    total_power: 10.5,
    input_items: [],
    output_items: [],
    production_lines: [
      {
        id: 'pl-1',
        name: 'Production Line 1',
        description: 'Line Description',
        recipe: 'IronPlate',
        machine_groups: [
          {
            number_of_machine: 2,
            oc_value: 100,
            somersloop: 0,
          },
        ],
      },
    ],
  }

  beforeEach(async () => {
    pinia = createPinia()
    setActivePinia(pinia)
    vi.clearAllMocks()
  })

  const mountComponent = (propsOverrides = {}) => {
    return mount(BlueprintFormModal, {
      props: {
        show: true,
        ...propsOverrides,
      },
      global: {
        plugins: [pinia],
      },
    })
  }

  describe('Initialization from template (edit mode)', () => {
    it('should populate formData from template when opened with template prop', async () => {
      const wrapper = mountComponent({ template: mockTemplate })

      // Wait for watchers to fire (show watcher triggers initializeForm)
      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.formData.name).toBe('Test Blueprint')
      expect(vm.formData.description).toBe('Test Description')
      expect(vm.formData.production_lines).toHaveLength(1)
      expect(vm.formData.production_lines[0].name).toBe('Production Line 1')
      expect(vm.formData.production_lines[0].recipe).toBe('IronPlate')
      expect(vm.formData.production_lines[0].machine_groups).toHaveLength(1)
      expect(vm.formData.production_lines[0].machine_groups[0].number_of_machine).toBe(2)
    })

    it('should start with empty form when no template provided', async () => {
      const wrapper = mountComponent()

      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.formData.name).toBe('')
      expect(vm.formData.description).toBe('')
      expect(vm.formData.production_lines).toHaveLength(0)
    })

    it('should reinitialize form when show changes from false to true', async () => {
      const wrapper = mountComponent({ template: mockTemplate })

      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      // Confirm data loaded
      expect(vm.formData.name).toBe('Test Blueprint')

      // Close the modal
      await wrapper.setProps({ show: false })
      await wrapper.vm.$nextTick()

      // Reopen
      await wrapper.setProps({ show: true })
      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      // Should reinitialize from template
      expect(vm.formData.name).toBe('Test Blueprint')
      expect(vm.formData.production_lines).toHaveLength(1)
    })
  })

  describe('Validation (isValid)', () => {
    it('should be invalid when form is empty', async () => {
      const wrapper = mountComponent()

      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.isValid).toBe(false)
    })

    it('should be invalid when name is filled but no production lines', async () => {
      const wrapper = mountComponent()

      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.name = 'Some Blueprint'
      await wrapper.vm.$nextTick()

      expect(vm.isValid).toBe(false)
    })

    it('should be valid with template data (name + lines + machines)', async () => {
      const wrapper = mountComponent({ template: mockTemplate })

      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      // Template has name='Test Blueprint', 1 line with recipe='IronPlate', 1 group with 2 machines
      expect(vm.isValid).toBe(true)
    })
  })

  describe('Save behavior', () => {
    it('should include template ID in save data when editing', async () => {
      const wrapper = mountComponent({ template: mockTemplate })

      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      await vm.onSave()
      await wrapper.vm.$nextTick()

      const saveEvents = wrapper.emitted('save')
      expect(saveEvents).toBeTruthy()
      expect(saveEvents![0][0]).toHaveProperty('id', 'template-1')
      expect(saveEvents![0][0].name).toBe('Test Blueprint')
    })

    it('should NOT include id in save data when creating new', async () => {
      const wrapper = mountComponent()

      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      // Manually set valid form data for creation
      vm.formData.name = 'New Blueprint'
      vm.formData.production_lines = [{
        name: 'Line 1',
        description: '',
        type: 'recipe',
        recipe: 'IronPlate',
        machine_groups: [{ number_of_machine: 1, oc_value: 100, somersloop: 0 }],
      }]
      await wrapper.vm.$nextTick()

      await vm.onSave()
      await wrapper.vm.$nextTick()

      const saveEvents = wrapper.emitted('save')
      expect(saveEvents).toBeTruthy()
      expect(saveEvents![0][0]).not.toHaveProperty('id')
      expect(saveEvents![0][0].name).toBe('New Blueprint')
    })

    it('should not emit save when form is invalid', async () => {
      const wrapper = mountComponent()

      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      await vm.onSave()
      await wrapper.vm.$nextTick()

      expect(wrapper.emitted('save')).toBeFalsy()
    })
  })

  describe('Production line management', () => {
    it('should add a production line with default machine group', async () => {
      const wrapper = mountComponent()

      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.formData.production_lines).toHaveLength(0)

      vm.addProductionLine()
      await wrapper.vm.$nextTick()

      expect(vm.formData.production_lines).toHaveLength(1)
      expect(vm.formData.production_lines[0].machine_groups).toHaveLength(1)
      expect(vm.formData.production_lines[0].machine_groups[0].number_of_machine).toBe(1)
      expect(vm.formData.production_lines[0].machine_groups[0].oc_value).toBe(100)
    })

    it('should remove a production line', async () => {
      const wrapper = mountComponent({ template: mockTemplate })

      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.formData.production_lines).toHaveLength(1)

      vm.removeProductionLine(0)
      await wrapper.vm.$nextTick()

      expect(vm.formData.production_lines).toHaveLength(0)
    })
  })

  describe('Computed values', () => {
    it('should calculate totalMachines correctly', async () => {
      const wrapper = mountComponent({ template: mockTemplate })

      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      // Template has 1 line with 1 group of 2 machines
      expect(vm.totalMachines).toBe(2)
    })

    it('should calculate totalPower correctly', async () => {
      const wrapper = mountComponent({ template: mockTemplate })

      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      // Power = machine_groups.length * 16 = 1 * 16 = 16
      expect(vm.totalPower).toBe(16)
    })

    it('should have 0 machines and power when empty', async () => {
      const wrapper = mountComponent()

      await wrapper.vm.$nextTick()
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.totalMachines).toBe(0)
      expect(vm.totalPower).toBe(0)
    })
  })
})
