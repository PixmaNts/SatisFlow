import axios from 'axios';
import type { AxiosInstance, AxiosRequestConfig, AxiosResponse, AxiosError } from 'axios';
import type { ErrorResponse } from './types';
import { errorNotification } from '@/composables/useErrorNotification';

/**
 * API Client Configuration
 *
 * Provides a configured Axios instance with error handling,
 * request/response interceptors, and proper TypeScript typing.
 */

// Determine base URL based on environment
const getBaseURL = (): string => {
  // In development, use the proxy configured in Vite
  if (import.meta.env.DEV) {
    return '/api';
  }

  // In production, use the configured API URL or default to relative path
  return import.meta.env.VITE_API_URL || '/api';
};

// Create Axios instance with default configuration
const apiClient: AxiosInstance = axios.create({
  baseURL: getBaseURL(),
  timeout: 30000, // 30 seconds timeout
  headers: {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
  },
});

// Request interceptor to add authentication or other headers if needed
apiClient.interceptors.request.use(
  (config) => {
    // Add any global request modifications here
    // For example, adding auth tokens:
    // const token = getAuthToken();
    // if (token) {
    //   config.headers.Authorization = `Bearer ${token}`;
    // }

    // Log request in development
    if (import.meta.env.DEV) {
      console.log(`API Request: ${config.method?.toUpperCase()} ${config.url}`, {
        params: config.params,
        data: config.data,
      });
    }

    return config;
  },
  (error) => {
    console.error('Request interceptor error:', error);
    return Promise.reject(error);
  }
);

// Response interceptor for error handling and logging
apiClient.interceptors.response.use(
  (response: AxiosResponse) => {
    // Log successful response in development
    if (import.meta.env.DEV) {
      console.log(`API Response: ${response.config.method?.toUpperCase()} ${response.config.url}`, {
        status: response.status,
        data: response.data,
      });
    }

    return response;
  },
  (error: AxiosError<ErrorResponse>) => {
    // Use centralized error notification
    // The error will be normalized and shown as a toast notification
    errorNotification.handleApiError(error);

    // Still reject the promise so calling code can handle it if needed
    return Promise.reject(error);
  }
);

/**
 * Generic HTTP methods with proper typing
 */
export const api = {
  /**
   * Perform a GET request
   * @param url - The endpoint URL
   * @param config - Optional Axios configuration
   * @returns Promise with typed response data
   */
  get: async <T = unknown>(url: string, config?: AxiosRequestConfig): Promise<T> => {
    const response = await apiClient.get<T>(url, config);
    return response.data;
  },

  /**
   * Perform a POST request
   * @param url - The endpoint URL
   * @param data - The request body data
   * @param config - Optional Axios configuration
   * @returns Promise with typed response data
   */
  post: async <T = unknown>(url: string, data?: unknown, config?: AxiosRequestConfig): Promise<T> => {
    const response = await apiClient.post<T>(url, data, config);
    return response.data;
  },

  /**
   * Perform a PUT request
   * @param url - The endpoint URL
   * @param data - The request body data
   * @param config - Optional Axios configuration
   * @returns Promise with typed response data
   */
  put: async <T = unknown>(url: string, data?: unknown, config?: AxiosRequestConfig): Promise<T> => {
    const response = await apiClient.put<T>(url, data, config);
    return response.data;
  },

  /**
   * Perform a DELETE request
   * @param url - The endpoint URL
   * @param config - Optional Axios configuration
   * @returns Promise with typed response data
   */
  delete: async <T = unknown>(url: string, config?: AxiosRequestConfig): Promise<T> => {
    const response = await apiClient.delete<T>(url, config);
    return response.data;
  },

  /**
   * Perform a PATCH request
   * @param url - The endpoint URL
   * @param data - The request body data
   * @param config - Optional Axios configuration
   * @returns Promise with typed response data
   */
  patch: async <T = unknown>(url: string, data?: unknown, config?: AxiosRequestConfig): Promise<T> => {
    const response = await apiClient.patch<T>(url, data, config);
    return response.data;
  },
};

/**
 * Utility function to handle API errors consistently
 * @param error - The error object from Axios
 * @returns Normalized error with message and metadata
 * @deprecated Use errorNotification.handleError() or errorNotification.handleApiError() instead
 */
export const handleApiError = (error: unknown): string => {
  const normalized = errorNotification.normalizeError(error);
  return normalized.message;
};

/**
 * Check if the application is connected to the API
 * @returns Promise that resolves to true if connected, false otherwise
 */
export const checkApiConnection = async (): Promise<boolean> => {
  try {
    await api.get('/health');
    return true;
  } catch (error) {
    console.error('API connection check failed:', error);
    return false;
  }
};

export default apiClient;
