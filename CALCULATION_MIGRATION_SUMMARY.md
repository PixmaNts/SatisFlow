# Calculation Migration Summary

## Quick Overview

**Problem**: Frontend has game calculations that should only exist in the Rust engine.

**Solution**: Add calculated fields to API responses and create preview endpoints for real-time form updates.

---

## Critical Issues to Fix

### 🔴 Immediate (Wrong Data Displayed)

1. **ProductionLineList.vue:295-308** - Assumes all machines use 4 MW (WRONG!)
2. **PowerGeneratorForm.vue:249-286** - Hardcoded base power & fuel rates
3. **PowerGeneratorList.vue:209-242** - Duplicate hardcoded values
4. **RawInputList.vue:224-239** - Hardcoded extractor power values
5. **RawInputForm.vue:443** - Frontend calculates placeholder power

### 🟡 Medium Priority (Duplicate Logic)

6. **ProductionLineForm.vue:245-256** - Uses backend data but applies frontend formulas

---

## Missing API Endpoints

### Phase 1: Add Calculated Fields to Existing Endpoints

| Endpoint | New Fields Needed |
|----------|-------------------|
| `GET /api/factories/:id/production-lines` | `total_power_consumption`, `total_machines`, `total_somersloop`, `input_items`, `output_items` |
| `GET /api/factories/:id/power-generators` | `total_power_generation`, `total_fuel_consumption`, `waste_production_rate`, `base_power_output`, `base_fuel_consumption` |
| `GET /api/factories/:id/raw-inputs` | `calculated_power_consumption`, `base_power_consumption` |

### Phase 2: Create Preview Endpoints (For Real-Time Form Updates)

| Endpoint | Purpose |
|----------|---------|
| `POST /api/factories/:id/production-lines/preview` | Calculate power/rates as user edits form |
| `POST /api/factories/:id/power-generators/preview` | Calculate generation/consumption in real-time |
| `POST /api/factories/:id/raw-inputs/preview` | Calculate power/extraction in real-time |

---

## Step-by-Step Implementation Plan

### Week 1: Backend - Add Calculated Fields

**File**: `crates/satisflow-server/src/handlers/factory.rs`

1. **Update ProductionLineResponse**
   ```rust
   #[derive(Serialize)]
   pub struct ProductionLineResponse {
       // ... existing fields
       pub total_power_consumption: f32,
       pub total_machines: u32,
       pub total_somersloop: u32,
       pub input_items: Vec<(String, f32)>,
       pub output_items: Vec<(String, f32)>,
   }
   ```

2. **Update PowerGeneratorResponse**
   ```rust
   #[derive(Serialize)]
   pub struct PowerGeneratorResponse {
       // ... existing fields
       pub total_power_generation: f32,
       pub total_fuel_consumption: f32,
       pub waste_production_rate: f32,
       pub base_power_output: f32,
       pub base_fuel_consumption: f32,
   }
   ```

3. **Update RawInputResponse**
   ```rust
   #[derive(Serialize)]
   pub struct RawInputResponse {
       // ... existing fields
       pub calculated_power_consumption: f32,
       pub base_power_consumption: f32,
   }
   ```

4. **Populate from engine methods**
   - Use `ProductionLine::total_power_consumption()`
   - Use `PowerGenerator::total_power_generation()`
   - Use `RawInput::power_consumption()`

---

### Week 2: Backend - Preview Endpoints

**File**: `crates/satisflow-server/src/handlers/factory.rs`

1. **Production Line Preview**
   ```rust
   #[derive(Deserialize)]
   struct ProductionLinePreviewRequest {
       recipe: String,
       machine_groups: Vec<MachineGroupPayload>,
   }

   #[derive(Serialize)]
   struct ProductionLinePreviewResponse {
       total_power_consumption: f32,
       total_machines: u32,
       input_items: Vec<(String, f32)>,
       output_items: Vec<(String, f32)>,
       valid: bool,
       validation_errors: Option<Vec<String>>,
   }

   async fn preview_production_line(
       Path(factory_id): Path<Uuid>,
       Json(req): Json<ProductionLinePreviewRequest>
   ) -> Result<Json<ProductionLinePreviewResponse>> {
       // Create temporary ProductionLineRecipe
       // Calculate using engine methods
       // Return without saving
   }
   ```

2. **Power Generator Preview** (similar structure)

3. **Raw Input Preview** (similar structure)

4. **Add routes**
   ```rust
   Router::new()
       .route("/:id/production-lines/preview", post(preview_production_line))
       .route("/:id/power-generators/preview", post(preview_power_generator))
       .route("/:id/raw-inputs/preview", post(preview_raw_input))
   ```

---

### Week 3: Frontend - Fix Critical Issues

#### 1. ProductionLineList.vue

**Remove (Lines 295-308)**:
```typescript
const getPowerConsumption = (line: ProductionLineResponse): number => {
  if (isProductionLineRecipe(line)) {
    return line.ProductionLineRecipe.machine_groups.reduce(
      (total: number, group: MachineGroup) =>
        total + (group.number_of_machine * 4 * Math.pow(group.oc_value / 100, 1.321928)),
      0
    )
  }
}
```

**Replace with**:
```typescript
const getPowerConsumption = (line: ProductionLineResponse): number => {
  return line.total_power_consumption  // ✅ From API
}
```

---

#### 2. PowerGeneratorForm.vue

**Remove (Lines 249-286)**:
```typescript
const basePower: Record<GeneratorType, number> = {
  Biomass: 30,
  Coal: 75,
  // ... etc
}

const calculatedPower = computed(() => {
  return formData.value.groups.reduce((total, group) => {
    // ... manual calculation
  }, 0)
})
```

**Replace with**:
```typescript
const calculatedPower = ref(0)
const calculatedFuelRate = ref(0)

const updatePreview = debounce(async () => {
  const preview = await api.post(
    `/factories/${factoryId}/power-generators/preview`,
    formData.value
  )
  calculatedPower.value = preview.total_power_generation
  calculatedFuelRate.value = preview.total_fuel_consumption
}, 300)

watch(() => formData.value, updatePreview, { deep: true })
```

---

#### 3. PowerGeneratorList.vue

**Remove**: All hardcoded `basePower` and `baseFuelRate` objects

**Use**: API-provided fields directly
```typescript
<div>{{ generator.total_power_generation }} MW</div>
<div>{{ generator.total_fuel_consumption }} items/min</div>
```

---

#### 4. RawInputList.vue

**Remove (Lines 224-239)**:
```typescript
const basePower: Record<ExtractorType, number> = {
  MinerMk1: 5,
  // ... etc
}
```

**Replace with**:
```typescript
const getPower = (input: RawInputResponse): number => {
  return input.calculated_power_consumption
}
```

---

#### 5. RawInputForm.vue

**Remove (Line 443)**:
```typescript
power_consumption: 150 * Math.pow(1, 1.321928)
```

**Replace with**:
```typescript
const calculatedPower = ref(0)

const updatePreview = debounce(async () => {
  const preview = await api.post(
    `/factories/${factoryId}/raw-inputs/preview`,
    formData.value
  )
  calculatedPower.value = preview.power_consumption
}, 300)
```

---

#### 6. ProductionLineForm.vue

**Remove (Lines 245-256)**: Manual power calculation with `Math.pow()`

**Replace with**: Preview API call (same pattern as others)

---

## Testing Checklist

### Backend Tests
- [ ] Verify all GET endpoints return calculated fields
- [ ] Test preview endpoints with various inputs
- [ ] Verify calculations match engine methods
- [ ] Test error handling (invalid inputs)

### Frontend Tests
- [ ] No `Math.pow()` calls in code
- [ ] No `1.321928` constant in code
- [ ] No hardcoded base power values
- [ ] Preview updates with debouncing
- [ ] Correct values displayed in lists

### Integration Tests
- [ ] Production line power matches engine
- [ ] Generator power/fuel matches engine
- [ ] Raw input power matches engine
- [ ] Form previews match saved values

---

## Before/After Comparison

### Before (Current State)
```typescript
// ❌ Frontend has game logic
const power = machines * 4 * Math.pow(oc / 100, 1.321928)
const basePower = { Coal: 75, Fuel: 150 }
```

### After (Target State)
```typescript
// ✅ Frontend displays backend data
const power = line.total_power_consumption
const generation = generator.total_power_generation
```

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Hardcoded game constants in frontend | 0 |
| `Math.pow()` calls in frontend | 0 |
| API response time for preview | < 100ms |
| Frontend test coverage | > 80% |
| Backend test coverage | > 90% |

---

## Quick Reference: Engine Methods Already Available

```rust
// ProductionLine
impl ProductionLine {
    pub fn total_power_consumption(&self) -> f32  // ✅
    pub fn total_machines(&self) -> u32           // ✅
    pub fn input_rate(&self) -> Vec<(Item, f32)>  // ✅
    pub fn output_rate(&self) -> Vec<(Item, f32)> // ✅
}

// PowerGenerator
impl PowerGenerator {
    pub fn total_power_generation(&self) -> f32   // ✅
    pub fn total_fuel_consumption(&self) -> f32   // ✅
    pub fn waste_production_rate(&self) -> f32    // ✅
}

// RawInput
impl RawInput {
    pub fn power_consumption(&self) -> f32        // ✅
}
```

**Everything we need already exists in the engine!**

---

## Timeline

| Week | Focus | Deliverable |
|------|-------|-------------|
| 1 | Backend response fields | Updated DTOs with calculated fields |
| 2 | Backend preview endpoints | Three new POST endpoints |
| 3 | Frontend critical fixes | Lists show correct data |
| 4 | Frontend form previews | Real-time calculations via API |

**Total estimated time: 4 weeks**

---

## Key Principles

1. **Engine is source of truth** - All game data and calculations in Rust
2. **Frontend is display layer** - Only formats and displays data
3. **No duplicate logic** - Calculations exist in ONE place only
4. **Type safety** - Rust compiler catches errors
5. **Testability** - Engine logic fully tested
6. **API-driven** - Any UI can be built on top

---

## Next Steps

1. Review this plan with team
2. Create feature branch: `feature/calculation-migration`
3. Start with Week 1 backend changes
4. Deploy to staging for testing
5. Complete frontend refactoring
6. Comprehensive testing
7. Deploy to production

---

**Questions or concerns? See full details in [CALCULATION_MIGRATION_PLAN.md](./CALCULATION_MIGRATION_PLAN.md)**
