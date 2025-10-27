# Example Blueprint Files

This directory contains example blueprint JSON files that demonstrate the Blueprint Import/Export feature in Satisflow. These files can be used to test the import functionality and serve as templates for creating your own blueprints.

## Available Examples

### 1. **example-blueprint-iron-plates.json**
**Simple Iron Plate Production**

- **Complexity**: Beginner
- **Production Lines**: 2
- **Total Machines**: 5
- **Description**: Basic iron plate production line with standard smelters and constructors
- **Use Case**: Perfect for testing basic import functionality and understanding blueprint structure

**What it produces:**
- Iron ingots from iron ore
- Iron plates from iron ingots
- Output: ~60 iron plates/minute

**Specifications:**
- 2 Smelters for Iron Ingots (100% clock speed)
- 3 Constructors for Iron Plates (100% clock speed)
- No overclocking
- No Somersloops

---

### 2. **example-blueprint-reinforced-plates.json**
**Reinforced Iron Plates Factory**

- **Complexity**: Intermediate
- **Production Lines**: 4
- **Total Machines**: 17
- **Description**: Advanced production line demonstrating overclocking and power shard usage
- **Use Case**: Testing import with overclocked machines and complex production chains

**What it produces:**
- Reinforced Iron Plates for construction

**Specifications:**
- 4 Overclocked Smelters (150% clock speed) for Iron Ingots
- 6 Standard Constructors (100%) for Iron Plates
- 3 High-speed Constructors (200% + 1 Somersloop) for Screws
- 4 Assemblers (125% clock speed) for Reinforced Plates

**Features:**
- Mixed overclock settings (100%, 125%, 150%, 200%)
- Power shard usage (Somersloop count: 1 per machine in screw production)
- Multi-stage production chain

---

### 3. **example-blueprint-motor-production.json**
**Motor Production Complex**

- **Complexity**: Advanced
- **Production Lines**: 9
- **Total Machines**: 45
- **Description**: Complete end-to-end motor production facility with all prerequisites
- **Use Case**: Testing complex blueprints with multiple machine groups and production stages

**What it produces:**
- Motors (complete supply chain from raw materials)

**Specifications:**
- **Base Materials:**
  - 8 Smelters for Iron Ingots
  - 6 Smelters for Copper Ingots

- **Component Production:**
  - 4 Constructors for Iron Rods
  - 6 Overclocked Constructors (150% + 2 Somersloops) for Screws
  - 3 Constructors for Copper Wire
  - 6 Assemblers for Rotors (split across 2 production lines)
  - 4 Assemblers (110%) for Stators

- **Final Assembly:**
  - 8 Assemblers for Motors (mixed groups: 5 at 100%, 3 at 150% with Somersloop)

**Features:**
- Multiple machine groups in single production line
- Redundant production lines (2 separate rotor lines)
- Complex dependency chain
- Mixed overclocking strategies
- Strategic Somersloop placement

---

## Blueprint JSON Structure

Each blueprint follows this structure:

```json
{
  "id": "a1b2c3d4-1111-2222-3333-000000000001",
  "name": "Blueprint Display Name",
  "description": "Detailed description of what this blueprint does",
  "production_lines": [
    {
      "id": "b1b2c3d4-1111-2222-3333-000000000011",
      "name": "Production Line Name",
      "description": "What this line produces",
      "recipe": "RecipeName",
      "machine_groups": [
        {
          "number_of_machine": 4,
          "oc_value": 100.0,
          "somersloop": 0
        }
      ]
    }
  ]
}
```

### Field Descriptions

- **`id`** (UUID): Unique identifier for the blueprint (must be valid UUID format)
- **`name`** (string): Human-readable name shown in the UI
- **`description`** (string or null): Detailed description of the blueprint's purpose
- **`production_lines`** (array): Array of production line recipe configurations
  - **`id`** (UUID): Unique identifier for this production line
  - **`name`** (string): Display name for the production line
  - **`description`** (string or null): What this specific line does
  - **`recipe`** (string): Game recipe name (e.g., "IronIngot", "Motor")
  - **`machine_groups`** (array): Array of machine group configurations
    - **`number_of_machine`** (integer): How many machines in this group
    - **`oc_value`** (float): Overclock percentage (1.0 to 250.0, use 100.0 for normal)
    - **`somersloop`** (integer): Number of power shards/somersloops per machine (0-3)

---

## How to Use These Examples

### Import via UI

1. **Start the Satisflow application**
   ```bash
   cd frontend
   npm run dev
   ```

2. **Navigate to a Factory**
   - Select or create a factory from the dropdown

3. **Import Blueprint**
   - Click the "📥 Import Blueprint" button in the Production Lines section
   - Select one of the example JSON files
   - Review the preview modal showing:
     - Blueprint name and description
     - Total machines count
     - Power consumption estimate
     - Input/output items
   - Optionally override the blueprint name
   - Click "Import Blueprint" to confirm

4. **Verify Import**
   - The blueprint should appear in your production lines list
   - It will show as a Blueprint type (not a single recipe)
   - Click to expand and see all nested production lines

### Export Your Own Blueprints

1. Create a complex production setup in a factory
2. In the Production Lines list, find a blueprint production line
3. Click the "💾 Export" button next to the blueprint
4. A JSON file will download with format: `blueprint-{name}-{timestamp}.json`
5. Share this file with others or import it into different factories

---

## Testing Checklist

Use these examples to test the following scenarios:

- [ ] **Basic Import**: Import `example-blueprint-iron-plates.json`
  - Verify it shows 2 production lines
  - Verify total machine count is 5
  - Verify name displays correctly

- [ ] **Overclocking**: Import `example-blueprint-reinforced-plates.json`
  - Verify overclock values preserved (100%, 125%, 150%, 200%)
  - Verify Somersloop count shows correctly
  - Verify power consumption calculated for overclocked machines

- [ ] **Complex Blueprint**: Import `example-blueprint-motor-production.json`
  - Verify all 9 production lines imported
  - Verify multiple machine groups in motor assembly line
  - Verify total machine count is 45

- [ ] **Name Override**: Import any blueprint
  - Enter custom name in preview modal
  - Verify imported blueprint uses custom name

- [ ] **Cross-Factory**: Export from one factory, import to another
  - Verify blueprint structure preserved
  - Verify it functions independently

- [ ] **Error Handling**: Try importing invalid JSON
  - Verify error message displays
  - Verify app doesn't crash

---

## Creating Your Own Blueprints

### Manual Creation

1. Start with one of the examples as a template
2. Modify the `id`, `name`, and `description`
3. Update `production_lines` array with your recipes
4. Set appropriate `recipe` names (must match game recipes)
5. Configure `machine_groups` with your desired settings
6. Save as `.json` file

### Export from Satisflow

1. Build your production setup in the app
2. Save it as a blueprint production line
3. Export to JSON
4. Edit the JSON if needed
5. Re-import to other factories

---

## Common Recipe Names

Here are some common recipe names you can use:

**Basic Materials:**
- `IronIngot`, `CopperIngot`, `CateriumIngot`
- `Limestone`, `Concrete`
- `IronOre`, `CopperOre`, `Coal`

**Basic Components:**
- `IronRod`, `IronPlate`, `ReinforcedIronPlate`
- `Wire`, `Cable`
- `Screw`, `Plate`

**Intermediate Components:**
- `Rotor`, `Stator`, `Motor`
- `ModularFrame`, `HeavyModularFrame`
- `SteelBeam`, `SteelPipe`

**Advanced Components:**
- `Computer`, `SuperComputer`
- `CircuitBoard`, `AILimiter`
- `HighSpeedConnector`

---

## Blueprint Best Practices

1. **Descriptive Names**: Use clear, descriptive names for blueprints and production lines
2. **Documentation**: Include detailed descriptions explaining what the blueprint does
3. **Modularity**: Create self-contained blueprints that can work independently
4. **Versioning**: Include version info in blueprint names or descriptions
5. **Power Planning**: Consider power requirements when setting overclock values
6. **Somersloop Strategy**: Document why specific machines use power shards
7. **Testing**: Always test exported blueprints by importing to a new factory

---

## Troubleshooting

### Import Fails
- **Check JSON syntax**: Use a JSON validator (https://jsonlint.com/)
- **Verify UUID format**: All `id` fields must be valid UUIDs (format: `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`)
- **Check structure**: Production lines should NOT be wrapped in `ProductionLineRecipe` object - they ARE ProductionLineRecipe objects directly
- **Verify recipe names**: Ensure they match game recipes exactly (case-sensitive)
- **Check IDs**: Ensure all IDs are unique within the blueprint
- **Float values**: Use `100.0` not `100` for `oc_value` field

### Blueprint Doesn't Display Correctly
- **Verify structure**: Compare with example blueprints
- **Check machine groups**: Ensure all required fields present
- **Validate overclock values**: Must be between 1-250

### Export Produces Invalid JSON
- **Report as bug**: This shouldn't happen
- **Check browser console**: Look for error messages
- **Try simpler blueprint**: Test with fewer production lines

---

## Contributing

If you create useful blueprint examples, consider contributing them:

1. Ensure the blueprint is well-documented
2. Test import/export functionality
3. Add entry to this README
4. Submit a pull request

---

## Technical Notes

- Blueprint IDs must be unique across the system
- Production line IDs must be unique within a blueprint
- Overclock values: 1-250 (1% to 250%)
- Somersloop values: 0-3 per machine (typical)
- Recipe names are case-sensitive
- JSON must be valid UTF-8 encoded

---

**Happy Building!** 🏗️
