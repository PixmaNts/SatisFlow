# Simplified Frontend Testing

## Philosophy

**Backend is the source of truth.** Since all calculations live in Rust, frontend testing is minimal:

- ✅ **TypeScript** catches most errors at compile time
- ✅ **Simple unit tests** for composables/stores (like Rust tests)
- ✅ **Backend integration tests** verify real functionality (you already do this!)

## What We Test

### 1. Simple Unit Tests (Vitest)

Test composables and stores - pure functions, easy to test:

```bash
npm run test              # Run all tests
npm run test:watch       # Watch mode
npm run test:coverage    # With coverage
```

**Example test:**
```typescript
// src/composables/__tests__/useValidation.test.ts
import { describe, it, expect } from 'vitest'
import { useValidation } from '../useValidation'

describe('useValidation', () => {
  it('validates overclock range', () => {
    const { validateOverclock } = useValidation()
    expect(validateOverclock(150)).toBeNull() // valid
    expect(validateOverclock(300)).toBeTruthy() // invalid
  })
})
```

### 2. Calculation Violation Tests

**Prevent frontend from doing calculations** - Critical guard:

```bash
npm run test calculation-violations
```

This static analysis test scans code for forbidden patterns like `Math.pow(oc, 1.321928)` to ensure all calculations stay in Rust.

## What We DON'T Test

### ❌ E2E Tests (Removed)

**Why removed:**
- Complex setup (browser, server, timing)
- Flaky and slow
- **Backend tests already verify calculations!**

### ✅ Replace with Backend Tests

Test the API directly in Rust - this is what you're good at:

```rust
// crates/satisflow-server/tests/api_tests.rs
#[tokio::test]
async fn test_create_factory() {
    // Test the actual API endpoint
    // Easier and more reliable than Playwright!
}
```

## Test Structure

```
frontend/src/
├── composables/
│   └── __tests__/          # Simple unit tests
│       ├── useValidation.test.ts
│       └── useErrorNotification.test.ts
├── stores/
│   └── __tests__/           # Store tests
│       └── factory.test.ts
└── tests/
    └── calculation-violations.test.ts  # Keep this!
```

## Benefits

- ✅ **Simpler** - No browser automation
- ✅ **Faster** - Tests run in milliseconds
- ✅ **More reliable** - No timing issues
- ✅ **Easier to maintain** - Like Rust tests
- ✅ **Better coverage** - Backend tests verify real behavior

## When to Add Tests

**Add a Vitest test when:**
- Writing a new composable (validation, formatting, etc.)
- Adding store logic (state management)
- Need to verify a pure function

**Don't add tests for:**
- UI rendering (TypeScript + manual check)
- API calls (test in Rust backend)
- Calculations (test in Rust backend)

## Running Tests

```bash
# Development
npm run test:watch

# CI/Production
npm run test:ci

# With coverage
npm run test:coverage
```

## Migration from Playwright

All E2E tests have been removed. Their functionality is better tested in:

1. **Backend integration tests** - Test API endpoints directly
2. **Manual testing** - TypeScript + browser for UI
3. **Calculation violation tests** - Prevent frontend calculations

See `TESTING-STRATEGY.md` for the full rationale.

