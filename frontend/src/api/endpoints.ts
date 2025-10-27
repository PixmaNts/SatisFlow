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
  UpdateLogisticsRequest,
  CreateProductionLineRequest,
  UpdateProductionLineRequest,
  CreateRawInputRequest,
  UpdateRawInputRequest,
  CreatePowerGeneratorRequest,
  UpdatePowerGeneratorRequest,
  SaveResponse,
  LoadRequest,
  LoadResponse,
  ResetResponse,
  BlueprintExportResponse,
  BlueprintImportRequest,
  BlueprintImportResponse,
  BlueprintMetadata,
  BlueprintTemplateResponse,
  CreateBlueprintTemplateRequest,
  UpdateBlueprintTemplateRequest,
  ImportTemplateRequest,
  ExportTemplateResponse,
  TemplateMetadata,
  CreateFromTemplateRequest,
  CreateFromTemplateResponse,
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
  getById: async (id: string): Promise<FactoryResponse> => {
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
  update: async (id: string, factoryData: UpdateFactoryRequest): Promise<FactoryResponse> => {
    return api.put<FactoryResponse>(`/factories/${id}`, factoryData);
  },

  /**
   * Delete a factory
   * @param id - The factory ID
   * @returns Promise resolving when deletion is complete
   */
  delete: async (id: string): Promise<void> => {
    return api.delete<void>(`/factories/${id}`);
  },

  /**
   * Production line sub-resource endpoints
   */
  productionLines: {
    create: async (
      factoryId: string,
      line: CreateProductionLineRequest
    ): Promise<FactoryResponse> => {
      return api.post<FactoryResponse>(`/factories/${factoryId}/production-lines`, line);
    },
    update: async (
      factoryId: string,
      lineId: string,
      line: UpdateProductionLineRequest
    ): Promise<FactoryResponse> => {
      return api.put<FactoryResponse>(
        `/factories/${factoryId}/production-lines/${lineId}`,
        line
      );
    },
    delete: async (factoryId: string, lineId: string): Promise<FactoryResponse> => {
      return api.delete<FactoryResponse>(
        `/factories/${factoryId}/production-lines/${lineId}`
      );
    },
  },

  /**
   * Raw input sub-resource endpoints
   */
  rawInputs: {
    create: async (
      factoryId: string,
      rawInput: CreateRawInputRequest
    ): Promise<FactoryResponse> => {
      return api.post<FactoryResponse>(`/factories/${factoryId}/raw-inputs`, rawInput);
    },
    update: async (
      factoryId: string,
      rawInputId: string,
      rawInput: UpdateRawInputRequest
    ): Promise<FactoryResponse> => {
      return api.put<FactoryResponse>(
        `/factories/${factoryId}/raw-inputs/${rawInputId}`,
        rawInput
      );
    },
    delete: async (factoryId: string, rawInputId: string): Promise<FactoryResponse> => {
      return api.delete<FactoryResponse>(
        `/factories/${factoryId}/raw-inputs/${rawInputId}`
      );
    },
  },

  /**
   * Power generator sub-resource endpoints
   */
  powerGenerators: {
    create: async (
      factoryId: string,
      generator: CreatePowerGeneratorRequest
    ): Promise<FactoryResponse> => {
      return api.post<FactoryResponse>(
        `/factories/${factoryId}/power-generators`,
        generator
      );
    },
    update: async (
      factoryId: string,
      generatorId: string,
      generator: UpdatePowerGeneratorRequest
    ): Promise<FactoryResponse> => {
      return api.put<FactoryResponse>(
        `/factories/${factoryId}/power-generators/${generatorId}`,
        generator
      );
    },
    delete: async (
      factoryId: string,
      generatorId: string
    ): Promise<FactoryResponse> => {
      return api.delete<FactoryResponse>(
        `/factories/${factoryId}/power-generators/${generatorId}`
      );
    },
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
  getById: async (id: string): Promise<LogisticsResponse> => {
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
   * Update an existing logistics line
   * @param id - The logistics line ID
   * @param logisticsData - The logistics line update data
   * @returns Promise resolving to the updated logistics line
   */
  update: async (
    id: string,
    logisticsData: UpdateLogisticsRequest
  ): Promise<LogisticsResponse> => {
    return api.put<LogisticsResponse>(`/logistics/${id}`, logisticsData);
  },

  /**
   * Delete a logistics line
   * @param id - The logistics line ID
   * @returns Promise resolving when deletion is complete
   */
  delete: async (id: string): Promise<void> => {
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
 * Blueprint API endpoints
 * Provides import/export functionality for production line blueprints
 */
export const blueprints = {
  /**
   * Export a production line blueprint as JSON
   * @param factoryId - The factory ID
   * @param lineId - The production line ID
   * @returns Promise resolving to blueprint JSON and metadata
   */
  export: async (
    factoryId: string,
    lineId: string
  ): Promise<BlueprintExportResponse> => {
    return api.get<BlueprintExportResponse>(
      `/factories/${factoryId}/production-lines/${lineId}/export`
    );
  },

  /**
   * Import a blueprint JSON into a factory
   * @param factoryId - The factory ID to import into
   * @param blueprint - The blueprint import request data
   * @returns Promise resolving to import response
   */
  import: async (
    factoryId: string,
    blueprint: BlueprintImportRequest
  ): Promise<BlueprintImportResponse> => {
    return api.post<BlueprintImportResponse>(
      `/factories/${factoryId}/production-lines/import`,
      blueprint
    );
  },

  /**
   * Preview a blueprint JSON without importing it
   * Returns calculated metadata (power, machines, items) from the backend
   * @param blueprintJson - The blueprint JSON string to preview
   * @returns Promise resolving to blueprint metadata
   */
  preview: async (blueprintJson: string): Promise<BlueprintMetadata> => {
    const request: BlueprintImportRequest = {
      blueprint_json: blueprintJson,
    };
    return api.post<BlueprintMetadata>('/blueprints/preview', request);
  },
};

/**
 * Save/Load API endpoints
 * Provides save and load functionality for engine state
 */
export const saveLoad = {
  /**
   * Save the current engine state
   * @returns Promise resolving to save data and summary
   */
  save: async (): Promise<SaveResponse> => {
    return api.get<SaveResponse>('/save');
  },

  /**
   * Load engine state from save data
   * @param saveData - The save data JSON string
   * @returns Promise resolving to load response with summary
   */
  load: async (saveData: string): Promise<LoadResponse> => {
    const request: LoadRequest = { save_data: saveData };
    return api.post<LoadResponse>('/load', request);
  },

  /**
   * Reset the engine to an empty state (clears all factories and logistics)
   * @returns Promise resolving to reset confirmation message
   */
  reset: async (): Promise<ResetResponse> => {
    return api.post<ResetResponse>('/reset', {});
  },
};

/**
 * Blueprint Templates API endpoints
 * Provides CRUD operations for blueprint template library
 */
export const blueprintTemplates = {
  /**
   * Get all blueprint templates from library
   * @returns Promise resolving to array of blueprint templates
   */
  getAll: async (): Promise<BlueprintTemplateResponse[]> => {
    return api.get<BlueprintTemplateResponse[]>('/blueprints/templates');
  },

  /**
   * Get a specific blueprint template by ID
   * @param id - The template ID
   * @returns Promise resolving to blueprint template data
   */
  getById: async (id: string): Promise<BlueprintTemplateResponse> => {
    return api.get<BlueprintTemplateResponse>(`/blueprints/templates/${id}`);
  },

  /**
   * Create a new blueprint template
   * @param templateData - The template creation data
   * @returns Promise resolving to created template
   */
  create: async (templateData: CreateBlueprintTemplateRequest): Promise<BlueprintTemplateResponse> => {
    return api.post<BlueprintTemplateResponse>('/blueprints/templates', templateData);
  },

  /**
   * Update an existing template (creates new version)
   * @param id - The template ID
   * @param templateData - The template update data
   * @returns Promise resolving to new template version
   */
  update: async (
    id: string,
    templateData: UpdateBlueprintTemplateRequest
  ): Promise<BlueprintTemplateResponse> => {
    return api.put<BlueprintTemplateResponse>(`/blueprints/templates/${id}`, templateData);
  },

  /**
   * Delete a template from library
   * @param id - The template ID
   * @returns Promise resolving when deletion is complete
   */
  delete: async (id: string): Promise<void> => {
    return api.delete<void>(`/blueprints/templates/${id}`);
  },

  /**
   * Import a blueprint JSON to library
   * @param importData - The import request data
   * @returns Promise resolving to imported template
   */
  importToLibrary: async (importData: ImportTemplateRequest): Promise<BlueprintTemplateResponse> => {
    return api.post<BlueprintTemplateResponse>('/blueprints/templates/import', importData);
  },

  /**
   * Export a template as JSON
   * @param id - The template ID
   * @returns Promise resolving to export data with metadata
   */
  export: async (id: string): Promise<ExportTemplateResponse> => {
    return api.get<ExportTemplateResponse>(`/blueprints/templates/${id}/export`);
  },

  /**
   * Create an instance from template in a factory
   * @param factoryId - The factory ID
   * @param templateId - The template ID
   * @param requestData - The creation request data
   * @returns Promise resolving to creation response
   */
  createInstanceInFactory: async (
    factoryId: string,
    templateId: string,
    requestData: CreateFromTemplateRequest
  ): Promise<CreateFromTemplateResponse> => {
    return api.post<CreateFromTemplateResponse>(
      `/factories/${factoryId}/production-lines/from-template/${templateId}`,
      requestData
    );
  },

  /**
   * Preview a blueprint JSON without importing to library
   * Returns calculated metadata (power, machines, items) from the backend
   * @param blueprintJson - The blueprint JSON string to preview
   * @returns Promise resolving to blueprint metadata
   */
  preview: async (blueprintJson: string): Promise<BlueprintMetadata> => {
    // Reuse the same preview endpoint as blueprints
    return blueprints.preview(blueprintJson);
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
  blueprints,
  blueprintTemplates,
  saveLoad,
};

/**
 * Default export with all endpoints
 */
export default endpoints;
