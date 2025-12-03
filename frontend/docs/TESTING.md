# Testing Guide (Simplified)

> **Note:** This project uses a simplified testing approach. See `README-TESTING.md` for the current strategy.

This document provides an overview of the testing infrastructure and guidelines for the Satisflow frontend application.

## Quick Start

```bash
npm run test          # Run all tests
npm run test:watch    # Watch mode
npm run test:coverage # With coverage
```

**Philosophy:** Backend is the source of truth. Frontend tests are minimal - just composables/stores and calculation violation checks.

## Table of Contents

1. [Testing Stack](#testing-stack)
2. [Test Structure](#test-structure)
3. [Running Tests](#running-tests)
4. [Test Coverage](#test-coverage)
5. [Writing Tests](#writing-tests)
6. [CI/CD Integration](#cicd-integration)

## Testing Stack

### Unit Tests
- **Vitest**: Fast unit test framework with TypeScript support
- **Vue Test Utils**: Official testing library for Vue.js
- **Pinia**: State management testing utilities
- **jsdom**: DOM environment for tests

### E2E Tests
- **Playwright**: Modern E2E testing framework
- **Test fixtures**: Pre-configured test data

### Mocking
- **MSW (Mock Service Worker)**: API mocking for tests
- **Vitest Mocks**: Built-in mocking utilities

## Test Structure

```
frontend/
├── src/
│   ├── __tests__/          # Unit tests
│   ├── components/
│   │   └── __tests__/      # Component tests
│   ├── composables/
│   │   └── __tests__/      # Composable tests
│   ├── stores/
│   │   └── __tests__/      # Store tests
│   ├── api/
│   │   └── __tests__/      # API tests
│   └── test-utils/         # Test utilities
│       ├── setup.ts        # Test setup
│       ├── test-helpers.ts # Test helpers
│       └── mocks/          # Mock data
└── tests/
    └── e2e/                # E2E tests
```

## Running Tests

### Unit Tests

```bash
# Run all unit tests
npm run test:unit

# Run tests in watch mode
npm run test:unit -- --watch

# Run tests with coverage
npm run test:coverage

# Run tests for a specific file
npm run test:unit -- src/components/__tests__/DashboardStats.test.ts
```

### E2E Tests

```bash
# Run all E2E tests
npm run test:e2e

# Run E2E tests in headed mode
npm run test:e2e -- --headed

# Run E2E tests for a specific file
npm run test:e2e -- tests/e2e/dashboard.spec.ts
```

## Test Coverage

Test coverage is configured with the following thresholds:
- **Branches**: 70%
- **Functions**: 70%
- **Lines**: 70%
- **Statements**: 70%

Coverage reports are generated in:
- Terminal output
- `coverage/` directory (HTML and JSON formats)

## Writing Tests

### Unit Tests

#### Component Tests

```typescript
import { describe, it, expect, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import MyComponent from '../MyComponent.vue'

describe('MyComponent', () => {
  let wrapper: any
  let pinia: any

  beforeEach(() => {
    pinia = createPinia()
    setActivePinia(pinia)
  })

  it('renders correctly', () => {
    wrapper = mount(MyComponent, {
      props: { someProp: 'value' },
      global: {
        plugins: [pinia],
      },
    })

    expect(wrapper.find('.some-class').exists()).toBe(true)
  })
})
```

#### Store Tests

```typescript
import { describe, it, expect, beforeEach, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useMyStore } from '../myStore'

describe('MyStore', () => {
  let store: ReturnType<typeof useMyStore>

  beforeEach(() => {
    const pinia = createPinia()
    setActivePinia(pinia)
    store = useMyStore()
  })

  it('should have correct initial state', () => {
    expect(store.someState).toBe(null)
  })
})
```

### E2E Tests

```typescript
import { test, expect } from '@playwright/test'

test.describe('My Feature', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
  })

  test('performs expected behavior', async ({ page }) => {
    await page.click('.some-button')
    await expect(page.locator('.some-result')).toBeVisible()
  })
})
```

## Test Utilities

### Mock Data

Mock data is provided in `src/test-utils/mocks/`:
- `handlers.ts`: MSW request handlers
- `data.ts`: Sample data for tests

### Test Helpers

Common test utilities are provided in `src/test-utils/test-helpers.ts`:
- `mountWithStore`: Mount components with Pinia store
- `createMockFactory`: Create mock factory data
- `flushPromises`: Wait for all promises to resolve

## CI/CD Integration

Tests are automatically run on:
- Push to `main` and `develop` branches
- Pull requests to `main` and `develop` branches

### Workflow

1. **Unit Tests**: Run on multiple Node.js versions (18.x, 20.x)
2. **E2E Tests**: Run on latest Node.js with Playwright
3. **Coverage**: Upload to Codecov for tracking

### GitHub Actions

The test workflow is defined in `.github/workflows/test.yml`:
- Runs unit tests with coverage
- Runs E2E tests
- Uploads test results and coverage reports

## Best Practices

1. **Test Naming**: Use descriptive test names that explain what is being tested
2. **Test Isolation**: Each test should be independent and not rely on other tests
3. **Mocking**: Mock external dependencies to ensure tests are reliable and fast
4. **Coverage**: Aim for high test coverage, especially for critical business logic
5. **E2E Tests**: Use E2E tests for critical user journeys, not for every UI interaction
6. **Test Data**: Use realistic test data that matches production scenarios

## Debugging Tests

### Unit Tests

```bash
# Run tests with inspector for debugging
npm run test:unit -- --inspect-brk

# Run tests with verbose output
npm run test:unit -- --reporter=verbose
```

### E2E Tests

```bash
# Run tests in headed mode with browser UI
npm run test:e2e -- --headed

# Run tests with debug logging
npm run test:e2e -- --debug
```

## Troubleshooting

### Common Issues

1. **Mock API not working**: Ensure MSW is properly set up in test setup
2. **Component not rendering**: Check if all required props are provided
3. **Store state not updating**: Ensure Pinia is properly initialized
4. **E2E test timing**: Use proper wait methods instead of fixed timeouts

### Resources

- [Vitest Documentation](https://vitest.dev/)
- [Vue Test Utils Documentation](https://test-utils.vuejs.org/)
- [Playwright Documentation](https://playwright.dev/)
- [Pinia Testing Guide](https://pinia.vuejs.org/cookbook/testing.html)