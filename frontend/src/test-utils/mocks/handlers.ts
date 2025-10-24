import { http, HttpResponse } from 'msw'
import type {
  FactoryResponse,
  LogisticsResponse,
  DashboardSummary,
  ItemBalance,
  PowerStats,
  CreateFactoryRequest,
  CreateLogisticsRequest
} from '@/api/types'

// Mock data
const mockFactories: FactoryResponse[] = [
  {
    id: 'factory-1',
    name: 'Iron Processing',
    description: 'Processes iron ore into plates',
    notes: null,
    production_lines: [],
    raw_inputs: [],
    power_generators: [],
    items: [],
    total_power_consumption: 80.0,
    total_power_generation: 100.0,
    power_balance: 20.0,
  },
  {
    id: 'factory-2',
    name: 'Copper Processing',
    description: 'Processes copper ore into wires',
    notes: null,
    production_lines: [],
    raw_inputs: [],
    power_generators: [],
    items: [],
    total_power_consumption: 70.5,
    total_power_generation: 100.0,
    power_balance: 29.5,
  },
]

const mockLogistics: LogisticsResponse[] = [
  {
    id: 'logistics-1',
    from_factory: 'factory-1',
    to_factory: 'factory-2',
    transport_type: 'Truck',
    transport_id: 'TRUCK-001',
    transport_name: 'Iron Transport',
    transport_details: 'Truck transporting iron plates',
    items: [{ item: 'IronPlate', quantity_per_min: 60 }],
    total_quantity_per_min: 60,
  },
]

const mockDashboardSummary: DashboardSummary = {
  total_factories: 2,
  total_production_lines: 5,
  total_logistics_lines: 3,
  total_power_consumption: 150.5,
  total_power_generation: 200.0,
  net_power: 49.5,
}

const mockItemBalance: ItemBalance[] = [
  {
    item: 'IronPlate',
    balance: 30.0,
    state: 'overflow',
  },
  {
    item: 'Wire',
    balance: -20.0,
    state: 'underflow',
  },
]

const mockPowerStats: PowerStats = {
  total_generation: 200.0,
  total_consumption: 150.5,
  power_balance: 49.5,
  has_surplus: true,
  has_deficit: false,
  is_balanced: false,
  factory_stats: [
    {
      factory_id: 'factory-1',
      factory_name: 'Iron Processing',
      generation: 100.0,
      consumption: 80.0,
      balance: 20.0,
      generator_count: 2,
      generator_types: ['Coal', 'Biomass'],
    },
    {
      factory_id: 'factory-2',
      factory_name: 'Copper Processing',
      generation: 100.0,
      consumption: 70.5,
      balance: 29.5,
      generator_count: 1,
      generator_types: ['Fuel'],
    },
  ],
}

// API handlers
export const handlers = [
  // Factory endpoints
  http.get('/api/factories', () => {
    return HttpResponse.json(mockFactories)
  }),

  http.get('/api/factories/:id', ({ params }: { params: { id: string } }) => {
    const { id } = params
    const factory = mockFactories.find(f => f.id === id)
    if (!factory) {
      return HttpResponse.json({ error: 'Factory not found' }, { status: 404 })
    }
    return HttpResponse.json(factory)
  }),

  http.post('/api/factories', async ({ request }) => {
    const newFactory = await request.json() as CreateFactoryRequest
    const sanitizedNotes =
      typeof newFactory.notes === 'string' && newFactory.notes.trim().length === 0
        ? null
        : newFactory.notes ?? null
    const factoryId = `factory-${mockFactories.length + 1}`
    const factory: FactoryResponse = {
      id: factoryId,
      name: newFactory.name || 'New Factory',
      description: newFactory.description || null,
      notes: sanitizedNotes,
      production_lines: [],
      raw_inputs: [],
      power_generators: [],
      items: [],
      total_power_consumption: 0,
      total_power_generation: 0,
      power_balance: 0,
    }
    mockFactories.push(factory)
    return HttpResponse.json(factory, { status: 201 })
  }),

  http.put('/api/factories/:id', async ({ params, request }) => {
    const idParam = params?.id
    if (!idParam) {
      return HttpResponse.json({ error: 'Factory id is required' }, { status: 400 })
    }
    const factoryId = idParam
    const updates = await request.json() as Partial<FactoryResponse>
    const index = mockFactories.findIndex(f => f.id === factoryId)
    if (index === -1) {
      return HttpResponse.json({ error: 'Factory not found' }, { status: 404 })
    }

    // Simple update - just merge the updates
    const currentFactory = mockFactories[index]!
    const sanitizedNotes =
      typeof updates.notes === 'string'
        ? (updates.notes.trim().length === 0 ? null : updates.notes)
        : updates.notes ?? currentFactory.notes

    const updatedFactory: FactoryResponse = {
      id: currentFactory.id,
      name: updates.name ?? currentFactory.name,
      description: updates.description ?? currentFactory.description,
      notes: sanitizedNotes,
      production_lines: updates.production_lines ?? currentFactory.production_lines,
      raw_inputs: updates.raw_inputs ?? currentFactory.raw_inputs,
      power_generators: updates.power_generators ?? currentFactory.power_generators,
      items: updates.items ?? currentFactory.items,
      total_power_consumption: updates.total_power_consumption ?? currentFactory.total_power_consumption,
      total_power_generation: updates.total_power_generation ?? currentFactory.total_power_generation,
      power_balance: updates.power_balance ?? currentFactory.power_balance,
    }

    mockFactories[index] = updatedFactory
    return HttpResponse.json(updatedFactory)
  }),

  http.delete('/api/factories/:id', ({ params }) => {
    const idParam = params?.id
    if (!idParam) {
      return HttpResponse.json({ error: 'Factory id is required' }, { status: 400 })
    }
    const factoryId = idParam
    const index = mockFactories.findIndex(f => f.id === factoryId)
    if (index === -1) {
      return HttpResponse.json({ error: 'Factory not found' }, { status: 404 })
    }
    mockFactories.splice(index, 1)
    return HttpResponse.json({ success: true })
  }),

  // Logistics endpoints
  http.get('/api/logistics', () => {
    return HttpResponse.json(mockLogistics)
  }),

  http.get('/api/logistics/:id', ({ params }) => {
    const idParam = params?.id
    if (!idParam) {
      return HttpResponse.json({ error: 'Logistics id is required' }, { status: 400 })
    }
    const logistics = mockLogistics.find(l => l.id === idParam)
    if (!logistics) {
      return HttpResponse.json({ error: 'Logistics line not found' }, { status: 404 })
    }
    return HttpResponse.json(logistics)
  }),

  http.post('/api/logistics', async ({ request }) => {
    const payload = await request.json() as CreateLogisticsRequest
    const newIndex = mockLogistics.length + 1
    const newId = `logistics-${newIndex}`

    const buildResponse = (response: Omit<LogisticsResponse, 'id'>): LogisticsResponse => ({
      id: newId,
      ...response,
    })

    const base = {
      from_factory: payload.from_factory,
      to_factory: payload.to_factory,
    }

    let response: LogisticsResponse

    switch (payload.transport_type) {
      case 'Truck': {
        const quantity = payload.quantity_per_min ?? 0
        response = buildResponse({
          ...base,
          transport_type: 'Truck',
          transport_id: payload.truck_id || `Truck-${newIndex}`,
          transport_name: null,
          transport_details: JSON.stringify({
            item: payload.item,
            quantity_per_min: payload.quantity_per_min,
            truck_id: payload.truck_id,
          }),
          items: quantity > 0 && payload.item
            ? [{ item: payload.item, quantity_per_min: quantity }]
            : [],
          total_quantity_per_min: quantity,
        })
        break
      }
      case 'Drone': {
        const quantity = payload.quantity_per_min ?? 0
        response = buildResponse({
          ...base,
          transport_type: 'Drone',
          transport_id: payload.drone_id || `Drone-${newIndex}`,
          transport_name: null,
          transport_details: JSON.stringify({
            item: payload.item,
            quantity_per_min: payload.quantity_per_min,
            drone_id: payload.drone_id,
          }),
          items: quantity > 0 && payload.item
            ? [{ item: payload.item, quantity_per_min: quantity }]
            : [],
          total_quantity_per_min: quantity,
        })
        break
      }
      case 'Bus': {
        const conveyors = payload.conveyors ?? []
        const pipelines = payload.pipelines ?? []
        const flows = [
          ...conveyors.map(entry => ({
            item: entry.item,
            quantity_per_min: entry.quantity_per_min,
          })),
          ...pipelines.map(entry => ({
            item: entry.item,
            quantity_per_min: entry.quantity_per_min,
          })),
        ].filter(flow => flow.item && flow.quantity_per_min > 0)

        const total = flows.reduce((sum, flow) => sum + flow.quantity_per_min, 0)

        response = buildResponse({
          ...base,
          transport_type: 'Bus',
          transport_id: `Bus-${newIndex}`,
          transport_name: payload.bus_name || null,
          transport_details: JSON.stringify({
            bus_name: payload.bus_name,
            conveyors: conveyors,
            pipelines: pipelines,
          }),
          items: flows,
          total_quantity_per_min: total,
        })
        break
      }
      case 'Train': {
        const wagons = payload.wagons ?? []
        const flows = wagons
          .map(wagon => ({
            item: wagon.item,
            quantity_per_min: wagon.quantity_per_min,
          }))
          .filter(flow => flow.item && flow.quantity_per_min > 0)

        const total = flows.reduce((sum, flow) => sum + flow.quantity_per_min, 0)

        response = buildResponse({
          ...base,
          transport_type: 'Train',
          transport_id: `Train-${newIndex}`,
          transport_name: payload.train_name || null,
          transport_details: JSON.stringify({
            train_name: payload.train_name,
            wagons,
          }),
          items: flows,
          total_quantity_per_min: total,
        })
        break
      }
      default: {
        return HttpResponse.json({ error: 'Unsupported transport type' }, { status: 400 })
      }
    }

    mockLogistics.push(response)
    return HttpResponse.json(response, { status: 201 })
  }),

  http.delete('/api/logistics/:id', ({ params }: { params: { id: string } }) => {
    const { id } = params
    const index = mockLogistics.findIndex(l => l.id === id)
    if (index === -1) {
      return HttpResponse.json({ error: 'Logistics line not found' }, { status: 404 })
    }
    mockLogistics.splice(index, 1)
    return HttpResponse.json({ success: true })
  }),

  // Dashboard endpoints
  http.get('/api/dashboard/summary', () => {
    return HttpResponse.json(mockDashboardSummary)
  }),

  http.get('/api/dashboard/items', () => {
    return HttpResponse.json(mockItemBalance)
  }),

  http.get('/api/dashboard/power', () => {
    return HttpResponse.json(mockPowerStats)
  }),

  // Game data endpoints
  http.get('/api/game-data/recipes', () => {
    return HttpResponse.json([
      {
        name: 'Iron Ingot',
        machine: 'Smelter',
        inputs: [{ item: 'IronOre', quantity: 30 }],
        outputs: [{ item: 'IronIngot', quantity: 30 }],
      },
      {
        name: 'Iron Plate',
        machine: 'Constructor',
        inputs: [{ item: 'IronIngot', quantity: 30 }],
        outputs: [{ item: 'IronPlate', quantity: 20 }],
      },
    ])
  }),

  http.get('/api/game-data/items', () => {
    return HttpResponse.json([
      { name: 'IronOre', display_name: 'Iron Ore' },
      { name: 'IronIngot', display_name: 'Iron Ingot' },
      { name: 'IronPlate', display_name: 'Iron Plate' },
      { name: 'CopperOre', display_name: 'Copper Ore' },
      { name: 'CopperIngot', display_name: 'Copper Ingot' },
      { name: 'CopperWire', display_name: 'Copper Wire' },
    ])
  }),

  http.get('/api/game-data/machines', () => {
    return HttpResponse.json([
      {
        name: 'Constructor',
        display_name: 'Constructor',
        max_somersloops: 1,
        base_power: 4.0,
      },
      {
        name: 'Smelter',
        display_name: 'Smelter',
        max_somersloops: 1,
        base_power: 4.0,
      },
      {
        name: 'Assembler',
        display_name: 'Assembler',
        max_somersloops: 2,
        base_power: 16.0,
      },
    ])
  }),
]
