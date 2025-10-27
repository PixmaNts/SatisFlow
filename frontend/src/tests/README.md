# Frontend Calculation Violation Tests

This test suite prevents future violations of the **single source of truth** principle by scanning the frontend codebase for forbidden calculation patterns.

## Purpose

The Satisflow engine (Rust backend) should be the **single source of truth** for all game calculations. The frontend should only:

- ✅ Display data from the backend
- ✅ Format data for UI (e.g., "123.4 MW" formatting)
- ✅ Handle user input validation (basic checks before sending to backend)
- ✅ Manage UI state (modals, forms, etc.)

The frontend should **NEVER**:

- ❌ Calculate power consumption
- ❌ Calculate item rates
- ❌ Apply overclock formulas
- ❌ Store base power values
- ❌ Store fuel consumption rates
- ❌ Apply game formulas (like `Math.pow(oc, 1.321928)`)

## Test Structure

### `calculation-violations.test.ts`

Comprehensive test suite that scans all frontend files for forbidden patterns:

```typescript
describe('Frontend Calculation Violations', () => {
  // Scans 16+ critical files including components, views, stores, and API types

  it('should not contain Math.pow() calls with overclock formula', () => {
    // Catches: Math.pow(clockSpeed, 1.321928)
  })

  it('should not contain hardcoded base power values', () => {
    // Catches: Biomass: 30, Coal: 75, Fuel: 150, etc.
  })

  it('should not contain hardcoded extraction rates', () => {
    // Catches: MinerMk1: { Impure: 30, Normal: 60, Pure: 120 }
  })

  it('should not contain power calculation functions', () => {
    // Catches: getPowerConsumption(), calculatePower(), etc.
  })

  it('should only use acceptable Math functions for display formatting', () => {
    // Allows: Math.max, Math.abs, Math.round, Math.floor, Math.ceil
    // Forbids: Math.pow, Math.sqrt, Math.log, Math.exp (except in display contexts)
  })
})
```

## Forbidden Patterns

### 1. Math.pow() with Overclock Formula
```typescript
// ❌ FORBIDDEN
Math.pow(clockSpeed, 1.321928)
Math.pow(oc / 100, 1.321928)

// ✅ ALLOWED (display formatting only)
Math.abs(powerBalance) / maxPower * 100  // Percentage calculation
```

### 2. Hardcoded Base Power Values
```typescript
// ❌ FORBIDDEN
const basePower = {
  Biomass: 30,
  Coal: 75,
  Fuel: 150,
  Nuclear: 2500,
  Geothermal: 200
}

// ✅ ALLOWED (type definitions)
interface PowerGeneratorResponse {
  base_power: number;  // Just a field name
}
```

### 3. Hardcoded Extraction Rates
```typescript
// ❌ FORBIDDEN
const baseRates = {
  MinerMk1: { Impure: 30, Normal: 60, Pure: 120 },
  MinerMk2: { Impure: 60, Normal: 120, Pure: 240 },
  OilExtractor: { Impure: 60, Normal: 120, Pure: 240 }
}

// ✅ ALLOWED (display hints)
<span>(m³/min: Impure 60, Normal 120, Pure 240)</span>
```

### 4. Power Calculation Functions
```typescript
// ❌ FORBIDDEN
const getPowerConsumption = (line: ProductionLineResponse): number => {
  return line.machine_groups.reduce((total, group) => {
    return total + (group.number_of_machine * 4 * Math.pow(group.oc_value / 100, 1.321928))
  }, 0)
}

// ✅ ALLOWED (backend provides calculated values)
const displayPower = computed(() => productionLine.value.total_power)
```

## Running the Tests

```bash
# Run calculation violation tests
npm run test:unit -- calculation-violations.test.ts

# Run all tests (includes calculation violations)
npm run test:unit

# Run with coverage
npm run test:unit:coverage
```

## Test Results

### ✅ Passing Tests
- No Math.pow() calls with overclock formula (1.321928)
- No hardcoded base power values for generators
- No hardcoded fuel consumption rates
- No overclock formula applications
- Acceptable Math functions only (Math.max, Math.abs for display)

### ❌ Failing Tests (Expected)
When violations are found, tests will fail with detailed error messages:

```
Found hardcoded extraction rates in frontend code:

Violations:
  src/components/factory/RawInputForm.vue:359 - MinerMk1: { Impure: 30, Normal: 60, Pure: 120 }
  src/components/factory/RawInputForm.vue:360 - MinerMk2: { Impure: 60, Normal: 120, Pure: 240 }

These rates should be calculated by the backend engine based on game data.
Frontend should only display values provided by the API.
```

## Integration with CI/CD

These tests should be included in:

1. **Pre-commit hooks** - Prevent accidental violations during development
2. **Pull request checks** - Block PRs with calculation violations
3. **Release pipeline** - Ensure production builds are clean

## Migration Guide

When fixing violations found by these tests:

### For Hardcoded Values
1. **Remove** hardcoded values from frontend
2. **Add** calculated fields to backend API responses
3. **Update** frontend to use backend-provided values

### For Calculation Functions
1. **Move** calculation logic to backend engine
2. **Create** preview API endpoints for real-time calculations
3. **Update** frontend to call preview APIs instead of calculating

### Example Migration

**Before (❌):**
```typescript
// frontend/src/components/ProductionLineForm.vue
const calculatedPower = computed(() => {
  return formData.value.machine_groups.reduce((total, group) => {
    const basePower = selectedMachineInfo.value!.base_power
    const clockSpeed = group.oc_value / 100
    const powerMultiplier = Math.pow(clockSpeed, 1.321928)
    return total + (group.number_of_machine * basePower * powerMultiplier)
  }, 0)
})
```

**After (✅):**
```typescript
// frontend/src/components/ProductionLineForm.vue
const calculatedPower = ref(0)

// Call backend preview API
watch(() => formData.value, async (newData) => {
  const preview = await api.post('/production-lines/preview', newData)
  calculatedPower.value = preview.total_power
}, { debounce: 300 })
```

## Architecture Benefits

After migration:

1. **Type Safety**: Rust compiler catches calculation errors
2. **Single Source of Truth**: Game data in ONE place
3. **Consistency**: All calculations use same formulas
4. **Testability**: Backend calculations have comprehensive tests
5. **Game Updates**: Update ONE place when game patches change values
6. **Performance**: Backend calculations can be cached

## Related Documentation

- [FRONTEND_CALCULATION_REVIEW.md](../../FRONTEND_CALCULATION_REVIEW.md) - Complete analysis of violations
- [CALCULATION_MIGRATION_PLAN.md](../../CALCULATION_MIGRATION_PLAN.md) - Migration strategy
- [Backend API Documentation](../../../crates/satisflow-server/README.md) - API endpoints

## Contributing

When adding new frontend code:

1. **Run tests** before committing: `npm run test:unit`
2. **Check for violations** in your changes
3. **Use backend APIs** instead of frontend calculations
4. **Add tests** for new calculation-dependent features

The goal is **zero calculation violations** in the frontend codebase.