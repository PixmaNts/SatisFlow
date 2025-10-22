// Extended types for logistics forms and UI

import type {
  TransportType,
  Item,
  BusConveyorPayload,
  BusPipelinePayload,
  TrainWagonPayload,
  ConveyorTier,
  PipelineTier,
  WagonCarType,
} from './types'

// Base transport configuration
export interface BaseTransportConfig {
  transport_type: TransportType
}

// Bus transport configuration
export interface BusConfig extends BaseTransportConfig {
  transport_type: 'Bus'
  bus_name?: string
  conveyors: ConveyorConfig[]
  pipelines: PipelineConfig[]
}

export interface ConveyorConfig extends BusConveyorPayload {
  line_id: string
  conveyor_type: ConveyorTier
  item: Item
  quantity_per_min: number
}

export interface PipelineConfig extends BusPipelinePayload {
  pipeline_id: string
  pipeline_type: PipelineTier
  item: Item
  quantity_per_min: number
}

// Train transport configuration
export interface TrainConfig extends BaseTransportConfig {
  transport_type: 'Train'
  train_name?: string
  wagons: WagonConfig[]
}

export interface WagonConfig extends TrainWagonPayload {
  wagon_id: string
  wagon_type: WagonCarType
  item: Item
  quantity_per_min: number
}

// Truck transport configuration
export interface TruckConfig extends BaseTransportConfig {
  transport_type: 'Truck'
  item: Item
  quantity_per_min: number
  truck_id: string
}

// Drone transport configuration
export interface DroneConfig extends BaseTransportConfig {
  transport_type: 'Drone'
  item: Item
  quantity_per_min: number
  drone_id: string
}

// Union type for all transport configurations
export type TransportConfig = BusConfig | TrainConfig | TruckConfig | DroneConfig

// Form data for creating logistics lines
export interface LogisticsFormData {
  from_factory: number
  to_factory: number
  transport_config: TransportConfig
}

// Transport type options for UI
export interface TransportTypeOption {
  value: TransportType
  label: string
  description: string
  icon: string
}

// Conveyor speed constants (items per minute)
export const CONVEYOR_SPEEDS = {
  Mk1: 60,
  Mk2: 120,
  Mk3: 270,
  Mk4: 480,
  Mk5: 780,
  Mk6: 1200,
} as const

// Pipeline capacity constants (m³ per minute)
export const PIPELINE_CAPACITIES = {
  Mk1: 300,
  Mk2: 600,
} as const
