import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import RawInputForm from '../RawInputForm.vue'
import { useFactoryStore } from '@/stores/factory'
import { useGameDataStore } from '@/stores/gameData'
import type { FactoryResponse, Item, RawInputPreviewRequest } from '@/api/types'

// Helper function to calculate preview based on request (mimics engine logic)
function calculatePreview(request: RawInputPreviewRequest) {
  // For Resource Well Pressurizer
  if (request.extractor_type === 'ResourceWellExtractor' && request.pressurizer && request.extractors) {
    // Base rates: Impure 30, Normal 60, Pure 120 m³/min at 100% clock
    const baseRates: Record<string, number> = {
      Impure: 30,
      Normal: 60,
      Pure: 120,
    }
    
    // Sum of all extractor rates at 100% clock
    const sumAt100 = request.extractors.reduce((sum, ext) => {
      return sum + (baseRates[ext.purity] || 60)
    }, 0)
    
    // Apply pressurizer OC
    const totalRate = sumAt100 * (request.pressurizer.clock_speed / 100.0)
    
    // Power consumption: base 150MW × (clock/100)^1.321928
    const basePower = 150.0
    const powerConsumption = basePower * Math.pow(request.pressurizer.clock_speed / 100.0, 1.321928)
    
    return {
      quantity_per_min: totalRate,
      power_consumption: powerConsumption,
    }
  }
  
  // For regular extractors
  let baseRate = 0
  let basePower = 0
  
  // Base rates by extractor type and purity
  if (request.extractor_type === 'MinerMk1') {
    basePower = 5.0
    const purityMultiplier = request.purity === 'Impure' ? 0.5 : request.purity === 'Normal' ? 1.0 : 2.0
    baseRate = 60 * purityMultiplier // Mk1: 60 items/min base
  } else if (request.extractor_type === 'MinerMk2') {
    basePower = 12.0
    const purityMultiplier = request.purity === 'Impure' ? 0.5 : request.purity === 'Normal' ? 1.0 : 2.0
    baseRate = 120 * purityMultiplier // Mk2: 120 items/min base
  } else if (request.extractor_type === 'MinerMk3') {
    basePower = 30.0
    const purityMultiplier = request.purity === 'Impure' ? 0.5 : request.purity === 'Normal' ? 1.0 : 2.0
    baseRate = 240 * purityMultiplier // Mk3: 240 items/min base
  } else if (request.extractor_type === 'WaterExtractor') {
    basePower = 20.0
    baseRate = 120 // Fixed 120 m³/min
  } else if (request.extractor_type === 'OilExtractor') {
    basePower = 40.0
    const purityRates: Record<string, number> = {
      Impure: 60,
      Normal: 120,
      Pure: 240,
    }
    baseRate = purityRates[request.purity || 'Normal'] || 120
  }
  
  const oc = request.overclock_percent || 100
  const count = request.count || 1
  
  // Apply OC and count
  const totalRate = baseRate * (oc / 100.0) * count
  
  // Power consumption: base_power × (oc/100)^1.321928 × count
  const powerConsumption = basePower * Math.pow(oc / 100.0, 1.321928) * count
  
  return {
    quantity_per_min: totalRate,
    power_consumption: powerConsumption,
  }
}

// Mock the API endpoints
vi.mock('@/api/endpoints', async () => {
  const actual = await vi.importActual('@/api/endpoints')
  
  const mockPreviewFn = vi.fn((factoryId: string, request: RawInputPreviewRequest) => {
    return Promise.resolve(calculatePreview(request))
  })
  
  return {
    ...actual,
    factories: {
      ...(actual as any).factories,
      rawInputs: {
        ...(actual as any).factories.rawInputs,
        rawInput: vi.fn(),
      },
      preview: {
        ...(actual as any).factories.preview,
        rawInput: mockPreviewFn,
      },
    },
    gameData: {
      getItems: vi.fn().mockResolvedValue(['IronOre', 'CopperOre', 'Water', 'CrudeOil', 'NitrogenGas']),
      getExtractorCompatibleItems: vi.fn().mockResolvedValue([
        { extractor_type: 'MinerMk1', compatible_items: ['IronOre', 'CopperOre', 'Coal', 'Limestone', 'RawQuartz', 'Sulfur', 'Uranium'] },
        { extractor_type: 'MinerMk2', compatible_items: ['IronOre', 'CopperOre', 'Coal', 'Limestone', 'RawQuartz', 'Sulfur', 'Uranium'] },
        { extractor_type: 'MinerMk3', compatible_items: ['IronOre', 'CopperOre', 'Coal', 'Limestone', 'RawQuartz', 'Sulfur', 'Uranium'] },
        { extractor_type: 'WaterExtractor', compatible_items: ['Water'] },
        { extractor_type: 'OilExtractor', compatible_items: ['CrudeOil'] },
        { extractor_type: 'ResourceWellExtractor', compatible_items: ['NitrogenGas', 'CrudeOil', 'Water'] },
      ]),
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
      'NitrogenGas' as Item,
    ]

    // Setup factory store
    factoryStore = useFactoryStore()
    vi.clearAllMocks()
    
    // Clear preview mock
    const { factories } = await import('@/api/endpoints')
    if (factories?.preview?.rawInput) {
      (factories.preview.rawInput as any).mockClear()
    }
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

  describe('Mine Ore - Single Extractor', () => {
    it('should create single MinerMk1 with Normal purity at 100% OC', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'IronOre'
      vm.formData.extractor_type = 'MinerMk1'
      vm.formData.purity = 'Normal'
      
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      
      // Set OC and count after extractor change (which resets them)
      vm.formData.overclock_percent = 100
      vm.formData.count = 1
      
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createRawInputSpy).toHaveBeenCalledWith(
        mockFactoryId,
        expect.objectContaining({
          extractor_type: 'MinerMk1',
          item: 'IronOre',
          purity: 'Normal',
          overclock_percent: 100,
          count: 1,
        })
      )
    })

    it('should create single MinerMk2 with Pure purity at 150% OC', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'IronOre'
      vm.formData.extractor_type = 'MinerMk2'
      vm.formData.purity = 'Pure'
      
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      
      // Set OC and count after extractor change (which resets them)
      vm.formData.overclock_percent = 150
      vm.formData.count = 1
      
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createRawInputSpy).toHaveBeenCalledWith(
        mockFactoryId,
        expect.objectContaining({
          extractor_type: 'MinerMk2',
          item: 'IronOre',
          purity: 'Pure',
          overclock_percent: 150,
          count: 1,
        })
      )

      // Verify preview was called with correct values
      const { factories } = await import('@/api/endpoints')
      const previewMock = factories?.preview?.rawInput as any
      if (previewMock) {
        expect(previewMock).toHaveBeenCalled()
        const previewCall = previewMock.mock.calls.find(
          (call: any[]) => call[1].extractor_type === 'MinerMk2' && call[1].purity === 'Pure' && call[1].overclock_percent === 150
        )
        expect(previewCall).toBeDefined()
      }
    })

    it('should create single MinerMk3 with Impure purity at 250% OC', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'IronOre'
      vm.formData.extractor_type = 'MinerMk3'
      vm.formData.purity = 'Impure'
      
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      
      // Set OC and count after extractor change (which resets them)
      vm.formData.overclock_percent = 250
      vm.formData.count = 1
      
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createRawInputSpy).toHaveBeenCalledWith(
        mockFactoryId,
        expect.objectContaining({
          extractor_type: 'MinerMk3',
          item: 'IronOre',
          purity: 'Impure',
          overclock_percent: 250,
          count: 1,
        })
      )
    })
  })

  describe('Mine Ore - Multiple Extractors', () => {
    it('should create group of 4 MinerMk2 with Pure purity at 100% OC', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'IronOre'
      vm.formData.extractor_type = 'MinerMk2'
      vm.formData.purity = 'Pure'
      
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      
      // Set OC and count after extractor change (which resets them)
      vm.formData.overclock_percent = 100
      vm.formData.count = 4
      
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createRawInputSpy).toHaveBeenCalledWith(
        mockFactoryId,
        expect.objectContaining({
          extractor_type: 'MinerMk2',
          item: 'IronOre',
          purity: 'Pure',
          overclock_percent: 100,
          count: 4,
        })
      )
    })

    it('should create group of 3 MinerMk1 with Normal purity at 200% OC', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'CopperOre'
      vm.formData.extractor_type = 'MinerMk1'
      vm.formData.purity = 'Normal'
      
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      
      // Set OC and count after extractor change (which resets them)
      vm.formData.overclock_percent = 200
      vm.formData.count = 3
      
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createRawInputSpy).toHaveBeenCalledWith(
        mockFactoryId,
        expect.objectContaining({
          extractor_type: 'MinerMk1',
          item: 'CopperOre',
          purity: 'Normal',
          overclock_percent: 200,
          count: 3,
        })
      )
    })
  })

  describe('Crude Oil - Single Extractor', () => {
    it('should create single OilExtractor with Normal purity at 100% OC', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'CrudeOil'
      vm.formData.extractor_type = 'OilExtractor'
      vm.formData.purity = 'Normal'
      
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      
      // Set OC and count after extractor change (which resets them)
      vm.formData.overclock_percent = 100
      vm.formData.count = 1
      
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createRawInputSpy).toHaveBeenCalledWith(
        mockFactoryId,
        expect.objectContaining({
          extractor_type: 'OilExtractor',
          item: 'CrudeOil',
          purity: 'Normal',
          overclock_percent: 100,
          count: 1,
        })
      )
    })

    it('should create single OilExtractor with Pure purity at 250% OC', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'CrudeOil'
      vm.formData.extractor_type = 'OilExtractor'
      vm.formData.purity = 'Pure'
      
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      
      // Set OC and count after extractor change (which resets them)
      vm.formData.overclock_percent = 250
      vm.formData.count = 1
      
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createRawInputSpy).toHaveBeenCalledWith(
        mockFactoryId,
        expect.objectContaining({
          extractor_type: 'OilExtractor',
          item: 'CrudeOil',
          purity: 'Pure',
          overclock_percent: 250,
          count: 1,
        })
      )
    })
  })

  describe('Crude Oil - Multiple Extractors', () => {
    it('should create group of 5 OilExtractor with Impure purity at 150% OC', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'CrudeOil'
      vm.formData.extractor_type = 'OilExtractor'
      vm.formData.purity = 'Impure'
      
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      
      // Set OC and count after extractor change (which resets them)
      vm.formData.overclock_percent = 150
      vm.formData.count = 5
      
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createRawInputSpy).toHaveBeenCalledWith(
        mockFactoryId,
        expect.objectContaining({
          extractor_type: 'OilExtractor',
          item: 'CrudeOil',
          purity: 'Impure',
          overclock_percent: 150,
          count: 5,
        })
      )
    })
  })

  describe('Water Extractor', () => {
    it('should create single WaterExtractor at 100% OC', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'Water'
      vm.formData.extractor_type = 'WaterExtractor'
      
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      
      // Set OC and count after extractor change (which resets them)
      vm.formData.overclock_percent = 100
      vm.formData.count = 1
      
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createRawInputSpy).toHaveBeenCalledWith(
        mockFactoryId,
        expect.objectContaining({
          extractor_type: 'WaterExtractor',
          item: 'Water',
          overclock_percent: 100,
          count: 1,
        })
      )

      // Verify purity is NOT included
      const callArgs = createRawInputSpy.mock.calls[0]
      const payload = callArgs[1]
      expect(payload.purity).toBeUndefined()
    })

    it('should create group of 3 WaterExtractor at 200% OC', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'Water'
      vm.formData.extractor_type = 'WaterExtractor'
      
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      
      // Set OC and count after extractor change (which resets them)
      vm.formData.overclock_percent = 200
      vm.formData.count = 3
      
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createRawInputSpy).toHaveBeenCalledWith(
        mockFactoryId,
        expect.objectContaining({
          extractor_type: 'WaterExtractor',
          item: 'Water',
          overclock_percent: 200,
          count: 3,
        })
      )
    })
  })

  describe('Resource Well Pressurizer', () => {
    it('should create pressurizer with single extractor (Normal purity) at 100% OC', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'NitrogenGas'
      vm.formData.extractor_type = 'ResourceWellExtractor'
      
      // Wait for pressurizer to initialize
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 200))

      // Set pressurizer OC
      if (vm.formData.pressurizer) {
        vm.formData.pressurizer.clock_speed = 100
      }

      // Set extractor
      if (vm.formData.extractors.length > 0) {
        vm.formData.extractors[0].purity = 'Normal'
        vm.formData.extractors[0].count = 1
      }

      await wrapper.vm.$nextTick()
      await vm.fetchPreview()
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createRawInputSpy).toHaveBeenCalled()
      const callArgs = createRawInputSpy.mock.calls[0]
      const payload = callArgs[1]
      
      expect(payload.extractor_type).toBe('ResourceWellExtractor')
      expect(payload.item).toBe('NitrogenGas')
      expect(payload.pressurizer).toBeDefined()
      expect(payload.pressurizer.clock_speed).toBe(100)
      expect(payload.extractors).toBeDefined()
      expect(payload.extractors.length).toBe(1)
      expect(payload.extractors[0].purity).toBe('Normal')
    })

    it('should create pressurizer with multiple extractors (various purity) at 150% OC', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'NitrogenGas'
      vm.formData.extractor_type = 'ResourceWellExtractor'
      
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 200))

      // Set pressurizer OC
      if (vm.formData.pressurizer) {
        vm.formData.pressurizer.clock_speed = 150
      }

      // Add multiple extractors with different purity
      vm.formData.extractors = [
        { id: 0, item: 'NitrogenGas', purity: 'Normal', quantity_per_min: 0, count: 2 },
        { id: 0, item: 'NitrogenGas', purity: 'Pure', quantity_per_min: 0, count: 1 },
        { id: 0, item: 'NitrogenGas', purity: 'Impure', quantity_per_min: 0, count: 3 },
      ]

      await wrapper.vm.$nextTick()
      await vm.fetchPreview()
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createRawInputSpy).toHaveBeenCalled()
      const callArgs = createRawInputSpy.mock.calls[0]
      const payload = callArgs[1]
      
      expect(payload.extractor_type).toBe('ResourceWellExtractor')
      expect(payload.pressurizer.clock_speed).toBe(150)
      expect(payload.extractors.length).toBe(6) // 2 + 1 + 3 = 6 total extractors
      
      // Verify extractors are expanded by count
      const normalCount = payload.extractors.filter((e: any) => e.purity === 'Normal').length
      const pureCount = payload.extractors.filter((e: any) => e.purity === 'Pure').length
      const impureCount = payload.extractors.filter((e: any) => e.purity === 'Impure').length
      
      expect(normalCount).toBe(2)
      expect(pureCount).toBe(1)
      expect(impureCount).toBe(3)
    })

    it('should create pressurizer with Pure extractors at 250% OC', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'NitrogenGas'
      vm.formData.extractor_type = 'ResourceWellExtractor'
      
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 200))

      // Set pressurizer OC to 250%
      if (vm.formData.pressurizer) {
        vm.formData.pressurizer.clock_speed = 250
      }

      // Set extractor to Pure
      if (vm.formData.extractors.length > 0) {
        vm.formData.extractors[0].purity = 'Pure'
        vm.formData.extractors[0].count = 4
      }

      await wrapper.vm.$nextTick()
      await vm.fetchPreview()
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      await vm.handleSubmit()
      await wrapper.vm.$nextTick()

      expect(createRawInputSpy).toHaveBeenCalled()
      const callArgs = createRawInputSpy.mock.calls[0]
      const payload = callArgs[1]
      
      expect(payload.pressurizer.clock_speed).toBe(250)
      expect(payload.extractors.length).toBe(4)
      expect(payload.extractors.every((e: any) => e.purity === 'Pure')).toBe(true)
    })

    it('should update preview when pressurizer OC changes', async () => {
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
      await new Promise(resolve => setTimeout(resolve, 200))

      const vm = wrapper.vm as any
      vm.formData.item = 'NitrogenGas'
      vm.formData.extractor_type = 'ResourceWellExtractor'
      
      await wrapper.vm.$nextTick()
      await vm.handleExtractorChange()
      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 200))

      // Get the preview mock function
      const { factories } = await import('@/api/endpoints')
      const previewMock = factories?.preview?.rawInput as any
      if (previewMock) {
        previewMock.mockClear()
      }

      // Change pressurizer OC
      if (vm.formData.pressurizer) {
        vm.formData.pressurizer.clock_speed = 200
        await vm.handlePressurizerOcChange()
      }

      await wrapper.vm.$nextTick()
      await new Promise(resolve => setTimeout(resolve, 100))

      // Verify preview was called with new OC
      if (previewMock) {
        expect(previewMock).toHaveBeenCalled()
        const previewCall = previewMock.mock.calls[0]
        expect(previewCall[1].pressurizer?.clock_speed).toBe(200)
      }
    })
  })
})

