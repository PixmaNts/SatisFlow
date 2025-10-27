/* eslint-disable @typescript-eslint/no-explicit-any */
import { test, expect } from '@playwright/test';
import type { Page } from '@playwright/test';

interface ProductionLineConfig {
  name: string;
  recipe: string;
  machineGroups: Array<{
    number_of_machine: number;
    oc_value: number;
    somersloop: number;
  }>;
}

/**
 * Calculation Accuracy E2E Tests
 *
 * Tests that all power consumption, generation, and fuel calculations
 * display correctly and match expected game values from the backend engine.
 */

test.describe('Calculation Accuracy Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard and wait for it to load
    await page.goto('/');
    await page.waitForSelector('.dashboard-view', { timeout: 10000 });

    // Ensure we have a clean state by resetting if needed
    await resetEngineState(page);
  });

  test.describe('Production Line Power Calculations', () => {
    test('should display correct power consumption for Constructor (4MW base)', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Iron Processing');

      // Create a production line with Constructor (Iron Ingot recipe)
      await createProductionLine(page, factoryId, {
        name: 'Iron Ingot Production',
        recipe: 'IronIngot',
        machineGroups: [{
          number_of_machine: 4,
          oc_value: 100.0,
          somersloop: 0
        }]
      });

      // Verify power consumption is calculated correctly
      // Constructor: 4MW base * 4 machines = 16MW at 100% clock
      await expectPowerConsumption(page, '16.0 MW');
    });

    test('should display correct power consumption for Assembler (16MW base)', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Electronics Factory');

      // Create a production line with Assembler (Circuit Board recipe)
      await createProductionLine(page, factoryId, {
        name: 'Circuit Board Production',
        recipe: 'CircuitBoard',
        machineGroups: [{
          number_of_machine: 2,
          oc_value: 100.0,
          somersloop: 0
        }]
      });

      // Verify power consumption is calculated correctly
      // Assembler: 16MW base * 2 machines = 32MW at 100% clock
      await expectPowerConsumption(page, '32.0 MW');
    });

    test('should display correct power consumption with overclock', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Steel Production');

      // Create a production line with overclock
      await createProductionLine(page, factoryId, {
        name: 'Steel Ingot Production',
        recipe: 'SteelIngot',
        machineGroups: [{
          number_of_machine: 3,
          oc_value: 150.0, // 150% overclock
          somersloop: 0
        }]
      });

      // Verify power consumption with overclock calculation
      // Foundry: 16MW base * (1.5)^1.321928 * 3 machines ≈ 16 * 1.89 * 3 ≈ 90.7MW
      await expectPowerConsumption(page, '90.7 MW');
    });

    test('should display correct power consumption with somersloop', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Advanced Production');

      // Create a production line with somersloop
      await createProductionLine(page, factoryId, {
        name: 'Heavy Modular Frame Production',
        recipe: 'HeavyModularFrame',
        machineGroups: [{
          number_of_machine: 2,
          oc_value: 100.0,
          somersloop: 2 // Manufacturer allows up to 4 somersloop
        }]
      });

      // Verify power consumption with somersloop calculation
      // Manufacturer: 32MW base * (1 + 2/4)² * 2 machines = 32 * (1.5)² * 2 = 32 * 2.25 * 2 = 144MW
      await expectPowerConsumption(page, '144.0 MW');
    });

    test('should display correct power consumption for blueprint production lines', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Blueprint Factory');

      // Create a blueprint production line (multiple recipes)
      await createBlueprintProductionLine(page, factoryId, {
        name: 'Motor Production Blueprint',
        productionLines: [
          {
            name: 'Rotor Production',
            recipe: 'Rotor',
            machineGroups: [{
              number_of_machine: 2,
              oc_value: 100.0,
              somersloop: 0
            }]
          },
          {
            name: 'Stator Production',
            recipe: 'Stator',
            machineGroups: [{
              number_of_machine: 2,
              oc_value: 100.0,
              somersloop: 0
            }]
          }
        ]
      });

      // Verify total power consumption for blueprint
      // Rotor: Assembler 16MW * 2 = 32MW
      // Stator: Assembler 16MW * 2 = 32MW
      // Total: 64MW
      await expectPowerConsumption(page, '64.0 MW');
    });
  });

  test.describe('Power Generator Calculations', () => {
    test('should display correct power generation for Coal Generator (75MW base)', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Power Plant');

      // Create a coal generator
      await createPowerGenerator(page, factoryId, {
        generator_type: 'Coal',
        fuel_type: 'Coal',
        groups: [{
          number_of_generators: 4,
          clock_speed: 100.0
        }]
      });

      // Verify power generation is calculated correctly
      // Coal Generator: 75MW base * 4 generators = 300MW at 100% clock
      await expectPowerGeneration(page, '300.0 MW');
    });

    test('should display correct fuel consumption for Coal Generator', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Power Plant');

      // Create a coal generator
      await createPowerGenerator(page, factoryId, {
        generator_type: 'Coal',
        fuel_type: 'Coal',
        groups: [{
          number_of_generators: 2,
          clock_speed: 100.0
        }]
      });

      // Verify fuel consumption is calculated correctly
      // Coal Generator: 15 items/min base * 2 generators = 30 items/min at 100% clock
      await expectFuelConsumption(page, '30.00/min');
    });

    test('should display correct fuel efficiency for Compacted Coal', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Efficient Power');

      // Create a coal generator with compacted coal (more efficient)
      await createPowerGenerator(page, factoryId, {
        generator_type: 'Coal',
        fuel_type: 'CompactedCoal',
        groups: [{
          number_of_generators: 3,
          clock_speed: 100.0
        }]
      });

      // Verify fuel consumption with efficiency multiplier
      // Compacted Coal: 15 * 0.8 * 3 = 36 items/min (vs 45 for regular coal)
      await expectFuelConsumption(page, '36.00/min');

      // Power generation should be the same (75MW * 3 = 225MW)
      await expectPowerGeneration(page, '225.0 MW');
    });

    test('should display correct fuel efficiency for Turbofuel', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Turbo Power');

      // Create a fuel generator with turbofuel (4x more efficient)
      await createPowerGenerator(page, factoryId, {
        generator_type: 'Fuel',
        fuel_type: 'Turbofuel',
        groups: [{
          number_of_generators: 1,
          clock_speed: 100.0
        }]
      });

      // Verify fuel consumption with turbofuel efficiency
      // Turbofuel: 4.5 * 0.25 * 1 = 1.125 m³/min (vs 4.5 for regular fuel)
      await expectFuelConsumption(page, '1.13/min');

      // Power generation should be the same (150MW)
      await expectPowerGeneration(page, '150.0 MW');
    });

    test('should display correct power generation with overclock', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Overclocked Power');

      // Create a fuel generator with overclock
      await createPowerGenerator(page, factoryId, {
        generator_type: 'Fuel',
        fuel_type: 'Fuel',
        groups: [{
          number_of_generators: 2,
          clock_speed: 200.0 // 200% overclock
        }]
      });

      // Verify power generation with overclock
      // Fuel Generator: 150MW * 2.0 * 2 generators = 600MW at 200% clock
      await expectPowerGeneration(page, '600.0 MW');

      // Fuel consumption should also scale: 4.5 * 2.0 * 2 = 18 m³/min
      await expectFuelConsumption(page, '18.00/min');
    });

    test('should display correct nuclear waste production', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Nuclear Plant');

      // Create a nuclear generator
      await createPowerGenerator(page, factoryId, {
        generator_type: 'Nuclear',
        fuel_type: 'UraniumFuelRod',
        groups: [{
          number_of_generators: 1,
          clock_speed: 100.0
        }]
      });

      // Verify nuclear waste production
      // Nuclear: 0.025 waste rods/min * 1 generator = 0.025/min
      await expectWasteProduction(page, '0.03/min');
    });

    test('should display zero fuel consumption for Geothermal', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Geothermal Plant');

      // Create a geothermal generator (no fuel)
      await createPowerGenerator(page, factoryId, {
        generator_type: 'Geothermal',
        fuel_type: null,
        groups: [{
          number_of_generators: 5,
          clock_speed: 100.0
        }]
      });

      // Verify no fuel consumption
      await expectFuelConsumption(page, '0.00/min');

      // But still generates power: 200MW * 5 = 1000MW
      await expectPowerGeneration(page, '1,000.0 MW');
    });
  });

  test.describe('Raw Input Power Calculations', () => {
    test('should display correct power consumption for Miner Mk1 (5MW)', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Mining Outpost');

      // Create a raw input with Miner Mk1
      await createRawInput(page, factoryId, {
        extractor_type: 'MinerMk1',
        item: 'IronOre',
        purity: 'Normal'
      });

      // Verify power consumption
      // Miner Mk1: 5MW base power
      await expectRawInputPower(page, '5.0 MW');
    });

    test('should display correct power consumption for Miner Mk2 (15MW)', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Advanced Mining');

      // Create a raw input with Miner Mk2
      await createRawInput(page, factoryId, {
        extractor_type: 'MinerMk2',
        item: 'CopperOre',
        purity: 'Pure' // 2x multiplier
      });

      // Verify power consumption
      // Miner Mk2: 15MW base power
      await expectRawInputPower(page, '15.0 MW');
    });

    test('should display correct power consumption for Miner Mk3 (45MW)', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Mega Mining');

      // Create a raw input with Miner Mk3
      await createRawInput(page, factoryId, {
        extractor_type: 'MinerMk3',
        item: 'Coal',
        purity: 'Impure' // 0.5x multiplier
      });

      // Verify power consumption
      // Miner Mk3: 45MW base power
      await expectRawInputPower(page, '45.0 MW');
    });

    test('should display correct power consumption for Water Extractor (20MW)', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Water Plant');

      // Create a raw input with Water Extractor (no purity)
      await createRawInput(page, factoryId, {
        extractor_type: 'WaterExtractor',
        item: 'Water',
        purity: null
      });

      // Verify power consumption
      // Water Extractor: 20MW base power
      await expectRawInputPower(page, '20.0 MW');
    });

    test('should display correct power consumption for Oil Extractor (40MW)', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Oil Field');

      // Create a raw input with Oil Extractor
      await createRawInput(page, factoryId, {
        extractor_type: 'OilExtractor',
        item: 'CrudeOil',
        purity: 'Normal'
      });

      // Verify power consumption
      // Oil Extractor: 40MW base power
      await expectRawInputPower(page, '40.0 MW');
    });

    test('should display correct power consumption for Resource Well Pressurizer', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Gas Field');

      // Create a resource well system with pressurizer
      await createResourceWellInput(page, factoryId, {
        item: 'NitrogenGas',
        pressurizer: {
          clock_speed: 100.0
        },
        extractors: [
          { purity: 'Normal' },
          { purity: 'Pure' }
        ]
      });

      // Verify power consumption for pressurizer
      // Resource Well Pressurizer: 150MW * (1.0)^1.321928 ≈ 150MW at 100% clock
      await expectRawInputPower(page, '150.0 MW');
    });

    test('should display correct power consumption for Resource Well with overclock', async ({ page }) => {
      const factoryId = await createTestFactory(page, 'Overclocked Gas');

      // Create a resource well system with overclocked pressurizer
      await createResourceWellInput(page, factoryId, {
        item: 'CrudeOil',
        pressurizer: {
          clock_speed: 200.0 // 200% overclock
        },
        extractors: [
          { purity: 'Normal' }
        ]
      });

      // Verify power consumption with overclock
      // Resource Well Pressurizer: 150MW * (2.0)^1.321928 ≈ 150 * 2.5 = 375MW
      await expectRawInputPower(page, '375.0 MW');
    });
  });

  test.describe('Preview Endpoint Calculations', () => {
    test('should show real-time power preview during production line form editing', async ({ page }) => {
      await page.goto('/factory');

      // Select a factory
      await page.waitForSelector('[data-test="factory-selector"]');
      await page.selectOption('[data-test="factory-selector"]', '1');

      // Click to add production line
      await page.click('[data-test="create-production-line"]');

      // Fill out the form
      await page.fill('[data-test="name-input"]', 'Test Production Line');
      await page.selectOption('[data-test="recipe-select"]', 'IronIngot');

      // Add machine group
      await page.fill('[data-test="machines-input"]', '4');
      await page.fill('[data-test="oc-input"]', '100');

      // Check that preview shows correct power consumption
      // Should show 16MW (4 machines * 4MW Constructor base)
      await expect(page.locator('[data-test="power-preview"]')).toContainText('16.0 MW');
    });

    test('should show real-time fuel preview during power generator form editing', async ({ page }) => {
      await page.goto('/factory');

      // Select a factory
      await page.waitForSelector('[data-test="factory-selector"]');
      await page.selectOption('[data-test="factory-selector"]', '1');

      // Click to add power generator
      await page.click('[data-test="create-power-generator"]');

      // Select generator type
      await page.selectOption('[data-test="generator-type-select"]', 'Coal');

      // Select fuel type
      await page.selectOption('[data-test="fuel-type-select"]', 'Coal');

      // Add generator group
      await page.fill('[data-test="generators-input"]', '3');
      await page.fill('[data-test="clock-speed-input"]', '100');

      // Check that preview shows correct power generation and fuel consumption
      await expect(page.locator('[data-test="power-preview"]')).toContainText('225.0 MW');
      await expect(page.locator('[data-test="fuel-preview"]')).toContainText('45.00/min');
    });

    test('should show real-time power preview during raw input form editing', async ({ page }) => {
      await page.goto('/factory');

      // Select a factory
      await page.waitForSelector('[data-test="factory-selector"]');
      await page.selectOption('[data-test="factory-selector"]', '1');

      // Click to add raw input
      await page.click('[data-test="create-raw-input"]');

      // Select extractor type
      await page.selectOption('[data-test="extractor-type-select"]', 'MinerMk2');

      // Select item
      await page.selectOption('[data-test="item-select"]', 'IronOre');

      // Select purity
      await page.selectOption('[data-test="purity-select"]', 'Normal');

      // Check that preview shows correct power consumption
      // Miner Mk2: 15MW base power
      await expect(page.locator('[data-test="power-preview"]')).toContainText('15.0 MW');
    });
  });

  test.describe('Form Preview vs Final Values', () => {
    test('should match preview values with final saved production line values', async ({ page }) => {
      await page.goto('/factory');

      // Select a factory
      await page.waitForSelector('[data-test="factory-selector"]');
      await page.selectOption('[data-test="factory-selector"]', '1');

      // Click to add production line
      await page.click('[data-test="create-production-line"]');

      // Fill out the form
      const lineName = 'Preview Test Line';
      await page.fill('[data-test="name-input"]', lineName);
      await page.selectOption('[data-test="recipe-select"]', 'IronIngot');
      await page.fill('[data-test="machines-input"]', '6');
      await page.fill('[data-test="oc-input"]', '150');
      await page.fill('[data-test="somersloop-input"]', '1');

      // Capture preview values
      const previewPower = await page.locator('[data-test="power-preview"]').textContent();

      // Submit the form
      await page.click('[data-test="submit-btn"]');

      // Wait for the production line to appear in the list
      await page.waitForSelector(`text=${lineName}`);

      // Verify the final values match the preview
      await expect(page.locator('.production-line-list').locator(`text=${lineName}`).locator('..').locator('.power-value')).toContainText(previewPower || '');
    });

    test('should match preview values with final saved power generator values', async ({ page }) => {
      await page.goto('/factory');

      // Select a factory
      await page.waitForSelector('[data-test="factory-selector"]');
      await page.selectOption('[data-test="factory-selector"]', '1');

      // Click to add power generator
      await page.click('[data-test="create-power-generator"]');

      // Fill out the form
      await page.selectOption('[data-test="generator-type-select"]', 'Fuel');
      await page.selectOption('[data-test="fuel-type-select"]', 'Turbofuel');
      await page.fill('[data-test="generators-input"]', '2');
      await page.fill('[data-test="clock-speed-input"]', '200');

      // Capture preview values
      const previewPower = await page.locator('[data-test="power-preview"]').textContent();
      const previewFuel = await page.locator('[data-test="fuel-preview"]').textContent();

      // Submit the form
      await page.click('[data-test="submit-btn"]');

      // Wait for the generator to appear in the list
      await page.waitForSelector('text=Fuel Generator');

      // Verify the final values match the preview
      await expect(page.locator('.power-generator-list').locator('.power-value')).toContainText(previewPower || '');
      await expect(page.locator('.power-generator-list').locator('.fuel-rate')).toContainText(previewFuel || '');
    });
  });

  test.describe('Expected Game Values Verification', () => {
    test('should match all machine base power values', async ({ page }) => {
      // Test all machine types have correct base power consumption
      const machineTests = [
        { recipe: 'IronIngot', expectedPower: 4.0, machine: 'Constructor' },
        { recipe: 'CircuitBoard', expectedPower: 16.0, machine: 'Assembler' },
        { recipe: 'HeavyModularFrame', expectedPower: 32.0, machine: 'Manufacturer' },
        { recipe: 'SteelIngot', expectedPower: 16.0, machine: 'Foundry' },
        { recipe: 'Plastic', expectedPower: 16.0, machine: 'Refinery' },
        { recipe: 'Motor', expectedPower: 32.0, machine: 'Blender' },
        { recipe: 'PackagedWater', expectedPower: 4.0, machine: 'Packager' },
        { recipe: 'UraniumFuelRod', expectedPower: 64.0, machine: 'ParticleAccelerator' }
      ];

      for (const testCase of machineTests) {
        const factoryId = await createTestFactory(page, `${testCase.machine} Test`);

        await createProductionLine(page, factoryId, {
          name: `${testCase.machine} Test Line`,
          recipe: testCase.recipe,
          machineGroups: [{
            number_of_machine: 1,
            oc_value: 100.0,
            somersloop: 0
          }]
        });

        await expectPowerConsumption(page, `${testCase.expectedPower} MW`);
      }
    });

    test('should match all generator base power outputs', async ({ page }) => {
      // Test all generator types have correct base power output
      const generatorTests = [
        { type: 'Biomass', fuel: 'Biomass', expectedPower: 30.0 },
        { type: 'Coal', fuel: 'Coal', expectedPower: 75.0 },
        { type: 'Fuel', fuel: 'Fuel', expectedPower: 150.0 },
        { type: 'Nuclear', fuel: 'UraniumFuelRod', expectedPower: 2500.0 },
        { type: 'Geothermal', fuel: null, expectedPower: 200.0 }
      ];

      for (const testCase of generatorTests) {
        const factoryId = await createTestFactory(page, `${testCase.type} Test`);

        await createPowerGenerator(page, factoryId, {
          generator_type: testCase.type,
          fuel_type: testCase.fuel,
          groups: [{
            number_of_generators: 1,
            clock_speed: 100.0
          }]
        });

        await expectPowerGeneration(page, `${testCase.expectedPower} MW`);
      }
    });

    test('should match all extractor base power consumptions', async ({ page }) => {
      // Test all extractor types have correct base power consumption
      const extractorTests = [
        { type: 'MinerMk1', item: 'IronOre', purity: 'Normal', expectedPower: 5.0 },
        { type: 'MinerMk2', item: 'CopperOre', purity: 'Normal', expectedPower: 15.0 },
        { type: 'MinerMk3', item: 'Coal', purity: 'Normal', expectedPower: 45.0 },
        { type: 'WaterExtractor', item: 'Water', purity: null, expectedPower: 20.0 },
        { type: 'OilExtractor', item: 'CrudeOil', purity: 'Normal', expectedPower: 40.0 }
      ];

      for (const testCase of extractorTests) {
        const factoryId = await createTestFactory(page, `${testCase.type} Test`);

        await createRawInput(page, factoryId, {
          extractor_type: testCase.type,
          item: testCase.item,
          purity: testCase.purity
        });

        await expectRawInputPower(page, `${testCase.expectedPower} MW`);
      }
    });
  });
});

// Helper functions for tests

async function resetEngineState(page: Page): Promise<void> {
  // Reset the engine to ensure clean state
  await page.evaluate(async () => {
    try {
      await fetch('/api/reset', { method: 'POST' });
    } catch {
      console.log('Reset endpoint not available, continuing...');
    }
  });
}

async function createTestFactory(page: Page, name: string): Promise<string> {
  const response = await page.evaluate(async (factoryName) => {
    const response = await fetch('/api/factories', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name: factoryName })
    });
    const result = await response.json();
    return result.id;
  }, name);

  return response.toString();
}

async function createProductionLine(page: Page, factoryId: string, config: {
  name: string;
  recipe: string;
  machineGroups: Array<{
    number_of_machine: number;
    oc_value: number;
    somersloop: number;
  }>;
}): Promise<void> {
  await page.evaluate(async ({ factoryId, config }: { factoryId: string; config: ProductionLineConfig }) => {
    await fetch(`/api/factories/${factoryId}/production-lines`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        name: config.name,
        type: 'recipe',
        recipe: config.recipe,
        machine_groups: config.machineGroups
      })
    });
  }, { factoryId, config });
}

async function createBlueprintProductionLine(page: Page, factoryId: string, config: {
  name: string;
  productionLines: Array<{
    name: string;
    recipe: string;
    machineGroups: Array<{
      number_of_machine: number;
      oc_value: number;
      somersloop: number;
    }>;
  }>;
}): Promise<void> {
  await page.evaluate(async ({ factoryId, config }: { factoryId: string; config: any }) => {
    await fetch(`/api/factories/${factoryId}/production-lines`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        name: config.name,
        type: 'blueprint',
        production_lines: config.productionLines.map((line: any) => ({
          name: line.name,
          recipe: line.recipe,
          machine_groups: line.machineGroups
        }))
      })
    });
  }, { factoryId, config });
}

async function createPowerGenerator(page: Page, factoryId: string, config: {
  generator_type: string;
  fuel_type: string | null;
  groups: Array<{
    number_of_generators: number;
    clock_speed: number;
  }>;
}): Promise<void> {
  await page.evaluate(async ({ factoryId, config }: { factoryId: string; config: any }) => {
    await fetch(`/api/factories/${factoryId}/power-generators`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        generator_type: config.generator_type,
        fuel_type: config.fuel_type,
        groups: config.groups
      })
    });
  }, { factoryId, config });
}

async function createRawInput(page: Page, factoryId: string, config: {
  extractor_type: string;
  item: string;
  purity: string | null;
}): Promise<void> {
  await page.evaluate(async ({ factoryId, config }: { factoryId: string; config: any }) => {
    await fetch(`/api/factories/${factoryId}/raw-inputs`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        extractor_type: config.extractor_type,
        item: config.item,
        purity: config.purity
      })
    });
  }, { factoryId, config });
}

async function createResourceWellInput(page: Page, factoryId: string, config: {
  item: string;
  pressurizer: {
    clock_speed: number;
  };
  extractors: Array<{
    purity: string;
  }>;
}): Promise<void> {
  await page.evaluate(async ({ factoryId, config }: { factoryId: string; config: any }) => {
    await fetch(`/api/factories/${factoryId}/raw-inputs`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        extractor_type: 'ResourceWellExtractor',
        item: config.item,
        pressurizer: {
          id: 1,
          clock_speed: config.pressurizer.clock_speed
        },
        extractors: config.extractors.map((ext: any, index: number) => ({
          id: index + 1,
          purity: ext.purity,
          item: config.item
        }))
      })
    });
  }, { factoryId, config });
}

async function expectPowerConsumption(page: Page, expected: string): Promise<void> {
  // Wait for the production line list to load and check power values
  await page.waitForSelector('.production-line-list');
  await expect(page.locator('.production-line-list .power-value')).toContainText(expected);
}

async function expectPowerGeneration(page: Page, expected: string): Promise<void> {
  await page.waitForSelector('.power-generator-list .power-value');
  await expect(page.locator('.power-generator-list .power-value')).toContainText(expected);
}

async function expectFuelConsumption(page: Page, expected: string): Promise<void> {
  await page.waitForSelector('.power-generator-list .fuel-rate');
  await expect(page.locator('.power-generator-list .fuel-rate')).toContainText(expected);
}

async function expectWasteProduction(page: Page, expected: string): Promise<void> {
  // Nuclear generators show waste production in the UI
  await page.waitForSelector('.power-generator-list');
  await expect(page.locator('.power-generator-list')).toContainText(expected);
}

async function expectRawInputPower(page: Page, expected: string): Promise<void> {
  await page.waitForSelector('.raw-input-list .power-value');
  await expect(page.locator('.raw-input-list .power-value')).toContainText(expected);
}
