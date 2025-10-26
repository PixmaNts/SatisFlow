# Frontend Calculation Review - Violations of Single Source of Truth

## Overview

This document identifies all places in the frontend where we are **incorrectly performing game calculations** instead of relying on the backend engine. The engine (Rust) should be the **single source of truth** for all game data and calculations.

## Philosophy

> **The whole goal of the engine is to be the source of truth, a single place where we can safely encode the game data and calculations, in a strong typed and compiled language.**

### Why Backend Calculations?

1. **Type Safety**: Rust's strong type system prevents calculation errors at compile time
2. **Single Source of Truth**: Game data (recipes, power formulas, item rates) in ONE place
3. **Consistency**: All calculations use the same formulas and data
4. **Testability**: Backend calculations have comprehensive unit tests
5. **Game Updates**: When game patches change values, we update ONE place
6. **Correctness**: No floating-point errors, no missing edge cases

### Frontend Responsibility

The frontend should ONLY:
- ✅ Display data from the backend
- ✅ Format data for UI (e.g., "123.4 MW" formatting)
- ✅ Handle user input validation (basic checks before sending to backend)
- ✅ Manage UI state (modals, forms, etc.)

The frontend should NEVER:
- ❌ Calculate power consumption
- ❌ Calculate item rates
- ❌ Apply overclock formulas
- ❌ Store base power values
- ❌ Store fuel consumption rates
- ❌ Apply game formulas (like `Math.pow(oc, 1.321928)`)

---

## Issues Found

### 🔴 Critical: Hardcoded Game Data in Frontend

#### 1. **PowerGeneratorForm.vue** - Lines 249-286

**Issue**: Hardcoded base power and fuel consumption rates

```typescript
// ❌ WRONG: Hardcoded game data in frontend
const basePower: Record<GeneratorType, number> = {
  Biomass: 30,
  Coal: 75,
  Fuel: 150,
  Nuclear: 2500,
  Geothermal: 200
}

const baseFuelRate: Record<GeneratorType, number> = {
  Biomass: 4,
  Coal: 15.3,     // ❌ Hardcoded
  Fuel: 4.5,      // ❌ Hardcoded
  Nuclear: 0.025,
  Geothermal: 0
}

// ❌ WRONG: Frontend calculates power
const calculatedPower = computed(() => {
  return formData.value.groups.reduce((total, group) => {
    const clockSpeed = group.clock_speed / 100
    const powerMultiplier = Math.pow(clockSpeed, 1.321928)  // ❌ Game formula
    return total + (group.number_of_generators * basePowerValue * powerMultiplier)
  }, 0)
})
```

**Impact**:
- If game updates change coal generator fuel rate from 15.3 to 16.0, frontend shows wrong value
- No type safety - typo in fuel rate would go unnoticed
- Duplicate logic exists in backend

**Solution**: Backend should provide calculated values via API

---

#### 2. **PowerGeneratorList.vue** - Lines 209-242

**Issue**: Duplicate of above - same hardcoded values for display

```typescript
// ❌ WRONG: Same hardcoded values duplicated in list component
const basePower: Record<GeneratorType, number> = { /* ... same as form ... */ }
const baseFuelRate: Record<GeneratorType, number> = { /* ... same as form ... */ }
```

**Impact**:
- TWO places to update when game changes
- Risk of inconsistency between form and list
- Violates DRY principle

---

#### 3. **ProductionLineList.vue** - Lines 295-308

**Issue**: Frontend calculates production line power consumption

```typescript
// ❌ WRONG: Frontend power calculation with hardcoded 4 MW base
const getPowerConsumption = (line: ProductionLineResponse): number => {
  if (isProductionLineRecipe(line)) {
    return line.ProductionLineRecipe.machine_groups.reduce(
      (total: number, group: MachineGroup) =>
        total + (group.number_of_machine * 4 * Math.pow(group.oc_value / 100, 1.321928)),  // ❌
      0
    )
  }
  // ...
}
```

**Impact**:
- Assumes all machines consume 4 MW (WRONG! Assemblers use 15 MW, Manufacturers use 55 MW)
- Power consumption shown in list is **completely incorrect**
- Backend already calculates this correctly via `ProductionLine::total_power_consumption()`

**Critical**: This is showing users **wrong information**!

---

#### 4. **ProductionLineForm.vue** - Lines 245-256

**Issue**: Frontend power calculation (but at least uses backend machine data)

```typescript
// ⚠️ BETTER but still wrong: Uses backend machine data but frontend formula
return formData.value.machine_groups.reduce((total, group) => {
  const basePower = selectedMachineInfo.value!.base_power  // ✅ From backend
  const clockSpeed = group.oc_value / 100
  const powerMultiplier = Math.pow(clockSpeed, 1.321928)  // ❌ Frontend formula
  const somersloopMultiplier = Math.pow(1 + group.somersloop / selectedMachineInfo.value!.max_somersloop, 2)  // ❌

  return total + (group.number_of_machine * basePower * powerMultiplier * somersloopMultiplier)
}, 0)
```

**Impact**:
- Frontend applies game formulas
- If Satisfactory changes overclock formula, frontend breaks
- Somersloop formula might be wrong (need to verify with game)

---

#### 5. **RawInputList.vue** - Lines 224-239

**Issue**: Hardcoded extractor power values

```typescript
// ❌ WRONG: Hardcoded extractor base power
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
  return 150 * Math.pow(clockSpeed, 1.321928)  // ❌ Hardcoded 150 MW + formula
}
```

**Impact**:
- Game data duplicated in frontend
- Pressurizer power hardcoded to 150 MW

---

#### 6. **RawInputForm.vue** - Line 443

**Issue**: Placeholder power calculation

```typescript
// ❌ WRONG: Frontend sets power consumption
power_consumption: 150 * Math.pow(1, 1.321928) // Will be calculated by backend
```

**Impact**:
- Comment says "will be calculated by backend" but frontend still calculates it
- Creates incorrect initial value

---

### 🟡 Medium: Display-Only Calculations (Acceptable)

These calculations are **formatting only** and are acceptable:

#### FactoryView.vue, ProductionLineList.vue, etc. - formatPower()

```typescript
// ✅ OK: This is just formatting, not game logic
const formatPower = (power: number): string => {
  if (power < 1) {
    return `${(power * 1000).toFixed(0)} kW`  // ✅ Display formatting
  }
  return `${power.toFixed(1)} MW`
}
```

**Why it's OK**:
- Just converting MW to kW for display
- No game rules involved
- Simple math that can't be wrong

---

## Recommended Solutions

### Solution 1: Backend Provides Calculated Values (Preferred)

**For Power Generators:**

Current backend returns:
```rust
PowerGeneratorResponse {
    generator_type: "Coal",
    fuel_type: "Coal",
    groups: [/* ... */]
}
```

Should return:
```rust
PowerGeneratorResponse {
    generator_type: "Coal",
    fuel_type: "Coal",
    groups: [/* ... */],
    total_power_generation: 600.0,  // ✅ Calculated by engine
    total_fuel_consumption: 122.4   // ✅ Calculated by engine
}
```

**For Production Lines:**

Backend already has methods:
```rust
impl ProductionLine {
    pub fn total_power_consumption(&self) -> f32 { /* ... */ }
    pub fn total_machines(&self) -> u32 { /* ... */ }
    pub fn input_rate(&self) -> Vec<(Item, f32)> { /* ... */ }
    pub fn output_rate(&self) -> Vec<(Item, f32)> { /* ... */ }
}
```

These should be included in `ProductionLineResponse`:
```typescript
interface ProductionLineResponse {
  // ... existing fields ...
  total_power: number           // ✅ From engine
  total_machines: number        // ✅ From engine
  input_items: [string, number][]   // ✅ From engine
  output_items: [string, number][]  // ✅ From engine
}
```

**For Raw Inputs:**

```typescript
interface RawInputResponse {
  // ... existing fields ...
  calculated_power: number       // ✅ From engine
  calculated_output_rate: number // ✅ From engine (considering overclock)
}
```

---

### Solution 2: Real-time Preview API (For Forms)

When user changes form values, call backend for live preview:

**For Production Line Form:**
```typescript
// ❌ Current: Frontend calculates
const calculatedPower = computed(() => {
  return formData.value.machine_groups.reduce(/* formula */)
})

// ✅ Better: Backend calculates
const calculatedPower = ref(0)

// Debounced API call when form changes
watch(() => formData.value, async (newData) => {
  const preview = await api.post('/production-lines/preview', newData)
  calculatedPower.value = preview.total_power
}, { debounce: 300 })
```

**For Power Generator Form:**
```typescript
// ✅ Backend provides preview
const preview = await api.post('/power-generators/preview', {
  generator_type: formData.value.generator_type,
  fuel_type: formData.value.fuel_type,
  groups: formData.value.groups
})

calculatedPower.value = preview.total_power_generation
calculatedFuelRate.value = preview.total_fuel_consumption
```

---

## Implementation Priority

### Phase 1: High Priority (Wrong Data) 🔴

1. **ProductionLineList.vue** - getPowerConsumption()
   - Currently shows WRONG power (assumes 4 MW for all machines)
   - Backend should include `total_power` in response

2. **PowerGeneratorForm.vue** & **PowerGeneratorList.vue**
   - Remove hardcoded base power and fuel rates
   - Backend should calculate and return these values

3. **RawInputList.vue** & **RawInputForm.vue**
   - Remove hardcoded extractor power values
   - Backend should provide calculated power

### Phase 2: Medium Priority (Duplicate Logic) 🟡

4. **ProductionLineForm.vue**
   - Currently uses backend data + frontend formula
   - Add preview API endpoint for real-time calculation

### Phase 3: Low Priority (Cleanup) 🟢

5. Remove all `Math.pow(oc, 1.321928)` from frontend
6. Remove all hardcoded game constants
7. Add backend validation that ensures all calculations come from engine

---

## Testing Strategy

After implementing backend calculations:

### Backend Tests
```rust
#[test]
fn test_power_generator_total_power() {
    let generator = /* create test generator */;
    assert_eq!(generator.total_power(), 600.0);
}

#[test]
fn test_production_line_power_with_overclock() {
    let line = /* create with 150% overclock */;
    assert_eq!(line.total_power(), 67.3);  // Exact value from game
}
```

### Frontend Tests
```typescript
it('displays power from backend response', async () => {
  const mockResponse = { total_power: 67.3 }
  // Verify frontend displays 67.3, doesn't calculate it
})

it('does not perform power calculations', () => {
  const code = fs.readFileSync('ProductionLineList.vue')
  expect(code).not.toContain('Math.pow')  // No power formulas
  expect(code).not.toContain('1.321928')  // No game constants
})
```

---

## Benefits After Refactoring

1. **Accuracy**: All calculations match game exactly
2. **Maintainability**: One place to update when game changes
3. **Type Safety**: Rust compiler catches errors
4. **Performance**: Backend calculations can be cached
5. **Consistency**: Form preview matches final saved values
6. **Trust**: Users see correct values, builds trust in the tool

---

## Migration Checklist

- [ ] Add `total_power` field to `ProductionLineResponse`
- [ ] Add `total_power_generation` and `total_fuel_consumption` to `PowerGeneratorResponse`
- [ ] Add `calculated_power` to `RawInputResponse`
- [ ] Create `/production-lines/preview` endpoint
- [ ] Create `/power-generators/preview` endpoint
- [ ] Create `/raw-inputs/preview` endpoint
- [ ] Update `ProductionLineList.vue` to use backend power
- [ ] Update `PowerGeneratorForm.vue` to use preview API
- [ ] Update `PowerGeneratorList.vue` to use backend values
- [ ] Update `ProductionLineForm.vue` to use preview API
- [ ] Update `RawInputList.vue` to use backend power
- [ ] Update `RawInputForm.vue` to use preview API
- [ ] Remove all hardcoded game constants from frontend
- [ ] Remove all `Math.pow(*, 1.321928)` from frontend
- [ ] Add backend tests for all calculations
- [ ] Add frontend tests to prevent future violations
- [ ] Document API endpoints in OpenAPI spec

---

## Conclusion

**Current State**: Frontend has game logic scattered across 6+ components with hardcoded values and duplicate formulas.

**Target State**: Frontend is a "dumb" UI layer that displays backend-calculated values. Engine is the single source of truth.

**Impact**: This refactoring will make the codebase:
- More maintainable
- More accurate
- More testable
- More aligned with the original architecture goals

The engine (Rust) was built to be the source of truth. Let's use it that way!
