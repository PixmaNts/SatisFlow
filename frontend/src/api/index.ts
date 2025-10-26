/**
 * API Module Index
 *
 * This file exports all API-related modules for convenient importing.
 * Use this file to import everything from the API at once:
 *
 * import { api, endpoints, handleApiError } from '@/api';
 */

// Export the API client
export { api, handleApiError, checkApiConnection } from './client';
export { default as apiClient } from './client';

// Export the endpoint functions
export { endpoints } from './endpoints';
export { factories, logistics, dashboard, gameData, system } from './endpoints';

// Export all types
export * from './types';
