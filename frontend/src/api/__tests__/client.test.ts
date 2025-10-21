import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import axios from 'axios'
import { api, handleApiError, checkApiConnection } from '../client'

// Mock axios
vi.mock('axios')
const mockedAxios = vi.mocked(axios)

describe('API Client', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  describe('GET requests', () => {
    it('should make successful GET request', async () => {
      const mockData = { id: 1, name: 'Test Factory' }
      mockedAxios.create = vi.fn().mockReturnValue({
        get: vi.fn().mockResolvedValue({ data: mockData, status: 200 }),
        post: vi.fn(),
        put: vi.fn(),
        delete: vi.fn(),
        patch: vi.fn(),
        interceptors: {
          request: { use: vi.fn() },
          response: { use: vi.fn() },
        },
      })

      const result = await api.get('/factories/1')

      expect(result).toEqual(mockData)
    })

    it('should handle GET request errors', async () => {
      const error = {
        response: {
          status: 404,
          data: { error: 'Factory not found' },
        },
      }
      mockedAxios.create = vi.fn().mockReturnValue({
        get: vi.fn().mockRejectedValue(error),
        post: vi.fn(),
        put: vi.fn(),
        delete: vi.fn(),
        patch: vi.fn(),
        interceptors: {
          request: { use: vi.fn() },
          response: { use: vi.fn() },
        },
      })

      await expect(api.get('/factories/999')).rejects.toEqual(error)
    })
  })

  describe('POST requests', () => {
    it('should make successful POST request', async () => {
      const requestData = { name: 'New Factory' }
      const mockResponse = { id: 2, name: 'New Factory' }
      mockedAxios.create = vi.fn().mockReturnValue({
        get: vi.fn(),
        post: vi.fn().mockResolvedValue({ data: mockResponse, status: 201 }),
        put: vi.fn(),
        delete: vi.fn(),
        patch: vi.fn(),
        interceptors: {
          request: { use: vi.fn() },
          response: { use: vi.fn() },
        },
      })

      const result = await api.post('/factories', requestData)

      expect(result).toEqual(mockResponse)
    })

    it('should handle POST request errors', async () => {
      const error = {
        response: {
          status: 400,
          data: { error: 'Invalid data' },
        },
      }
      mockedAxios.create = vi.fn().mockReturnValue({
        get: vi.fn(),
        post: vi.fn().mockRejectedValue(error),
        put: vi.fn(),
        delete: vi.fn(),
        patch: vi.fn(),
        interceptors: {
          request: { use: vi.fn() },
          response: { use: vi.fn() },
        },
      })

      await expect(api.post('/factories', {})).rejects.toEqual(error)
    })
  })

  describe('PUT requests', () => {
    it('should make successful PUT request', async () => {
      const requestData = { name: 'Updated Factory' }
      const mockResponse = { id: 1, name: 'Updated Factory' }
      mockedAxios.create = vi.fn().mockReturnValue({
        get: vi.fn(),
        post: vi.fn(),
        put: vi.fn().mockResolvedValue({ data: mockResponse, status: 200 }),
        delete: vi.fn(),
        patch: vi.fn(),
        interceptors: {
          request: { use: vi.fn() },
          response: { use: vi.fn() },
        },
      })

      const result = await api.put('/factories/1', requestData)

      expect(result).toEqual(mockResponse)
    })

    it('should handle PUT request errors', async () => {
      const error = {
        response: {
          status: 404,
          data: { error: 'Factory not found' },
        },
      }
      mockedAxios.create = vi.fn().mockReturnValue({
        get: vi.fn(),
        post: vi.fn(),
        put: vi.fn().mockRejectedValue(error),
        delete: vi.fn(),
        patch: vi.fn(),
        interceptors: {
          request: { use: vi.fn() },
          response: { use: vi.fn() },
        },
      })

      await expect(api.put('/factories/999', {})).rejects.toEqual(error)
    })
  })

  describe('DELETE requests', () => {
    it('should make successful DELETE request', async () => {
      const mockResponse = { success: true }
      mockedAxios.create = vi.fn().mockReturnValue({
        get: vi.fn(),
        post: vi.fn(),
        put: vi.fn(),
        delete: vi.fn().mockResolvedValue({ data: mockResponse, status: 200 }),
        patch: vi.fn(),
        interceptors: {
          request: { use: vi.fn() },
          response: { use: vi.fn() },
        },
      })

      const result = await api.delete('/factories/1')

      expect(result).toEqual(mockResponse)
    })

    it('should handle DELETE request errors', async () => {
      const error = {
        response: {
          status: 404,
          data: { error: 'Factory not found' },
        },
      }
      mockedAxios.create = vi.fn().mockReturnValue({
        get: vi.fn(),
        post: vi.fn(),
        put: vi.fn(),
        delete: vi.fn().mockRejectedValue(error),
        patch: vi.fn(),
        interceptors: {
          request: { use: vi.fn() },
          response: { use: vi.fn() },
        },
      })

      await expect(api.delete('/factories/999')).rejects.toEqual(error)
    })
  })

  describe('handleApiError utility', () => {
    it('should handle Axios errors with response data', () => {
      const error = {
        response: {
          data: { error: 'Not found' },
        },
      }
      // Mock isAxiosError as a type guard function
      const originalIsAxiosError = axios.isAxiosError
      axios.isAxiosError = vi.fn().mockReturnValue(true) as any

      const result = handleApiError(error)
      expect(result).toBe('Not found')

      // Restore original function
      axios.isAxiosError = originalIsAxiosError
    })

    it('should handle regular errors', () => {
      const error = new Error('Something went wrong')
      // Mock isAxiosError as a type guard function
      const originalIsAxiosError = axios.isAxiosError
      axios.isAxiosError = vi.fn().mockReturnValue(false) as any

      const result = handleApiError(error)
      expect(result).toBe('Something went wrong')

      // Restore original function
      axios.isAxiosError = originalIsAxiosError
    })

    it('should handle unknown errors', () => {
      const error = 'string error'
      // Mock isAxiosError as a type guard function
      const originalIsAxiosError = axios.isAxiosError
      axios.isAxiosError = vi.fn().mockReturnValue(false) as any

      const result = handleApiError(error)
      expect(result).toBe('An unexpected error occurred')

      // Restore original function
      axios.isAxiosError = originalIsAxiosError
    })
  })

  describe('checkApiConnection utility', () => {
    it('should return true when health check succeeds', async () => {
      // Mock the api.get method
      vi.spyOn(api, 'get').mockResolvedValue({ status: 'ok' })

      const result = await checkApiConnection()
      expect(result).toBe(true)
    })

    it('should return false when health check fails', async () => {
      vi.spyOn(api, 'get').mockRejectedValue(new Error('Connection failed'))

      const result = await checkApiConnection()
      expect(result).toBe(false)
    })
  })
})
