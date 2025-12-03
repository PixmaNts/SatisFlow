# Simplified Frontend Testing Strategy

## Philosophy

**The backend is the single source of truth.** Since all calculations and business logic live in Rust, frontend testing should be minimal and focused on:

1. **Type safety** - TypeScript catches most errors at compile time
2. **Simple unit tests** - Test composables and stores (like Rust tests)
3. **Backend integration tests** - Test the actual API (Rust tests are easier!)

## What We Test

### ✅ Keep: Simple Unit Tests (Vitest)

**Test composables and stores** - These are pure functions, easy to test:

```typescript
// frontend/src/composables/__tests__/useValidation.test.ts
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

**Why this works:**
- Simple, like Rust tests
- Fast (no browser needed)
- Tests actual logic (validation, formatting)

### ✅ Keep: Calculation Violation Tests

**Prevent frontend from doing calculations** - This is critical:

```typescript
// frontend/src/tests/calculation-violations.test.ts
// Scans code for forbidden patterns like Math.pow(oc, 1.321928)
```

**Why this works:**
- Static analysis (no runtime needed)
- Prevents regression
- Aligns with architecture

### ❌ Remove: Complex E2E Tests (Playwright)

**Why remove Playwright:**
- Complex setup (browser, server, etc.)
- Flaky (timing issues, browser differences)
- Slow (takes minutes to run)
- Hard to debug
- **Backend tests already verify calculations work!**

### ✅ Replace: Backend Integration Tests

**Test the API directly** - This is what you're good at:

```rust
// crates/satisflow-server/tests/api_tests.rs
#[tokio::test]
async fn test_create_factory() {
    // Test the actual API endpoint
    // This is easier and more reliable than Playwright!
}
```

**Why this is better:**
- You already know Rust testing
- Tests the real API (not mocked)
- Fast and reliable
- Catches real bugs

## Recommended Test Structure

```
frontend/
├── src/
│   ├── composables/
│   │   └── __tests__/          # Simple unit tests (like Rust)
│   │       ├── useValidation.test.ts
│   │       ├── useErrorNotification.test.ts
│   │       └── useTheme.test.ts
│   ├── stores/
│   │   └── __tests__/          # Store tests (simple)
│   │       ├── factory.test.ts
│   │       └── dashboard.test.ts
│   └── tests/
│       └── calculation-violations.test.ts  # Keep this!
│
# NO e2e/ directory needed!
```

## Migration Plan

1. **Keep existing unit tests** - They're simple and work
2. **Remove Playwright** - Delete `e2e/` directory and Playwright config
3. **Add backend integration tests** - Test API endpoints in Rust
4. **Update package.json** - Remove E2E scripts

## What Gets Tested Where

| What | Where | Why |
|------|-------|-----|
| Calculations | Rust backend tests | Single source of truth |
| API endpoints | Rust integration tests | You know Rust testing |
| Validation logic | Vitest unit tests | Simple, fast |
| UI rendering | Manual testing | TypeScript catches most issues |
| User flows | Backend API tests | Test the real thing |

## Example: Testing a Feature

**Before (Complex):**
1. Write Playwright E2E test (complex, flaky)
2. Mock API responses
3. Wait for UI updates
4. Assert DOM elements

**After (Simple):**
1. Write Rust integration test for API endpoint ✅
2. Write Vitest unit test for validation ✅
3. Manual UI check (TypeScript + browser) ✅

## Benefits

- ✅ **Simpler** - No browser automation complexity
- ✅ **Faster** - Unit tests run in milliseconds
- ✅ **More reliable** - No timing issues or browser quirks
- ✅ **Easier to maintain** - Like Rust tests you already write
- ✅ **Better coverage** - Backend tests verify real behavior

## When to Add E2E Tests (Optional)

Only if you need to test:
- Browser-specific behavior (rare)
- Complex user interactions (not needed for this app)
- Visual regression (use screenshots manually)

For Satisflow, **backend integration tests are sufficient!**

