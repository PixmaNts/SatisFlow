import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import FactoryEditModal from '../FactoryEditModal.vue'
import { useFactoryStore } from '@/stores/factory'
import type { FactoryResponse } from '@/api/types'

// Mock the API endpoints
vi.mock('@/api/endpoints', async () => {
  const actual = await vi.importActual('@/api/endpoints')

  return {
    ...actual,
    factories: {
      ...(actual as any).factories,
      update: vi.fn().mockResolvedValue({ success: true }),
    },
  }
})

// Mock the toast composable
vi.mock('@/composables/useToast', () => ({
  useToast: () => ({
    showSuccess: vi.fn(),
    showError: vi.fn(),
  }),
}))

import { factories } from '@/api/endpoints'

describe('FactoryEditModal', () => {
  let pinia: ReturnType<typeof createPinia>
  let factoryStore: ReturnType<typeof useFactoryStore>
  const mockFactoryId = '550e8400-e29b-41d4-a716-446655440000'

  const mockFactory: FactoryResponse = {
    id: mockFactoryId,
    name: 'Test Factory',
    description: 'Test Description',
    notes: 'Test Notes',
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

    factoryStore = useFactoryStore()
    vi.clearAllMocks()
  })

  describe('Props and initialization', () => {
    it('should initialize form with factory data when show is true', async () => {
      const wrapper = mount(FactoryEditModal, {
        props: {
          show: true,
          factory: mockFactory,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.formData.name).toBe('Test Factory')
      expect(vm.formData.description).toBe('Test Description')
      expect(vm.formData.notes).toBe('Test Notes')
    })

    it('should reset form when show becomes false', async () => {
      const wrapper = mount(FactoryEditModal, {
        props: {
          show: true,
          factory: mockFactory,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      // Change show to false
      await wrapper.setProps({ show: false })
      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.formData.name).toBe('')
      expect(vm.formData.description).toBe('')
      expect(vm.formData.notes).toBe('')
    })

    it('should handle null factory gracefully', async () => {
      const wrapper = mount(FactoryEditModal, {
        props: {
          show: true,
          factory: null,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      expect(vm.formData.name).toBe('')
      expect(vm.formData.description).toBe('')
      expect(vm.formData.notes).toBe('')
    })
  })

  describe('Validation', () => {
    it('should disable submit when name is empty', async () => {
      const wrapper = mount(FactoryEditModal, {
        props: {
          show: true,
          factory: mockFactory,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.name = ''
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBe(false)
    })

    it('should enable submit when name is filled', async () => {
      const wrapper = mount(FactoryEditModal, {
        props: {
          show: true,
          factory: mockFactory,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.name = 'New Factory Name'
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBe(true)
    })

    it('should trim whitespace from name', async () => {
      const wrapper = mount(FactoryEditModal, {
        props: {
          show: true,
          factory: mockFactory,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.name = '   Factory Name   '
      await wrapper.vm.$nextTick()

      expect(vm.canSubmit).toBe(true)
    })
  })

  describe('Form submission', () => {
    it('should call update factory with correct payload', async () => {
      vi.mocked(factories.update).mockResolvedValue(mockFactory)

      const wrapper = mount(FactoryEditModal, {
        props: {
          show: true,
          factory: mockFactory,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.name = 'Updated Factory'
      vm.formData.description = 'Updated Description'
      vm.formData.notes = 'Updated Notes'
      await wrapper.vm.$nextTick()

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(factories.update).toHaveBeenCalledWith(mockFactoryId, {
        name: 'Updated Factory',
        description: 'Updated Description',
        notes: 'Updated Notes',
      })
    })

    it('should emit saved and close on successful update', async () => {
      vi.mocked(factories.update).mockResolvedValue(mockFactory)

      const wrapper = mount(FactoryEditModal, {
        props: {
          show: true,
          factory: mockFactory,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(wrapper.emitted('saved')).toBeTruthy()
      expect(wrapper.emitted('update:show')).toBeTruthy()
    })

    it('should not submit when canSubmit is false', async () => {
      const wrapper = mount(FactoryEditModal, {
        props: {
          show: true,
          factory: mockFactory,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      const vm = wrapper.vm as any
      vm.formData.name = '' // Empty name
      await wrapper.vm.$nextTick()

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(factories.update).not.toHaveBeenCalled()
    })
  })

  describe('Close behavior', () => {
    it('should emit update:show false when handleClose is called', async () => {
      const wrapper = mount(FactoryEditModal, {
        props: {
          show: true,
          factory: mockFactory,
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
      const updateShowEvents = wrapper.emitted('update:show')
      expect(updateShowEvents![0]).toEqual([false])
    })

    it('should reset form when closed', async () => {
      const wrapper = mount(FactoryEditModal, {
        props: {
          show: true,
          factory: mockFactory,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()

      let vm = wrapper.vm as any
      vm.formData.name = 'Changed Name'
      vm.formData.description = 'Changed Description'
      await wrapper.vm.$nextTick()

      vm.handleClose()
      await wrapper.vm.$nextTick()

      // Need to re-reference vm after the component updates
      vm = wrapper.vm as any
      expect(vm.formData.name).toBe('')
      expect(vm.formData.description).toBe('')
      expect(vm.formData.notes).toBe('')
    })
  })
})
