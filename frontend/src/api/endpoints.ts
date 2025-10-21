import { api } from './client';
import type {
  FactoryResponse,
  LogisticsResponse,
  DashboardSummary,
  ItemBalance,
  PowerStats,
  RecipeInfo,
  MachineInfo,
  ItemInfo,
  HealthCheckResponse,
  CreateFactoryRequest,
  UpdateFactoryRequest,
  CreateLogisticsRequest,
} from './types';

/**
 * Factory API endpoints
 * Provides CRUD operations for factories
 */
export const factories = {
  /**
   * Get all factories
   * @returns Promise resolving to array of factories
   */
  getAll: async (): Promise<FactoryResponse[]> => {
    return api.get<FactoryResponse[]>('/factories');
  },

  /**
   * Get a specific factory by ID
   * @param id - The factory ID
   * @returns Promise resolving to the factory data
   */
  getById: async (id: number): Promise<FactoryResponse> => {
    return api.get<FactoryResponse>(`/factories/${id}`);
  },

  /**
   * Create a new factory
   * @param factoryData - The factory creation data
   * @returns Promise resolving to the created factory
   */
  create: async (factoryData: CreateFactoryRequest): Promise<FactoryResponse> => {
    return api.post<FactoryResponse>('/factories', factoryData);
  },

  /**
   * Update an existing factory
   * @param id - The factory ID
   * @param factoryData - The factory update data
   * @returns Promise resolving to the updated factory
   */
  update: async (id: number, factoryData: UpdateFactoryRequest): Promise<FactoryResponse> => {
    return api.put<FactoryResponse>(`/factories/${id}`, factoryData);
  },

  /**
   * Delete a factory
   * @param id - The factory ID
   * @returns Promise resolving when deletion is complete
   */
  delete: async (id: number): Promise<void> => {
    return api.delete<void>(`/factories/${id}`);
  },
};

/**
 * Logistics API endpoints
 * Provides CRUD operations for logistics lines
 */
export const logistics = {
  /**
   * Get all logistics lines
   * @returns Promise resolving to array of logistics lines
   */
  getAll: async (): Promise<LogisticsResponse[]> => {
    return api.get<LogisticsResponse[]>('/logistics');
  },

  /**
   * Get a specific logistics line by ID
   * @param id - The logistics line ID
   * @returns Promise resolving to the logistics line data
   */
  getById: async (id: number): Promise<LogisticsResponse> => {
    return api.get<LogisticsResponse>(`/logistics/${id}`);
  },

  /**
   * Create a new logistics line
   * @param logisticsData - The logistics line creation data
   * @returns Promise resolving to the created logistics line
   */
  create: async (logisticsData: CreateLogisticsRequest): Promise<LogisticsResponse> => {
    return api.post<LogisticsResponse>('/logistics', logisticsData);
  },

  /**
   * Delete a logistics line
   * @param id - The logistics line ID
   * @returns Promise resolving when deletion is complete
   */
  delete: async (id: number): Promise<void> => {
    return api.delete<void>(`/logistics/${id}`);
  },
};

/**
 * Dashboard API endpoints
 * Provides aggregated data for the dashboard view
 */
export const dashboard = {
  /**
   * Get dashboard summary statistics
   * @returns Promise resolving to dashboard summary data
   */
  getSummary: async (): Promise<DashboardSummary> => {
    return api.get<DashboardSummary>('/dashboard/summary');
  },

  /**
   * Get item balance data across all factories
   * @returns Promise resolving to item balance data
   */
  getItemBalances: async (): Promise<ItemBalance[]> => {
    return api.get<ItemBalance[]>('/dashboard/items');
  },

  /**
   * Get power statistics across all factories
   * @returns Promise resolving to power statistics
   */
  getPowerStats: async (): Promise<PowerStats> => {
    return api.get<PowerStats>('/dashboard/power');
  },
};

/**
 * Game Data API endpoints
 * Provides static game data (recipes, items, machines)
 */
export const gameData = {
  /**
   * Get all available recipes
   * @returns Promise resolving to array of recipe information
   */
  getRecipes: async (): Promise<RecipeInfo[]> => {
    return api.get<RecipeInfo[]>('/game-data/recipes');
  },

  /**
   * Get all available items
   * @returns Promise resolving to array of item information
   */
  getItems: async (): Promise<ItemInfo[]> => {
    return api.get<ItemInfo[]>('/game-data/items');
  },

  /**
   * Get all machine types with their specifications
   * @returns Promise resolving to array of machine information
   */
  getMachines: async (): Promise<MachineInfo[]> => {
    return api.get<MachineInfo[]>('/game-data/machines');
  },
};

/**
 * System API endpoints
 * Provides system-level operations like health checks
 */
export const system = {
  /**
   * Check API health status
   * @returns Promise resolving to health check response
   */
  healthCheck: async (): Promise<HealthCheckResponse> => {
    return api.get<HealthCheckResponse>('/health');
  },
};

/**
 * Export all endpoint groups for convenient importing
 */
export const endpoints = {
  factories,
  logistics,
  dashboard,
  gameData,
  system,
};

/**
 * Default export with all endpoints
 */
export default endpoints;
