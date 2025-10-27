/**
 * Frontend Calculation Violation Tests
 *
 * These tests scan the frontend codebase to ensure no game calculations
 * are performed in the frontend. All calculations should be done in the
 * backend engine (Rust) to maintain single source of truth.
 *
 * Forbidden patterns:
 * - Math.pow() calls with overclock formula (1.321928)
 * - Hardcoded base power values (Biomass: 30, Coal: 75, etc.)
 * - Hardcoded fuel consumption rates
 * - Hardcoded extractor power values
 * - Any other game calculations that should only exist in backend
 */

import { describe, it, expect, beforeAll } from 'vitest'
import fs from 'fs'
import path from 'path'

// Forbidden patterns that should never appear in frontend code
const FORBIDDEN_PATTERNS = [
  // Math.pow with overclock formula (1.321928)
  /Math\.pow\([^,]+,\s*1\.321928\s*\)/g,

  // Hardcoded base power values for generators
  /Biomass.*:\s*30[^.]|Coal.*:\s*75[^.]|Fuel.*:\s*150[^.]|Nuclear.*:\s*2500[^.]|Geothermal.*:\s*200[^.]/g,

  // Hardcoded fuel consumption rates
  /Coal.*:\s*15\.3[^.]|Fuel.*:\s*4\.5[^.]|Biomass.*:\s*4[^.]|Nuclear.*:\s*0\.025[^.]/g,

  // Hardcoded extractor power values
  /MinerMk1.*:\s*5[^.]|MinerMk2.*:\s*12[^.]|MinerMk3.*:\s*30[^.]|WaterExtractor.*:\s*20[^.]|OilExtractor.*:\s*40[^.]/g,

  // Hardcoded extraction rates (found in RawInputForm.vue)
  /MinerMk1.*Impure.*30[^.]|MinerMk1.*Normal.*60[^.]|MinerMk1.*Pure.*120[^.]/g,
  /MinerMk2.*Impure.*60[^.]|MinerMk2.*Normal.*120[^.]|MinerMk2.*Pure.*240[^.]/g,
  /MinerMk3.*Impure.*120[^.]|MinerMk3.*Normal.*240[^.]|MinerMk3.*Pure.*480[^.]/g,
  /WaterExtractor.*Normal.*120[^.]/g,
  /OilExtractor.*Impure.*60[^.]|OilExtractor.*Normal.*120[^.]|OilExtractor.*Pure.*240[^.]/g,
  /ResourceWellExtractor.*Impure.*30[^.]|ResourceWellExtractor.*Normal.*60[^.]|ResourceWellExtractor.*Pure.*120[^.]/g,

  // Power calculation functions that should be in backend
  /basePower.*=.*\{[^}]*Biomass[^}]*\}/g,
  /baseFuelRate.*=.*\{[^}]*Coal[^}]*\}/g,
  /getPowerConsumption.*=.*\(/g,
  /calculatePower.*=.*\(/g,
  /calculateFuel.*=.*\(/g,

  // Overclock formula applications
  /Math\.pow\(.*clock.*100.*1\.321928\)/g,
  /Math\.pow\(.*oc.*100.*1\.321928\)/g,
  /Math\.pow\(.*overclock.*100.*1\.321928\)/g,

  // Somersloop formula applications
  /Math\.pow\(.*1.*\+.*somersloop.*max.*2\)/g,
  /Math\.pow\(.*1.*\+.*somer.*2\)/g,
]

// Key files that are most likely to contain calculation violations
const CRITICAL_FILES = [
  'components/factory/PowerGeneratorForm.vue',
  'components/factory/PowerGeneratorList.vue',
  'components/factory/ProductionLineForm.vue',
  'components/factory/ProductionLineList.vue',
  'components/factory/RawInputForm.vue',
  'components/factory/RawInputList.vue',
  'components/dashboard/PowerStatsChart.vue',
  'views/FactoryView.vue',
  'views/DashboardView.vue',
  'views/LogisticsView.vue',
  'api/types.ts',
  'api/endpoints.ts',
  'stores/factory.ts',
  'stores/gameData.ts',
  'stores/dashboard.ts',
  'stores/logistics.ts',
]

describe('Frontend Calculation Violations', () => {
  let scannedFiles: string[] = []

  beforeAll(() => {
    // Get absolute paths for critical files using process.cwd()
    const srcDir = path.join(process.cwd(), 'src')
    scannedFiles = CRITICAL_FILES.map(file => path.join(srcDir, file)).filter(file => {
      const exists = fs.existsSync(file)
      if (!exists) {
        console.warn(`Warning: File not found: ${file}`)
      }
      return exists
    })
    console.log(`Scanning ${scannedFiles.length} critical files:`, scannedFiles.map(f => path.basename(f)))
  })

  it('should scan all relevant frontend files', () => {
    expect(scannedFiles.length).toBeGreaterThan(0)
    expect(scannedFiles.some(file => file.includes('.vue'))).toBe(true)
    expect(scannedFiles.some(file => file.includes('.ts'))).toBe(true)
  })

  FORBIDDEN_PATTERNS.forEach((pattern, index) => {
    it(`should not contain forbidden pattern ${index + 1}: ${pattern.source}`, () => {
      const violations: Array<{ file: string; line: number; content: string }> = []

      scannedFiles.forEach(file => {
        if (!fs.existsSync(file)) return

        const content = fs.readFileSync(file, 'utf-8')
        const lines = content.split('\n')

        lines.forEach((line, lineIndex) => {
          if (pattern.test(line)) {
            violations.push({
              file: path.relative(process.cwd(), file),
              line: lineIndex + 1,
              content: line.trim()
            })
          }
        })
      })

      if (violations.length > 0) {
        const violationDetails = violations.map(v =>
          `  ${v.file}:${v.line} - ${v.content}`
        ).join('\n')

        expect.fail(
          `Found ${violations.length} violation(s) of forbidden pattern:\n` +
          `${pattern.source}\n\n` +
          `Violations:\n${violationDetails}\n\n` +
          `This indicates game calculations are being performed in the frontend.\n` +
          `All calculations should be moved to the backend engine.`
        )
      }
    })
  })

  it('should not contain hardcoded extraction rates', () => {
    const violations: Array<{ file: string; line: number; content: string }> = []

    scannedFiles.forEach(file => {
      if (!fs.existsSync(file)) return

      const content = fs.readFileSync(file, 'utf-8')
      const lines = content.split('\n')

      lines.forEach((line, lineIndex) => {
        // Check for the specific hardcoded rates found in RawInputForm.vue
        if (line.includes('MinerMk1') && line.includes('Impure') && line.includes('30')) {
          violations.push({
            file: path.relative(process.cwd(), file),
            line: lineIndex + 1,
            content: line.trim()
          })
        }
        if (line.includes('MinerMk2') && line.includes('Normal') && line.includes('120')) {
          violations.push({
            file: path.relative(process.cwd(), file),
            line: lineIndex + 1,
            content: line.trim()
          })
        }
        if (line.includes('OilExtractor') && line.includes('Pure') && line.includes('240')) {
          violations.push({
            file: path.relative(process.cwd(), file),
            line: lineIndex + 1,
            content: line.trim()
          })
        }

        // Also check for the baseRates object structure
        if (line.includes('baseRates') && line.includes('Record') && line.includes('Purity')) {
          violations.push({
            file: path.relative(process.cwd(), file),
            line: lineIndex + 1,
            content: line.trim()
          })
        }

        // Check for the actual hardcoded values in the baseRates object
        if (line.includes('MinerMk1:') && line.includes('{ Impure: 30')) {
          violations.push({
            file: path.relative(process.cwd(), file),
            line: lineIndex + 1,
            content: line.trim()
          })
        }
        if (line.includes('MinerMk2:') && line.includes('{ Impure: 60')) {
          violations.push({
            file: path.relative(process.cwd(), file),
            line: lineIndex + 1,
            content: line.trim()
          })
        }
        if (line.includes('OilExtractor:') && line.includes('Pure: 240')) {
          violations.push({
            file: path.relative(process.cwd(), file),
            line: lineIndex + 1,
            content: line.trim()
          })
        }

        // Check for hardcoded rates in getExtractorRate function
        if (line.includes('Impure: 30') || line.includes('Normal: 60') || line.includes('Pure: 120')) {
          violations.push({
            file: path.relative(process.cwd(), file),
            line: lineIndex + 1,
            content: line.trim()
          })
        }

        // Check for the baseRates object definition
        if (line.includes('baseRates:') && line.includes('Record<string, Record<Purity, number>>')) {
          violations.push({
            file: path.relative(process.cwd(), file),
            line: lineIndex + 1,
            content: line.trim()
          })
        }
      })
    })

    if (violations.length > 0) {
      const violationDetails = violations.map(v =>
        `  ${v.file}:${v.line} - ${v.content}`
      ).join('\n')

      expect.fail(
        `Found hardcoded extraction rates in frontend code:\n\n` +
        `Violations:\n${violationDetails}\n\n` +
        `These rates should be calculated by the backend engine based on game data.\n` +
        `Frontend should only display values provided by the API.`
      )
    }
  })

  it('should not contain power calculation functions', () => {
    const violations: Array<{ file: string; line: number; content: string }> = []

    scannedFiles.forEach(file => {
      if (!fs.existsSync(file)) return

      const content = fs.readFileSync(file, 'utf-8')
      const lines = content.split('\n')

      lines.forEach((line, lineIndex) => {
        // Check for function definitions that calculate power
        if (line.includes('function') || line.includes('const') || line.includes('let')) {
          if (line.includes('power') && (line.includes('calculate') || line.includes('calc'))) {
            violations.push({
              file: path.relative(process.cwd(), file),
              line: lineIndex + 1,
              content: line.trim()
            })
          }
        }

        // Check for power calculation logic (but allow type definitions and data access)
        if (line.includes('basePower') && (line.includes('=') || line.includes('function') || line.includes('=>'))) {
          violations.push({
            file: path.relative(process.cwd(), file),
            line: lineIndex + 1,
            content: line.trim()
          })
        }
        if (line.includes('base_power') && (line.includes('=') || line.includes('function') || line.includes('=>'))) {
          violations.push({
            file: path.relative(process.cwd(), file),
            line: lineIndex + 1,
            content: line.trim()
          })
        }
      })
    })

    if (violations.length > 0) {
      const violationDetails = violations.map(v =>
        `  ${v.file}:${v.line} - ${v.content}`
      ).join('\n')

      expect.fail(
        `Found power calculation functions in frontend code:\n\n` +
        `Violations:\n${violationDetails}\n\n` +
        `Power calculations should only be performed in the backend engine.\n` +
        `Frontend should display values from API responses.`
      )
    }
  })

  it('should not contain overclock formula applications', () => {
    const violations: Array<{ file: string; line: number; content: string }> = []

    scannedFiles.forEach(file => {
      if (!fs.existsSync(file)) return

      const content = fs.readFileSync(file, 'utf-8')
      const lines = content.split('\n')

      lines.forEach((line, lineIndex) => {
        // Check for overclock calculations
        if (line.includes('Math.pow') && line.includes('clock') && line.includes('1.321928')) {
          violations.push({
            file: path.relative(process.cwd(), file),
            line: lineIndex + 1,
            content: line.trim()
          })
        }

        // Check for somersloop calculations
        if (line.includes('Math.pow') && line.includes('somersloop') && line.includes('max')) {
          violations.push({
            file: path.relative(process.cwd(), file),
            line: lineIndex + 1,
            content: line.trim()
          })
        }
      })
    })

    if (violations.length > 0) {
      const violationDetails = violations.map(v =>
        `  ${v.file}:${v.line} - ${v.content}`
      ).join('\n')

      expect.fail(
        `Found overclock/somersloop formula applications in frontend code:\n\n` +
        `Violations:\n${violationDetails}\n\n` +
        `Game formulas should only be applied in the backend engine.\n` +
        `Frontend should use preview APIs for real-time calculations.`
      )
    }
  })

  it('should only use acceptable Math functions for display formatting', () => {
    const violations: Array<{ file: string; line: number; content: string }> = []
    const acceptablePatterns = [
      /Math\.max\(/g,  // For finding maximum values (display)
      /Math\.abs\(/g,  // For absolute values (display formatting)
      /Math\.round\(/g, // For rounding (display)
      /Math\.floor\(/g, // For flooring (display)
      /Math\.ceil\(/g,  // For ceiling (display)
    ]

    scannedFiles.forEach(file => {
      if (!fs.existsSync(file)) return

      const content = fs.readFileSync(file, 'utf-8')
      const lines = content.split('\n')

      lines.forEach((line, lineIndex) => {
        // Check for any Math function calls
        if (line.includes('Math.')) {
          // Check if it's an acceptable pattern
          const isAcceptable = acceptablePatterns.some(pattern => pattern.test(line))

          // Also allow Math.abs in specific display formatting contexts
          const isDisplayFormatting = line.includes('Math.abs') && (
            line.includes('powerBalance') || // Power chart percentage calculation
            line.includes('value') && line.includes('1000') || // Power value formatting
            line.includes('/ maxPower') || // Percentage calculation
            line.includes('* 100') // Percentage conversion
          )

          if (!isAcceptable && !isDisplayFormatting) {
            violations.push({
              file: path.relative(process.cwd(), file),
              line: lineIndex + 1,
              content: line.trim()
            })
          }
        }
      })
    })

    if (violations.length > 0) {
      const violationDetails = violations.map(v =>
        `  ${v.file}:${v.line} - ${v.content}`
      ).join('\n')

      expect.fail(
        `Found unacceptable Math function calls in frontend code:\n\n` +
        `Violations:\n${violationDetails}\n\n` +
        `Only Math.max, Math.abs, Math.round, Math.floor, and Math.ceil are allowed for display formatting.\n` +
        `All other calculations should be performed in the backend.`
      )
    }
  })
})
