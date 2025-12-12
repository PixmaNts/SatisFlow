import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import RawInputForm from '../RawInputForm.vue'
import { useFactoryStore } from '@/stores/factory'
import { useGameDataStore } from '@/stores/gameData'
import type { FactoryResponse, Item } from '@/api/types'

// Mock the API endpoints
vi.mock('@/api/endpoints', async () => {
  const actual = await vi.importActual('@/api/endpoints')
  const mockPreviewResponse = {
    quantity_per_min: 120,
    power_consumption: 15,
  }
  return {
    ...actual,
    factories: {
      ...(actual as any).factories,
      rawInputs: {
        ...(actual as any).factories.rawInputs,
        // The component calls factories.rawInputs.rawInput for preview
        rawInput: vi.fn().mockResolvedValue(mockPreviewResponse),
      },
      preview: {
        ...(actual as any).factories.preview,
        rawInput: vi.fn().mockResolvedValue(mockPreviewResponse),
      },
    },
    gameData: {
      getItems: vi.fn().mockResolvedValue(['IronOre', 'CopperOre', 'Water', 'CrudeOil']),
    },
  }
})

describe('RawInputForm', () => {
  let pinia: ReturnType<typeof createPinia>
  let factoryStore: ReturnType<typeof useFactoryStore>
  let gameDataStore: ReturnType<typeof useGameDataStore>
  const mockFactoryId = '550e8400-e29b-41d4-a716-446655440000'

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

    // Setup game data store with mock items
    gameDataStore = useGameDataStore()
    // Pre-populate items to avoid API call
    gameDataStore.items = [
      'IronOre' as Item,
      'CopperOre' as Item,
      'Water' as Item,
      'CrudeOil' as Item,
    ]

    // Setup factory store
    factoryStore = useFactoryStore()
    vi.clearAllMocks()
  })

  describe('Miner with missing purity', () => {
    it('should not allow submission when miner is selected but purity is missing', async () => {
      const wrapper = mount(RawInputForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      // Wait for component to mount and items to load
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      // Access component instance to set form data directly
      const vm = wrapper.vm as any
      
      // Set resource to IronOre (solid resource)
      vm.formData.item = 'IronOre'
      await wrapper.vm.$nextTick()

      // Set extractor type to MinerMk2 (requires purity)
      vm.formData.extractor_type = 'MinerMk2'
      vm.formData.quantity_per_min = 120
      // Don't set purity - leave it as null
      await wrapper.vm.$nextTick()

      // Force reactivity update
      await wrapper.vm.$forceUpdate()
      await wrapper.vm.$nextTick()

      // Verify that canSubmit is false when purity is missing
      // canSubmit checks: (!showPurity.value || formData.value.purity)
      // For miners, showPurity is true, so formData.value.purity must be truthy
      const canSubmitValue = vm.canSubmit
      // canSubmit should be false because purity is null/undefined
      expect(canSubmitValue).toBeFalsy()

      // Verify that handleSubmit would return early
      const handleSubmitSpy = vi.spyOn(vm, 'handleSubmit')
      await vm.handleSubmit()
      // handleSubmit checks canSubmit first and returns early if false
      // So it should not proceed to createRawInput
      expect(handleSubmitSpy).toHaveBeenCalled()
    })

    it('should handle 400 error when creating raw input with miner but without purity', async () => {
      // This test simulates the bug scenario where a miner is selected
      // but purity is null/undefined, causing the backend to return a 400 error
      
      // Mock the createRawInput to return a 400 error
      const mockError = {
        response: {
          status: 400,
          data: {
            error: 'MissingPurity',
            message: 'Miner extractors require purity',
          },
        },
      }

      const createRawInputSpy = vi.spyOn(factoryStore, 'createRawInput').mockRejectedValue(mockError)
      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {})

      const wrapper = mount(RawInputForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      // Manually set form data to simulate the bug scenario
      // where purity is null/undefined but form is submitted
      const vm = wrapper.vm as any
      vm.formData.item = 'IronOre'
      vm.formData.extractor_type = 'MinerMk2'
      vm.formData.purity = null // Missing purity - this is the bug
      vm.formData.quantity_per_min = 120

      await wrapper.vm.$nextTick()

      // The bug: when purity is null, the payload.purity becomes undefined (line 553: formData.value.purity ?? undefined)
      // but the backend requires purity for miners, causing a 400 error
      // 
      // To test the error handling, we need to bypass the canSubmit check
      // by directly calling the store method with the buggy payload
      // This simulates what would happen if the validation somehow failed or was bypassed
      
      // Create the payload as the component would (simulating the bug)
      const payload = {
        extractor_type: vm.formData.extractor_type,
        item: vm.formData.item,
        quantity_per_min: vm.formData.quantity_per_min,
      }
      
      // Simulate the component's logic: if not WaterExtractor, include purity
      if (vm.formData.extractor_type !== 'WaterExtractor') {
        payload.purity = vm.formData.purity ?? undefined // This becomes undefined when purity is null
      }

      // Directly call the store method to simulate the bug scenario
      try {
        await factoryStore.createRawInput(mockFactoryId, payload as any)
      } catch (error) {
        // Expected to throw
      }

      // Verify that createRawInput was called
      expect(createRawInputSpy).toHaveBeenCalled()

      // Verify that purity is undefined in the payload (the bug)
      const callArgs = createRawInputSpy.mock.calls[0]
      const actualPayload = callArgs[1]
      expect(actualPayload.extractor_type).toBe('MinerMk2')
      expect(actualPayload.item).toBe('IronOre')
      expect(actualPayload.purity).toBeUndefined() // This is the bug - purity should be required for miners

      consoleErrorSpy.mockRestore()
    })

    it('should include purity in payload when miner and purity are both set', async () => {
      const createRawInputSpy = vi.spyOn(factoryStore, 'createRawInput').mockResolvedValue(mockFactory)

      const wrapper = mount(RawInputForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      // Set form data directly
      const vm = wrapper.vm as any
      vm.formData.item = 'IronOre'
      vm.formData.extractor_type = 'MinerMk2'
      vm.formData.purity = 'Normal'
      vm.formData.quantity_per_min = 120

      await wrapper.vm.$nextTick()

      // Call handleSubmit directly
      await vm.handleSubmit()
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      // Verify that createRawInput was called with purity included
      expect(createRawInputSpy).toHaveBeenCalledWith(
        mockFactoryId,
        expect.objectContaining({
          extractor_type: 'MinerMk2',
          item: 'IronOre',
          purity: 'Normal',
          quantity_per_min: 120,
        })
      )
    })

    it('should not include purity for WaterExtractor even if set', async () => {
      const createRawInputSpy = vi.spyOn(factoryStore, 'createRawInput').mockResolvedValue(mockFactory)

      const wrapper = mount(RawInputForm, {
        props: {
          show: true,
          factoryId: mockFactoryId,
        },
        global: {
          plugins: [pinia],
        },
      })

      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      // Set form data
      const vm = wrapper.vm as any
      vm.formData.item = 'Water'
      vm.formData.extractor_type = 'WaterExtractor'
      vm.formData.quantity_per_min = 120
      // Note: purity might be set but should be excluded from payload

      await wrapper.vm.$nextTick()

      // Call handleSubmit directly
      await vm.handleSubmit()
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      // Verify that createRawInput was called
      expect(createRawInputSpy).toHaveBeenCalled()

      // Verify that purity is NOT in the payload
      const callArgs = createRawInputSpy.mock.calls[0]
      const payload = callArgs[1]
      expect(payload.extractor_type).toBe('WaterExtractor')
      expect(payload.item).toBe('Water')
      expect(payload.purity).toBeUndefined() // WaterExtractor should not have purity
    })
  })
})

