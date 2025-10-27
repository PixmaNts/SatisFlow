# Calculation Migration Plan: Moving All Calculations to Engine

## Executive Summary

This document outlines a comprehensive plan to eliminate all game calculations from the frontend and ensure the Rust engine is the **single source of truth** for all Satisfactory game data and calculations.

**Current Problem**: Frontend contains hardcoded game data (base power values, fuel rates, formulas) and performs calculations that should only exist in the engine.

**Goal**: Frontend becomes a "dumb" display layer that only formats data. All calculations happen in the engine.

---

## Architecture Philosophy

### ✅ Engine Responsibilities (Rust)
- Store all game data (recipes, items, power values, rates)
- Calculate power consumption/generation
- Apply overclock formulas (`(clockSpeed/100)^1.321928`)
- Apply somersloop formulas
- Calculate item rates (input/output)
- Validate game rules
- Provide calculated values via API

### ✅ Frontend Responsibilities (TypeScript/Vue)
- Display data from backend
- Format values for UI (e.g., "123.4 MW" string formatting)
- Handle user input validation (basic checks before API calls)
- Manage UI state (modals, forms, routing)

### ❌ Frontend Must NEVER
- Store base power values
- Store fuel consumption rates
- Apply game formulas
- Calculate power/rates
- Duplicate game logic

---

## Current Violations Analysis

### 🔴 Critical: Wrong Data Being Displayed

| File | Issue | Impact | Lines |
|------|-------|--------|-------|
| **ProductionLineList.vue** | Assumes all machines consume 4 MW | **WRONG power shown to users** | 295-308 |
| **PowerGeneratorForm.vue** | Hardcoded base power & fuel rates | Duplicate of engine data | 249-286 |
| **PowerGeneratorList.vue** | Same hardcoded values as form | Two places to update | 209-242 |
| **RawInputList.vue** | Hardcoded extractor power values | Duplicate of engine data | 224-239 |
| **RawInputForm.vue** | Frontend calculates placeholder power | Creates incorrect initial value | 443 |

### 🟡 Medium: Duplicate Logic

| File | Issue | Lines |
|------|-------|-------|
| **ProductionLineForm.vue** | Uses backend data but applies frontend formula | 245-256 |

### ✅ Acceptable: Display-Only Formatting

| File | Function | Reason It's OK |
|------|----------|----------------|
| Multiple files | `formatPower()` | Just string formatting (MW/kW display) |

---

## Engine Capabilities (Already Implemented)

The engine already has all necessary calculation methods:

### ProductionLine (production_line.rs:55-62)
```rust
impl ProductionLine {
    pub fn total_machines(&self) -> u32
    pub fn total_somersloop(&self) -> u32
    pub fn output_rate(&self) -> Vec<(Item, f32)>
    pub fn input_rate(&self) -> Vec<(Item, f32)>
    pub fn total_power_consumption(&self) -> f32
}
```

### PowerGenerator (power_generator.rs:248-283)
```rust
impl PowerGenerator {
    pub fn total_power_generation(&self) -> f32
    pub fn total_fuel_consumption(&self) -> f32
    pub fn waste_production_rate(&self) -> f32
}
```

### RawInput (raw_input.rs:266-276)
```rust
impl RawInput {
    pub fn power_consumption(&self) -> f32
    pub fn calculate_extraction_rate(extractor_type, purity) -> f32
}
```

**The engine has everything we need. We just need to expose it via API.**

---

## Migration Plan

### Phase 1: Backend API Extensions 🔴 HIGH PRIORITY

#### 1.1 Production Line API - Add Calculated Fields

**Current Response Structure:**
```typescript
interface ProductionLineResponse {
  id: string
  name: string
  description: string | null
  ProductionLineRecipe?: {
    recipe: string
    machine_groups: MachineGroup[]
  }
  ProductionLineBlueprint?: {
    production_lines: ProductionLineRecipe[]
  }
}
```

**Missing Fields:**
- ❌ `total_power_consumption`
- ❌ `total_machines`
- ❌ `total_somersloop`
- ❌ `input_items`
- ❌ `output_items`

**Required Changes:**

**Backend:**
- File: `crates/satisflow-server/src/handlers/factory.rs`
- Add to response DTOs:
  ```rust
  #[derive(Serialize)]
  pub struct ProductionLineResponse {
      pub id: Uuid,
      pub name: String,
      pub description: Option<String>,
      pub total_power_consumption: f32,  // ✅ NEW
      pub total_machines: u32,            // ✅ NEW
      pub total_somersloop: u32,          // ✅ NEW
      pub input_items: Vec<(String, f32)>, // ✅ NEW: (item_name, rate)
      pub output_items: Vec<(String, f32)>, // ✅ NEW: (item_name, rate)
      // ... existing variant fields
  }
  ```
- Populate from engine methods in handlers

**Impact:**
- Fixes ProductionLineList.vue incorrect power display
- Removes need for frontend calculations

---

#### 1.2 Power Generator API - Add Calculated Fields

**Current Response Structure:**
```typescript
interface PowerGeneratorResponse {
  id: string
  generator_type: GeneratorType
  fuel_type: string
  groups: GeneratorGroup[]
}
```

**Missing Fields:**
- ❌ `total_power_generation`
- ❌ `total_fuel_consumption`
- ❌ `waste_production_rate` (for nuclear)
- ❌ `base_power_output` (for display info)
- ❌ `base_fuel_consumption` (for display info)

**Required Changes:**

**Backend:**
- File: `crates/satisflow-server/src/handlers/factory.rs`
- Add to response DTO:
  ```rust
  #[derive(Serialize)]
  pub struct PowerGeneratorResponse {
      pub id: Uuid,
      pub generator_type: GeneratorType,
      pub fuel_type: Item,
      pub groups: Vec<GeneratorGroupResponse>,
      pub total_power_generation: f32,    // ✅ NEW: Sum across all groups
      pub total_fuel_consumption: f32,    // ✅ NEW: Sum across all groups
      pub waste_production_rate: f32,     // ✅ NEW: 0 for non-nuclear
      pub base_power_output: f32,         // ✅ NEW: For UI info
      pub base_fuel_consumption: f32,     // ✅ NEW: For UI info
  }
  ```

**Impact:**
- Removes hardcoded `basePower` and `baseFuelRate` from PowerGeneratorForm.vue
- Removes duplicate data from PowerGeneratorList.vue

---

#### 1.3 Raw Input API - Add Calculated Fields

**Current Response Structure:**
```typescript
interface RawInputResponse {
  id: string
  extractor_type: ExtractorType
  item: string
  purity: Purity | null
  quantity_per_min: f32
  pressurizer?: ResourceWellPressurizer
  extractors?: ResourceWellExtractor[]
}
```

**Missing Fields:**
- ❌ `calculated_power_consumption`
- ❌ `base_power_consumption` (for display)

**Required Changes:**

**Backend:**
- File: `crates/satisflow-server/src/handlers/factory.rs`
- Add to response DTO:
  ```rust
  #[derive(Serialize)]
  pub struct RawInputResponse {
      // ... existing fields
      pub calculated_power_consumption: f32, // ✅ NEW: From RawInput::power_consumption()
      pub base_power_consumption: f32,       // ✅ NEW: For UI info
  }
  ```

**Impact:**
- Removes hardcoded `basePower` from RawInputList.vue
- Removes calculation from RawInputForm.vue

---

### Phase 2: Preview/Calculation Endpoints 🟡 MEDIUM PRIORITY

For real-time form previews (before saving), create dedicated endpoints.

#### 2.1 Production Line Preview Endpoint

**Purpose**: Calculate power/rates as user edits form

**Endpoint**: `POST /api/factories/:factory_id/production-lines/preview`

**Request Body**:
```typescript
{
  name: string
  recipe: string  // Recipe name
  machine_groups: Array<{
    number_of_machine: number
    oc_value: number
    somersloop: number
  }>
}
```

**Response**:
```typescript
{
  total_power_consumption: number
  total_machines: number
  input_items: Array<[string, number]>
  output_items: Array<[string, number]>
  valid: boolean
  validation_errors?: string[]
}
```

**Implementation**:
- Create temporary `ProductionLineRecipe` in memory
- Run engine calculations
- Return results without saving
- Use debouncing in frontend (300ms) to avoid excessive API calls

---

#### 2.2 Power Generator Preview Endpoint

**Purpose**: Calculate generation/consumption as user edits form

**Endpoint**: `POST /api/factories/:factory_id/power-generators/preview`

**Request Body**:
```typescript
{
  generator_type: GeneratorType
  fuel_type: string
  groups: Array<{
    number_of_generators: number
    clock_speed: number
  }>
}
```

**Response**:
```typescript
{
  total_power_generation: number
  total_fuel_consumption: number
  waste_production_rate: number
  valid: boolean
  validation_errors?: string[]
}
```

---

#### 2.3 Raw Input Preview Endpoint

**Purpose**: Calculate power/extraction as user edits form

**Endpoint**: `POST /api/factories/:factory_id/raw-inputs/preview`

**Request Body**:
```typescript
{
  extractor_type: ExtractorType
  item: string
  purity?: Purity
  pressurizer?: {
    clock_speed: number
  }
  extractors?: Array<{
    purity: Purity
  }>
}
```

**Response**:
```typescript
{
  quantity_per_min: number
  power_consumption: number
  valid: boolean
  validation_errors?: string[]
}
```

---

### Phase 3: Frontend Refactoring 🔴 HIGH PRIORITY

#### 3.1 ProductionLineList.vue

**File**: `frontend/src/components/factory/ProductionLineList.vue`

**Current Problem (Lines 295-308)**:
```typescript
// ❌ WRONG: Hardcoded 4 MW assumption
const getPowerConsumption = (line: ProductionLineResponse): number => {
  if (isProductionLineRecipe(line)) {
    return line.ProductionLineRecipe.machine_groups.reduce(
      (total: number, group: MachineGroup) =>
        total + (group.number_of_machine * 4 * Math.pow(group.oc_value / 100, 1.321928)),
      0
    )
  }
  // ...
}
```

**Solution**:
```typescript
// ✅ CORRECT: Use backend-calculated value
const getPowerConsumption = (line: ProductionLineResponse): number => {
  return line.total_power_consumption
}
```

**Changes Required**:
1. Remove `getPowerConsumption()` function entirely
2. Display `line.total_power_consumption` directly
3. Remove all `Math.pow()` calls
4. Display `line.input_items` and `line.output_items` directly

**Testing**:
- Verify power values match engine calculations
- Test with various machine types (Constructor, Assembler, Manufacturer)
- Confirm overclock multipliers are correct

---

#### 3.2 PowerGeneratorForm.vue

**File**: `frontend/src/components/factory/PowerGeneratorForm.vue`

**Current Problem (Lines 249-286)**:
```typescript
// ❌ WRONG: Hardcoded game data
const basePower: Record<GeneratorType, number> = {
  Biomass: 30,
  Coal: 75,
  Fuel: 150,
  Nuclear: 2500,
  Geothermal: 200
}

const baseFuelRate: Record<GeneratorType, number> = {
  Biomass: 4,
  Coal: 15.3,
  Fuel: 4.5,
  Nuclear: 0.025,
  Geothermal: 0
}

const calculatedPower = computed(() => {
  return formData.value.groups.reduce((total, group) => {
    const clockSpeed = group.clock_speed / 100
    const powerMultiplier = Math.pow(clockSpeed, 1.321928)
    return total + (group.number_of_generators * basePowerValue * powerMultiplier)
  }, 0)
})
```

**Solution**:
```typescript
// ✅ CORRECT: Use preview API with debouncing
const calculatedPower = ref(0)
const calculatedFuelRate = ref(0)

// Debounced API call
const updatePreview = debounce(async () => {
  if (!formData.value.generator_type || !formData.value.fuel_type) return

  try {
    const preview = await api.post(`/factories/${factoryId}/power-generators/preview`, {
      generator_type: formData.value.generator_type,
      fuel_type: formData.value.fuel_type,
      groups: formData.value.groups
    })

    calculatedPower.value = preview.total_power_generation
    calculatedFuelRate.value = preview.total_fuel_consumption
  } catch (error) {
    console.error('Preview failed:', error)
  }
}, 300)

// Watch for form changes
watch(() => formData.value, updatePreview, { deep: true })
```

**Changes Required**:
1. Remove `basePower` object
2. Remove `baseFuelRate` object
3. Remove `calculatedPower` computed property
4. Add preview API integration with debouncing
5. Remove all `Math.pow()` calls

**Testing**:
- Verify preview updates as user types
- Test debouncing (should not spam API)
- Confirm values match saved generator data

---

#### 3.3 PowerGeneratorList.vue

**File**: `frontend/src/components/factory/PowerGeneratorList.vue`

**Current Problem (Lines 209-242)**:
```typescript
// ❌ WRONG: Duplicate hardcoded data
const basePower: Record<GeneratorType, number> = { /* ... */ }
const baseFuelRate: Record<GeneratorType, number> = { /* ... */ }
```

**Solution**:
```typescript
// ✅ CORRECT: Use response fields
const displayGenerator = (gen: PowerGeneratorResponse) => {
  return {
    type: gen.generator_type,
    fuel: gen.fuel_type,
    power: gen.total_power_generation,      // ✅ From API
    fuelRate: gen.total_fuel_consumption,   // ✅ From API
    wasteRate: gen.waste_production_rate    // ✅ From API
  }
}
```

**Changes Required**:
1. Remove all hardcoded constants
2. Display API-provided calculated values
3. Remove calculation logic

---

#### 3.4 RawInputList.vue

**File**: `frontend/src/components/factory/RawInputList.vue`

**Current Problem (Lines 224-239)**:
```typescript
// ❌ WRONG: Hardcoded extractor power
const basePower: Record<ExtractorType, number> = {
  MinerMk1: 5,
  MinerMk2: 12,
  MinerMk3: 30,
  WaterExtractor: 20,
  OilExtractor: 40,
  ResourceWellExtractor: 0
}

if (input.pressurizer) {
  const clockSpeed = input.pressurizer.clock_speed / 100
  return 150 * Math.pow(clockSpeed, 1.321928)
}
```

**Solution**:
```typescript
// ✅ CORRECT: Use API field
const getPower = (input: RawInputResponse): number => {
  return input.calculated_power_consumption
}
```

**Changes Required**:
1. Remove `basePower` object
2. Remove pressurizer power calculation
3. Display `input.calculated_power_consumption`

---

#### 3.5 RawInputForm.vue

**File**: `frontend/src/components/factory/RawInputForm.vue`

**Current Problem (Line 443)**:
```typescript
// ❌ WRONG: Frontend calculates placeholder
power_consumption: 150 * Math.pow(1, 1.321928)
```

**Solution**:
```typescript
// ✅ CORRECT: Use preview API
const calculatedPower = ref(0)

const updatePreview = debounce(async () => {
  const preview = await api.post(`/factories/${factoryId}/raw-inputs/preview`, {
    extractor_type: formData.value.extractor_type,
    item: formData.value.item,
    purity: formData.value.purity,
    pressurizer: formData.value.pressurizer,
    extractors: formData.value.extractors
  })

  calculatedPower.value = preview.power_consumption
}, 300)
```

**Changes Required**:
1. Remove hardcoded power calculation
2. Add preview API integration
3. Remove `Math.pow()` call

---

#### 3.6 ProductionLineForm.vue

**File**: `frontend/src/components/forms/ProductionLineForm.vue`

**Current Problem (Lines 245-256)**:
```typescript
// ⚠️ Uses backend machine data but applies frontend formula
return formData.value.machine_groups.reduce((total, group) => {
  const basePower = selectedMachineInfo.value!.base_power
  const clockSpeed = group.oc_value / 100
  const powerMultiplier = Math.pow(clockSpeed, 1.321928)
  const somersloopMultiplier = Math.pow(1 + group.somersloop / selectedMachineInfo.value!.max_somersloop, 2)

  return total + (group.number_of_machine * basePower * powerMultiplier * somersloopMultiplier)
}, 0)
```

**Solution**:
```typescript
// ✅ CORRECT: Use preview API
const calculatedPower = ref(0)
const calculatedInputs = ref<Array<[string, number]>>([])
const calculatedOutputs = ref<Array<[string, number]>>([])

const updatePreview = debounce(async () => {
  if (!formData.value.recipe) return

  const preview = await api.post(`/factories/${factoryId}/production-lines/preview`, {
    name: formData.value.name,
    recipe: formData.value.recipe,
    machine_groups: formData.value.machine_groups
  })

  calculatedPower.value = preview.total_power_consumption
  calculatedInputs.value = preview.input_items
  calculatedOutputs.value = preview.output_items
}, 300)

watch(() => formData.value, updatePreview, { deep: true })
```

**Changes Required**:
1. Remove manual power calculation
2. Remove somersloop formula
3. Add preview API integration
4. Remove all `Math.pow()` calls

---

### Phase 4: Validation & Testing 🟢 LOW PRIORITY

#### 4.1 Backend Tests

Add tests to verify API responses include calculated fields:

**File**: `crates/satisflow-server/tests/api_tests.rs` (create if needed)

```rust
#[tokio::test]
async fn test_production_line_includes_calculations() {
    let response = get_production_lines(factory_id).await;

    for line in response.production_lines {
        assert!(line.total_power_consumption >= 0.0);
        assert!(line.total_machines > 0);
        assert!(!line.input_items.is_empty());
        assert!(!line.output_items.is_empty());
    }
}

#[tokio::test]
async fn test_power_generator_includes_calculations() {
    let response = get_power_generator(factory_id, gen_id).await;

    assert!(response.total_power_generation > 0.0);
    assert!(response.total_fuel_consumption >= 0.0);
    assert_eq!(response.base_power_output, response.generator_type.base_power_output());
}

#[tokio::test]
async fn test_production_line_preview_endpoint() {
    let preview = post_production_line_preview(factory_id, payload).await;

    assert!(preview.total_power_consumption > 0.0);
    assert!(preview.valid);
}
```

---

#### 4.2 Frontend Tests

Add tests to ensure no calculations in frontend:

**File**: `frontend/tests/no-calculations.test.ts`

```typescript
import { describe, it, expect } from 'vitest'
import fs from 'fs'
import path from 'path'

describe('Frontend Calculation Prevention', () => {
  const componentsDir = path.join(__dirname, '../src/components')

  it('should not contain Math.pow calls', () => {
    const files = getVueFiles(componentsDir)

    for (const file of files) {
      const content = fs.readFileSync(file, 'utf-8')
      expect(content).not.toContain('Math.pow')
    }
  })

  it('should not contain overclock formula constant', () => {
    const files = getVueFiles(componentsDir)

    for (const file of files) {
      const content = fs.readFileSync(file, 'utf-8')
      expect(content).not.toContain('1.321928')
    }
  })

  it('should not contain hardcoded base power values', () => {
    const files = ['PowerGeneratorForm.vue', 'PowerGeneratorList.vue']

    for (const filename of files) {
      const file = path.join(componentsDir, 'factory', filename)
      const content = fs.readFileSync(file, 'utf-8')

      expect(content).not.toContain('basePower')
      expect(content).not.toContain('Biomass: 30')
      expect(content).not.toContain('Coal: 75')
    }
  })
})
```

---

#### 4.3 Integration Tests

Verify frontend-backend integration:

**File**: `frontend/tests/e2e/calculations.spec.ts`

```typescript
import { test, expect } from '@playwright/test'

test('Production line shows correct power from backend', async ({ page }) => {
  await page.goto('/factory/test-factory')

  // Create production line
  await page.click('button:has-text("Add Production Line")')
  await page.fill('[name="recipe"]', 'Iron Ingot')
  await page.fill('[name="machines"]', '4')
  await page.fill('[name="overclock"]', '100')
  await page.click('button:has-text("Save")')

  // Verify power displayed matches engine calculation
  const powerText = await page.textContent('.production-line-power')
  expect(powerText).toContain('16.0 MW') // 4 machines * 4 MW each
})

test('Power generator preview updates in real-time', async ({ page }) => {
  await page.goto('/factory/test-factory/power')

  await page.click('button:has-text("Add Generator")')
  await page.selectOption('[name="type"]', 'Coal')
  await page.selectOption('[name="fuel"]', 'Coal')
  await page.fill('[name="count"]', '4')
  await page.fill('[name="clock_speed"]', '100')

  // Wait for debounced preview
  await page.waitForTimeout(400)

  // Verify preview shows correct values
  const powerText = await page.textContent('.preview-power')
  expect(powerText).toContain('300.0 MW') // 4 * 75 MW
})
```

---

## Missing API Endpoints Summary

### New Response Fields Needed

| Resource | Endpoint | Missing Fields |
|----------|----------|----------------|
| **Production Line** | `GET /api/factories/:id/production-lines` | `total_power_consumption`, `total_machines`, `total_somersloop`, `input_items`, `output_items` |
| **Production Line** | `GET /api/factories/:id/production-lines/:line_id` | Same as above |
| **Power Generator** | `GET /api/factories/:id/power-generators` | `total_power_generation`, `total_fuel_consumption`, `waste_production_rate`, `base_power_output`, `base_fuel_consumption` |
| **Power Generator** | `GET /api/factories/:id/power-generators/:gen_id` | Same as above |
| **Raw Input** | `GET /api/factories/:id/raw-inputs` | `calculated_power_consumption`, `base_power_consumption` |
| **Raw Input** | `GET /api/factories/:id/raw-inputs/:input_id` | Same as above |

### New Preview Endpoints Needed

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/factories/:id/production-lines/preview` | POST | Real-time calculation preview for form |
| `/api/factories/:id/power-generators/preview` | POST | Real-time calculation preview for form |
| `/api/factories/:id/raw-inputs/preview` | POST | Real-time calculation preview for form |

---

## Implementation Order

### Week 1: Backend Foundation
1. ✅ Day 1-2: Add calculated fields to `ProductionLineResponse`
2. ✅ Day 3-4: Add calculated fields to `PowerGeneratorResponse`
3. ✅ Day 5: Add calculated fields to `RawInputResponse`

### Week 2: Preview Endpoints
4. ✅ Day 1-2: Implement `/production-lines/preview` endpoint
5. ✅ Day 3: Implement `/power-generators/preview` endpoint
6. ✅ Day 4: Implement `/raw-inputs/preview` endpoint
7. ✅ Day 5: Backend testing

### Week 3: Frontend Refactoring (Critical Fixes)
8. ✅ Day 1: Fix ProductionLineList.vue (wrong power display)
9. ✅ Day 2: Fix PowerGeneratorForm.vue & PowerGeneratorList.vue
10. ✅ Day 3: Fix RawInputForm.vue & RawInputList.vue
11. ✅ Day 4-5: Testing & bug fixes

### Week 4: Frontend Refactoring (Forms)
12. ✅ Day 1-2: Refactor ProductionLineForm.vue with preview API
13. ✅ Day 3-5: Testing, cleanup, documentation

---

## Success Criteria

### ✅ Phase 1 Complete When:
- [ ] All GET endpoints return calculated fields
- [ ] No frontend component calculates power/rates
- [ ] ProductionLineList.vue shows correct power values
- [ ] Backend tests pass

### ✅ Phase 2 Complete When:
- [ ] All three preview endpoints implemented
- [ ] Preview endpoints return calculations
- [ ] Debouncing works (no excessive API calls)
- [ ] Integration tests pass

### ✅ Phase 3 Complete When:
- [ ] No `Math.pow()` in frontend code
- [ ] No hardcoded game constants (base power, fuel rates)
- [ ] No `1.321928` in frontend code
- [ ] All forms use preview API
- [ ] All list views use API-provided values
- [ ] Frontend tests pass

### ✅ Project Complete When:
- [ ] Engine is provably the single source of truth
- [ ] Frontend can be rebuilt from scratch using only API
- [ ] All tests pass (backend, frontend, e2e)
- [ ] Documentation updated
- [ ] Code review approved

---

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|-----------|
| **API latency affects UX** | Medium | Use debouncing (300ms), optimistic UI updates |
| **Breaking changes in API** | High | Version API, provide migration guide |
| **Missed calculations in frontend** | Medium | Automated tests to detect `Math.pow()` and game constants |
| **Preview endpoints overload backend** | Low | Rate limiting, caching, debouncing |
| **Regression in displayed values** | High | Comprehensive E2E tests comparing old vs new |

---

## Rollout Strategy

### Option A: Big Bang (Recommended)
- Complete all changes in feature branch
- Test thoroughly
- Deploy all at once
- **Pros**: Clean cut, no partial state
- **Cons**: Longer development time

### Option B: Incremental
- Deploy backend changes first (add fields)
- Frontend uses new fields gradually
- **Pros**: Smaller changes, easier rollback
- **Cons**: Temporary inconsistency, more complex

**Recommendation**: Option A (Big Bang) - The changes are interconnected and this ensures consistency.

---

## Documentation Updates Needed

1. **API Documentation** (`docs/api.md`):
   - Document all new response fields
   - Document preview endpoints
   - Add calculation examples

2. **Architecture Documentation** (`docs/architecture.md`):
   - Update to emphasize engine as single source of truth
   - Document calculation flow
   - Add diagrams

3. **Frontend Developer Guide** (`docs/frontend-guide.md`):
   - Rules for when to call backend
   - Debouncing best practices
   - Forbidden patterns (hardcoded values, formulas)

4. **Migration Guide** (`docs/migration-v2.md`):
   - Breaking changes list
   - Before/after examples
   - Upgrade checklist

---

## Long-Term Benefits

After this refactoring:

1. **Accuracy**: All calculations match game exactly (single source)
2. **Maintainability**: Game updates require changes in ONE place
3. **Type Safety**: Rust compiler catches calculation errors
4. **Performance**: Backend calculations can be cached/optimized
5. **Consistency**: Form preview matches saved values exactly
6. **Trust**: Users see correct values, builds confidence
7. **Testability**: Game logic tested once in Rust
8. **Flexibility**: Any UI (mobile, desktop, web) can use same API
9. **Documentation**: API serves as contract for all clients

---

## Appendix A: Formula Reference

### Overclock Power Formula (Consumers)
```
power_consumption = base_power × (clock_speed/100)^1.321928
```

### Overclock Power Formula (Generators)
```
power_generation = base_power × (clock_speed/100)
fuel_consumption = base_fuel × (clock_speed/100)
```
*Note: Generators scale linearly, consumers use exponent*

### Somersloop Power Formula
```
somersloop_multiplier = (1 + somersloop_count / max_somersloop)²
total_power = base_power × somersloop_multiplier × overclock_multiplier
```

### Somersloop Production Formula
```
production_multiplier = (1 + somersloop_count / max_somersloop)
output_rate = base_rate × production_multiplier × (clock_speed/100)
```

---

## Appendix B: Code Review Checklist

Before marking this migration complete, verify:

- [ ] No `Math.pow()` in frontend code
- [ ] No hardcoded game constants in frontend
- [ ] No `1.321928` magic number in frontend
- [ ] All API responses include calculated fields
- [ ] All forms use preview endpoints with debouncing
- [ ] All list views display API-provided calculations
- [ ] Backend has comprehensive unit tests
- [ ] Frontend has integration tests
- [ ] E2E tests verify calculations
- [ ] API documentation updated
- [ ] Architecture documentation updated
- [ ] Migration guide created

---

**Document Version**: 1.0
**Last Updated**: 2025-10-27
**Status**: Planning Phase
**Owner**: Development Team
