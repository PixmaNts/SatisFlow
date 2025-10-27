import { describe, it, expect, vi, beforeEach } from 'vitest';
import { blueprintTemplates } from '@/api/endpoints';
import { api } from '@/api/client';
import type {
  BlueprintTemplateResponse,
  CreateBlueprintTemplateRequest,
  UpdateBlueprintTemplateRequest,
  ImportTemplateRequest,
  ExportTemplateResponse,
  CreateFromTemplateRequest,
  CreateFromTemplateResponse,
} from '@/api/types';

// Mock the API client
vi.mock('@/api/client', () => ({
  api: {
    get: vi.fn(),
    post: vi.fn(),
    put: vi.fn(),
    delete: vi.fn(),
  },
}));

describe('Blueprint Templates API', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('getAll', () => {
    it('should fetch all blueprint templates', async () => {
      const mockTemplates: BlueprintTemplateResponse[] = [
        {
          id: 'template-1',
          name: 'Test Blueprint',
          description: 'A test blueprint',
          production_lines: [],
          total_machines: 10,
          total_power: 100.5,
          input_items: [['IronOre', 60.0]],
          output_items: [['IronIngot', 60.0]],
        },
      ];

      vi.mocked(api.get).mockResolvedValue(mockTemplates);

      const result = await blueprintTemplates.getAll();

      expect(api.get).toHaveBeenCalledWith('/blueprints/templates');
      expect(result).toEqual(mockTemplates);
    });

    it('should handle API errors', async () => {
      const error = new Error('Network error');
      vi.mocked(api.get).mockRejectedValue(error);

      await expect(blueprintTemplates.getAll()).rejects.toThrow('Network error');
    });
  });

  describe('getById', () => {
    it('should fetch a specific blueprint template', async () => {
      const mockTemplate: BlueprintTemplateResponse = {
        id: 'template-1',
        name: 'Test Blueprint',
        description: 'A test blueprint',
        production_lines: [],
        total_machines: 10,
        total_power: 100.5,
        input_items: [['IronOre', 60.0]],
        output_items: [['IronIngot', 60.0]],
      };

      vi.mocked(api.get).mockResolvedValue(mockTemplate);

      const result = await blueprintTemplates.getById('template-1');

      expect(api.get).toHaveBeenCalledWith('/blueprints/templates/template-1');
      expect(result).toEqual(mockTemplate);
    });

    it('should handle not found errors', async () => {
      const error = new Error('Template not found');
      vi.mocked(api.get).mockRejectedValue(error);

      await expect(blueprintTemplates.getById('invalid-id')).rejects.toThrow('Template not found');
    });
  });

  describe('create', () => {
    it('should create a new blueprint template', async () => {
      const createData: CreateBlueprintTemplateRequest = {
        name: 'New Blueprint',
        description: 'A new blueprint',
        production_lines: [
          {
            name: 'Iron Ingot Production',
            description: 'Basic iron production',
            type: 'recipe',
            recipe: 'IronIngot',
            machine_groups: [
              {
                number_of_machine: 4,
                oc_value: 100.0,
                somersloop: 0,
              },
            ],
          },
        ],
      };

      const mockTemplate: BlueprintTemplateResponse = {
        id: 'new-template-id',
        name: 'New Blueprint',
        description: 'A new blueprint',
        production_lines: [],
        total_machines: 4,
        total_power: 16.0,
        input_items: [['IronOre', 120.0]],
        output_items: [['IronIngot', 120.0]],
      };

      vi.mocked(api.post).mockResolvedValue(mockTemplate);

      const result = await blueprintTemplates.create(createData);

      expect(api.post).toHaveBeenCalledWith('/blueprints/templates', createData);
      expect(result).toEqual(mockTemplate);
    });

    it('should handle validation errors', async () => {
      const invalidData: CreateBlueprintTemplateRequest = {
        name: '',
        production_lines: [],
      };

      const error = new Error('Invalid blueprint data');
      vi.mocked(api.post).mockRejectedValue(error);

      await expect(blueprintTemplates.create(invalidData)).rejects.toThrow('Invalid blueprint data');
    });
  });

  describe('update', () => {
    it('should update a blueprint template (creates new version)', async () => {
      const updateData: UpdateBlueprintTemplateRequest = {
        name: 'Updated Blueprint',
        description: 'Updated description',
        production_lines: [],
      };

      const mockUpdatedTemplate: BlueprintTemplateResponse = {
        id: 'new-version-id',
        name: 'Updated Blueprint',
        description: 'Updated description',
        production_lines: [],
        total_machines: 0,
        total_power: 0.0,
        input_items: [],
        output_items: [],
      };

      vi.mocked(api.put).mockResolvedValue(mockUpdatedTemplate);

      const result = await blueprintTemplates.update('template-1', updateData);

      expect(api.put).toHaveBeenCalledWith('/blueprints/templates/template-1', updateData);
      expect(result).toEqual(mockUpdatedTemplate);
    });

    it('should handle not found errors during update', async () => {
      const updateData: UpdateBlueprintTemplateRequest = {
        name: 'Updated Blueprint',
        production_lines: [],
      };

      const error = new Error('Template not found');
      vi.mocked(api.put).mockRejectedValue(error);

      await expect(blueprintTemplates.update('invalid-id', updateData)).rejects.toThrow('Template not found');
    });
  });

  describe('delete', () => {
    it('should delete a blueprint template', async () => {
      vi.mocked(api.delete).mockResolvedValue(undefined);

      await blueprintTemplates.delete('template-1');

      expect(api.delete).toHaveBeenCalledWith('/blueprints/templates/template-1');
    });

    it('should handle not found errors during delete', async () => {
      const error = new Error('Template not found');
      vi.mocked(api.delete).mockRejectedValue(error);

      await expect(blueprintTemplates.delete('invalid-id')).rejects.toThrow('Template not found');
    });
  });

  describe('importToLibrary', () => {
    it('should import a blueprint to library', async () => {
      const importData: ImportTemplateRequest = {
        blueprint_json: '{"name":"Imported Blueprint","production_lines":[]}',
        name: 'Imported Blueprint',
      };

      const mockTemplate: BlueprintTemplateResponse = {
        id: 'imported-template-id',
        name: 'Imported Blueprint',
        description: null,
        production_lines: [],
        total_machines: 0,
        total_power: 0.0,
        input_items: [],
        output_items: [],
      };

      vi.mocked(api.post).mockResolvedValue(mockTemplate);

      const result = await blueprintTemplates.importToLibrary(importData);

      expect(api.post).toHaveBeenCalledWith('/blueprints/templates/import', importData);
      expect(result).toEqual(mockTemplate);
    });

    it('should handle invalid JSON during import', async () => {
      const invalidImportData: ImportTemplateRequest = {
        blueprint_json: 'invalid json',
      };

      const error = new Error('Invalid blueprint JSON');
      vi.mocked(api.post).mockRejectedValue(error);

      await expect(blueprintTemplates.importToLibrary(invalidImportData)).rejects.toThrow('Invalid blueprint JSON');
    });
  });

  describe('export', () => {
    it('should export a blueprint template', async () => {
      const mockExportResponse: ExportTemplateResponse = {
        blueprint_json: '{"name":"Test Blueprint","production_lines":[]}',
        metadata: {
          name: 'Test Blueprint',
          description: 'A test blueprint',
          total_machines: 10,
          total_power: 100.5,
          input_items: [['IronOre', 60.0]],
          output_items: [['IronIngot', 60.0]],
          exported_at: '2025-10-26T17:00:00Z',
        },
      };

      vi.mocked(api.get).mockResolvedValue(mockExportResponse);

      const result = await blueprintTemplates.export('template-1');

      expect(api.get).toHaveBeenCalledWith('/blueprints/templates/template-1/export');
      expect(result).toEqual(mockExportResponse);
    });

    it('should handle not found errors during export', async () => {
      const error = new Error('Template not found');
      vi.mocked(api.get).mockRejectedValue(error);

      await expect(blueprintTemplates.export('invalid-id')).rejects.toThrow('Template not found');
    });
  });

  describe('createInstanceInFactory', () => {
    it('should create an instance from template in factory', async () => {
      const requestData: CreateFromTemplateRequest = {
        name: 'Instance Name',
      };

      const mockResponse: CreateFromTemplateResponse = {
        message: 'Blueprint instance created in factory factory-1',
        blueprint_id: 'instance-id',
        factory_id: 'factory-1',
      };

      vi.mocked(api.post).mockResolvedValue(mockResponse);

      const result = await blueprintTemplates.createInstanceInFactory('factory-1', 'template-1', requestData);

      expect(api.post).toHaveBeenCalledWith(
        '/factories/factory-1/production-lines/from-template/template-1',
        requestData
      );
      expect(result).toEqual(mockResponse);
    });

    it('should handle factory not found errors', async () => {
      const requestData: CreateFromTemplateRequest = {};
      const error = new Error('Factory not found');
      vi.mocked(api.post).mockRejectedValue(error);

      await expect(
        blueprintTemplates.createInstanceInFactory('invalid-factory', 'template-1', requestData)
      ).rejects.toThrow('Factory not found');
    });

    it('should handle template not found errors', async () => {
      const requestData: CreateFromTemplateRequest = {};
      const error = new Error('Template not found');
      vi.mocked(api.post).mockRejectedValue(error);

      await expect(
        blueprintTemplates.createInstanceInFactory('factory-1', 'invalid-template', requestData)
      ).rejects.toThrow('Template not found');
    });
  });
});
