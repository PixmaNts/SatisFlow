import { describe, it, expect, afterEach, vi } from 'vitest'

const setupClient = async (instanceOverrides: Partial<Record<'get' | 'post' | 'put' | 'delete' | 'patch', (...args: any[]) => any>> = {}) => {
  const axiosInstance = {
    get: vi.fn(),
    post: vi.fn(),
    put: vi.fn(),
    delete: vi.fn(),
    patch: vi.fn(),
    interceptors: {
      request: { use: vi.fn() },
      response: { use: vi.fn() },
    },
    ...instanceOverrides,
  }

  vi.doMock('axios', () => {
    return {
      default: {
        create: vi.fn(() => axiosInstance),
        isAxiosError: (error: unknown): error is { response?: { data?: { error?: string } } } =>
          Boolean((error as any)?.isAxiosError),
      },
    }
  })

  const module = await import('../client')
  return { axiosInstance, api: module.api, handleApiError: module.handleApiError, checkApiConnection: module.checkApiConnection }
}

afterEach(() => {
  vi.resetModules()
  vi.doUnmock('axios')
})

describe('API client', () => {
  it('performs GET requests and returns response data', async () => {
    const mockResponse = { data: { id: 'factory-1' }, status: 200 }
    const { api, axiosInstance } = await setupClient({ get: vi.fn().mockResolvedValue(mockResponse) })

    const data = await api.get('/factories/factory-1')

    expect(axiosInstance.get).toHaveBeenCalledWith('/factories/factory-1', undefined)
    expect(data).toEqual(mockResponse.data)
  })

  it('propagates request errors from the underlying client', async () => {
    const error = { response: { status: 404, data: { error: 'Not found' } } }
    const { api, axiosInstance } = await setupClient({ get: vi.fn().mockRejectedValue(error) })

    await expect(api.get('/factories/missing')).rejects.toEqual(error)
    expect(axiosInstance.get).toHaveBeenCalled()
  })
})

describe('handleApiError', () => {
  it('returns backend error message for Axios errors', async () => {
    const { handleApiError } = await setupClient()

    const message = handleApiError({ isAxiosError: true, response: { data: { error: 'Invalid payload' } } })
    expect(message).toBe('Invalid payload')
  })

  it('falls back to error.message when available', async () => {
    const { handleApiError } = await setupClient()

    const message = handleApiError(new Error('Something went wrong'))
    expect(message).toBe('Something went wrong')
  })
})

describe('checkApiConnection', () => {
  it('resolves to true when the health endpoint responds', async () => {
    const { checkApiConnection, axiosInstance } = await setupClient({ get: vi.fn().mockResolvedValue({ data: { status: 'ok' } }) })

    await expect(checkApiConnection()).resolves.toBe(true)
    expect(axiosInstance.get).toHaveBeenCalledWith('/health', undefined)
  })

  it('resolves to false when the health endpoint fails', async () => {
    const { checkApiConnection, axiosInstance } = await setupClient({ get: vi.fn().mockRejectedValue(new Error('Network error')) })

    await expect(checkApiConnection()).resolves.toBe(false)
    expect(axiosInstance.get).toHaveBeenCalled()
  })
})
