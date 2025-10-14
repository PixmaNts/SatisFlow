macro_rules! recipe_data {
    ($macro:ident) => {
        $macro! {
AILimiter => {
        name: "AI Limiter",
        machine: Assembler,
        inputs: [
            (CopperSheet, 25.000000),
            (Quickwire, 100.000000),
        ],
        outputs: [
            (AILimiter, 5.000000),
        ],
    },
    AWESOMEShop => {
        name: "AWESOME Shop",
        machine: Manual,
        inputs: [
            (Screw, 12000.000000),
            (IronPlate, 600.000000),
            (Cable, 600.000000),
        ],
        outputs: [],
    },
    AWESOMESink => {
        name: "AWESOME Sink",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 900.000000),
            (Cable, 1800.000000),
            (Concrete, 2700.000000),
        ],
        outputs: [],
    },
    AdaptiveControlUnit => {
        name: "Adaptive Control Unit",
        machine: Manufacturer,
        inputs: [
            (AutomatedWiring, 7.500000),
            (CircuitBoard, 5.000000),
            (HeavyModularFrame, 1.000000),
            (Computer, 1.000000),
        ],
        outputs: [
            (AdaptiveControlUnit, 1.000000),
        ],
    },
    AlcladAluminumSheet => {
        name: "Alclad Aluminum Sheet",
        machine: Assembler,
        inputs: [
            (AluminumIngot, 30.000000),
            (CopperIngot, 10.000000),
        ],
        outputs: [
            (AlcladAluminumSheet, 30.000000),
        ],
    },
    AlienDNACapsule => {
        name: "Alien DNA Capsule",
        machine: Constructor,
        inputs: [
            (AlienProtein, 10.000000),
        ],
        outputs: [
            (AlienDNACapsule, 10.000000),
        ],
    },
    AlternateAdheredIronPlate => {
        name: "Alternate: Adhered Iron Plate",
        machine: Assembler,
        inputs: [
            (IronPlate, 11.250000),
            (Rubber, 3.750000),
        ],
        outputs: [
            (ReinforcedIronPlate, 3.750000),
        ],
    },
    AlternateAlcladCasing => {
        name: "Alternate: Alclad Casing",
        machine: Assembler,
        inputs: [
            (AluminumIngot, 150.000000),
            (CopperIngot, 75.000000),
        ],
        outputs: [
            (AluminumCasing, 112.500000),
        ],
    },
    AlternateAutomatedMiner => {
        name: "Alternate: Automated Miner",
        machine: Manufacturer,
        inputs: [
            (Motor, 1.000000),
            (SteelPipe, 4.000000),
            (IronRod, 4.000000),
            (IronPlate, 2.000000),
        ],
        outputs: [
            (PortableMiner, 1.000000),
        ],
    },
    AlternateAutomatedSpeedWiring => {
        name: "Alternate: Automated Speed Wiring",
        machine: Manufacturer,
        inputs: [
            (Stator, 3.750000),
            (Wire, 75.000000),
            (HighSpeedConnector, 1.875000),
        ],
        outputs: [
            (AutomatedWiring, 7.500000),
        ],
    },
    AlternateBiocoal => {
        name: "Alternate: Biocoal",
        machine: Constructor,
        inputs: [
            (Biomass, 37.500000),
        ],
        outputs: [
            (Coal, 45.000000),
        ],
    },
    AlternateBoltedFrame => {
        name: "Alternate: Bolted Frame",
        machine: Assembler,
        inputs: [
            (ReinforcedIronPlate, 7.500000),
            (Screw, 140.000000),
        ],
        outputs: [
            (ModularFrame, 5.000000),
        ],
    },
    AlternateBoltedIronPlate => {
        name: "Alternate: Bolted Iron Plate",
        machine: Assembler,
        inputs: [
            (IronPlate, 90.000000),
            (Screw, 250.000000),
        ],
        outputs: [
            (ReinforcedIronPlate, 15.000000),
        ],
    },
    AlternateCastScrew => {
        name: "Alternate: Cast Screw",
        machine: Constructor,
        inputs: [
            (IronIngot, 12.500000),
        ],
        outputs: [
            (Screw, 50.000000),
        ],
    },
    AlternateCateriumCircuitBoard => {
        name: "Alternate: Caterium Circuit Board",
        machine: Assembler,
        inputs: [
            (Plastic, 12.500000),
            (Quickwire, 37.500000),
        ],
        outputs: [
            (CircuitBoard, 8.750000),
        ],
    },
    AlternateCateriumComputer => {
        name: "Alternate: Caterium Computer",
        machine: Manufacturer,
        inputs: [
            (CircuitBoard, 26.250000),
            (Quickwire, 105.000000),
            (Rubber, 45.000000),
        ],
        outputs: [
            (Computer, 3.750000),
        ],
    },
    AlternateCateriumWire => {
        name: "Alternate: Caterium Wire",
        machine: Constructor,
        inputs: [
            (CateriumIngot, 15.000000),
        ],
        outputs: [
            (Wire, 120.000000),
        ],
    },
    AlternateCharcoal => {
        name: "Alternate: Charcoal",
        machine: Constructor,
        inputs: [
            (Wood, 15.000000),
        ],
        outputs: [
            (Coal, 150.000000),
        ],
    },
    AlternateCheapSilica => {
        name: "Alternate: Cheap Silica",
        machine: Assembler,
        inputs: [
            (RawQuartz, 11.250000),
            (Limestone, 18.750000),
        ],
        outputs: [
            (Silica, 26.250000),
        ],
    },
    AlternateClassicBattery => {
        name: "Alternate: Classic Battery",
        machine: Manufacturer,
        inputs: [
            (Sulfur, 45.000000),
            (AlcladAluminumSheet, 52.500000),
            (Plastic, 60.000000),
            (Wire, 90.000000),
        ],
        outputs: [
            (Battery, 30.000000),
        ],
    },
    AlternateCoatedCable => {
        name: "Alternate: Coated Cable",
        machine: Refinery,
        inputs: [
            (Wire, 37.500000),
            (HeavyOilResidue, 15.000000),
        ],
        outputs: [
            (Cable, 67.500000),
        ],
    },
    AlternateCoatedIronCanister => {
        name: "Alternate: Coated Iron Canister",
        machine: Assembler,
        inputs: [
            (IronPlate, 30.000000),
            (CopperSheet, 15.000000),
        ],
        outputs: [
            (EmptyCanister, 60.000000),
        ],
    },
    AlternateCoatedIronPlate => {
        name: "Alternate: Coated Iron Plate",
        machine: Assembler,
        inputs: [
            (IronIngot, 50.000000),
            (Plastic, 10.000000),
        ],
        outputs: [
            (IronPlate, 75.000000),
        ],
    },
    AlternateCokeSteelIngot => {
        name: "Alternate: Coke Steel Ingot",
        machine: Foundry,
        inputs: [
            (IronOre, 75.000000),
            (PetroleumCoke, 75.000000),
        ],
        outputs: [
            (SteelIngot, 100.000000),
        ],
    },
    AlternateCompactedCoal => {
        name: "Alternate: Compacted Coal",
        machine: Assembler,
        inputs: [
            (Coal, 25.000000),
            (Sulfur, 25.000000),
        ],
        outputs: [
            (CompactedCoal, 25.000000),
        ],
    },
    AlternateCompactedSteelIngot => {
        name: "Alternate: Compacted Steel Ingot",
        machine: Foundry,
        inputs: [
            (IronOre, 22.500000),
            (CompactedCoal, 11.250000),
        ],
        outputs: [
            (SteelIngot, 37.500000),
        ],
    },
    AlternateCoolingDevice => {
        name: "Alternate: Cooling Device",
        machine: Blender,
        inputs: [
            (HeatSink, 9.375000),
            (Motor, 1.875000),
            (NitrogenGas, 45.000000),
        ],
        outputs: [
            (CoolingSystem, 3.750000),
        ],
    },
    AlternateCopperAlloyIngot => {
        name: "Alternate: Copper Alloy Ingot",
        machine: Foundry,
        inputs: [
            (CopperOre, 50.000000),
            (IronOre, 25.000000),
        ],
        outputs: [
            (CopperIngot, 100.000000),
        ],
    },
    AlternateCopperRotor => {
        name: "Alternate: Copper Rotor",
        machine: Assembler,
        inputs: [
            (CopperSheet, 22.500000),
            (Screw, 195.000000),
        ],
        outputs: [
            (Rotor, 11.250000),
        ],
    },
    AlternateCrystalBeacon => {
        name: "Alternate: Crystal Beacon",
        machine: Manufacturer,
        inputs: [
            (SteelBeam, 2.000000),
            (SteelPipe, 8.000000),
            (CrystalOscillator, 0.500000),
        ],
        outputs: [
            (Beacon, 10.000000),
        ],
    },
    AlternateCrystalComputer => {
        name: "Alternate: Crystal Computer",
        machine: Assembler,
        inputs: [
            (CircuitBoard, 7.500000),
            (CrystalOscillator, 2.812500),
        ],
        outputs: [
            (Computer, 2.812500),
        ],
    },
    AlternateDilutedFuel => {
        name: "Alternate: Diluted Fuel",
        machine: Blender,
        inputs: [
            (HeavyOilResidue, 50.000000),
            (Water, 100.000000),
        ],
        outputs: [
            (Fuel, 100.000000),
        ],
    },
    AlternateDilutedPackagedFuel => {
        name: "Alternate: Diluted Packaged Fuel",
        machine: Refinery,
        inputs: [
            (HeavyOilResidue, 30.000000),
            (PackagedWater, 60.000000),
        ],
        outputs: [
            (PackagedFuel, 60.000000),
        ],
    },
    AlternateElectricMotor => {
        name: "Alternate: Electric Motor",
        machine: Assembler,
        inputs: [
            (ElectromagneticControlRod, 3.750000),
            (Rotor, 7.500000),
        ],
        outputs: [
            (Motor, 7.500000),
        ],
    },
    AlternateElectrodeAluminumScrap => {
        name: "Alternate: Electrode Aluminum Scrap",
        machine: Refinery,
        inputs: [
            (AluminaSolution, 180.000000),
            (PetroleumCoke, 60.000000),
        ],
        outputs: [
            (AluminumScrap, 300.000000),
            (Water, 105.000000),
        ],
    },
    AlternateElectrodeCircuitBoard => {
        name: "Alternate: Electrode Circuit Board",
        machine: Assembler,
        inputs: [
            (Rubber, 30.000000),
            (PetroleumCoke, 45.000000),
        ],
        outputs: [
            (CircuitBoard, 5.000000),
        ],
    },
    AlternateElectromagneticConnectionRod => {
        name: "Alternate: Electromagnetic Connection Rod",
        machine: Assembler,
        inputs: [
            (Stator, 8.000000),
            (HighSpeedConnector, 4.000000),
        ],
        outputs: [
            (ElectromagneticControlRod, 8.000000),
        ],
    },
    AlternateEncasedIndustrialPipe => {
        name: "Alternate: Encased Industrial Pipe",
        machine: Assembler,
        inputs: [
            (SteelPipe, 28.000000),
            (Concrete, 20.000000),
        ],
        outputs: [
            (EncasedIndustrialBeam, 4.000000),
        ],
    },
    AlternateFertileUranium => {
        name: "Alternate: Fertile Uranium",
        machine: Blender,
        inputs: [
            (Uranium, 25.000000),
            (UraniumWaste, 25.000000),
            (NitricAcid, 15.000000),
            (SulfuricAcid, 25.000000),
        ],
        outputs: [
            (NonFissileUranium, 100.000000),
            (Water, 40.000000),
        ],
    },
    AlternateFineBlackPowder => {
        name: "Alternate: Fine Black Powder",
        machine: Assembler,
        inputs: [
            (Sulfur, 7.500000),
            (CompactedCoal, 3.750000),
        ],
        outputs: [
            (BlackPowder, 15.000000),
        ],
    },
    AlternateFineConcrete => {
        name: "Alternate: Fine Concrete",
        machine: Assembler,
        inputs: [
            (Silica, 7.500000),
            (Limestone, 30.000000),
        ],
        outputs: [
            (Concrete, 25.000000),
        ],
    },
    AlternateFlexibleFramework => {
        name: "Alternate: Flexible Framework",
        machine: Manufacturer,
        inputs: [
            (ModularFrame, 3.750000),
            (SteelBeam, 22.500000),
            (Rubber, 30.000000),
        ],
        outputs: [
            (VersatileFramework, 7.500000),
        ],
    },
    AlternateFusedQuickwire => {
        name: "Alternate: Fused Quickwire",
        machine: Assembler,
        inputs: [
            (CateriumIngot, 7.500000),
            (CopperIngot, 37.500000),
        ],
        outputs: [
            (Quickwire, 90.000000),
        ],
    },
    AlternateFusedWire => {
        name: "Alternate: Fused Wire",
        machine: Assembler,
        inputs: [
            (CopperIngot, 12.000000),
            (CateriumIngot, 3.000000),
        ],
        outputs: [
            (Wire, 90.000000),
        ],
    },
    AlternateHeatExchanger => {
        name: "Alternate: Heat Exchanger",
        machine: Assembler,
        inputs: [
            (AluminumCasing, 30.000000),
            (Rubber, 30.000000),
        ],
        outputs: [
            (HeatSink, 10.000000),
        ],
    },
    AlternateHeatFusedFrame => {
        name: "Alternate: Heat-Fused Frame",
        machine: Blender,
        inputs: [
            (HeavyModularFrame, 3.000000),
            (AluminumIngot, 150.000000),
            (NitricAcid, 24.000000),
            (Fuel, 30.000000),
        ],
        outputs: [
            (FusedModularFrame, 3.000000),
        ],
    },
    AlternateHeavyEncasedFrame => {
        name: "Alternate: Heavy Encased Frame",
        machine: Manufacturer,
        inputs: [
            (ModularFrame, 7.500000),
            (EncasedIndustrialBeam, 9.375000),
            (SteelPipe, 33.750000),
            (Concrete, 20.625000),
        ],
        outputs: [
            (HeavyModularFrame, 2.812500),
        ],
    },
    AlternateHeavyFlexibleFrame => {
        name: "Alternate: Heavy Flexible Frame",
        machine: Manufacturer,
        inputs: [
            (ModularFrame, 18.750000),
            (EncasedIndustrialBeam, 11.250000),
            (Rubber, 75.000000),
            (Screw, 390.000000),
        ],
        outputs: [
            (HeavyModularFrame, 3.750000),
        ],
    },
    AlternateHeavyOilResidue => {
        name: "Alternate: Heavy Oil Residue",
        machine: Refinery,
        inputs: [
            (CrudeOil, 30.000000),
        ],
        outputs: [
            (HeavyOilResidue, 40.000000),
            (PolymerResin, 20.000000),
        ],
    },
    AlternateInfusedUraniumCell => {
        name: "Alternate: Infused Uranium Cell",
        machine: Manufacturer,
        inputs: [
            (Uranium, 25.000000),
            (Silica, 15.000000),
            (Sulfur, 25.000000),
            (Quickwire, 75.000000),
        ],
        outputs: [
            (EncasedUraniumCell, 20.000000),
        ],
    },
    AlternateInstantPlutoniumCell => {
        name: "Alternate: Instant Plutonium Cell",
        machine: ParticleAccelerator,
        inputs: [
            (NonFissileUranium, 75.000000),
            (AluminumCasing, 10.000000),
        ],
        outputs: [
            (EncasedPlutoniumCell, 10.000000),
        ],
    },
    AlternateInstantScrap => {
        name: "Alternate: Instant Scrap",
        machine: Blender,
        inputs: [
            (Bauxite, 150.000000),
            (Coal, 100.000000),
            (SulfuricAcid, 50.000000),
            (Water, 60.000000),
        ],
        outputs: [
            (AluminumScrap, 300.000000),
            (Water, 50.000000),
        ],
    },
    AlternateInsulatedCable => {
        name: "Alternate: Insulated Cable",
        machine: Assembler,
        inputs: [
            (Wire, 45.000000),
            (Rubber, 30.000000),
        ],
        outputs: [
            (Cable, 100.000000),
        ],
    },
    AlternateInsulatedCrystalOscillator => {
        name: "Alternate: Insulated Crystal Oscillator",
        machine: Manufacturer,
        inputs: [
            (QuartzCrystal, 18.750000),
            (Rubber, 13.125000),
            (AILimiter, 1.875000),
        ],
        outputs: [
            (CrystalOscillator, 1.875000),
        ],
    },
    AlternateIronAlloyIngot => {
        name: "Alternate: Iron Alloy Ingot",
        machine: Foundry,
        inputs: [
            (IronOre, 20.000000),
            (CopperOre, 20.000000),
        ],
        outputs: [
            (IronIngot, 50.000000),
        ],
    },
    AlternateIronWire => {
        name: "Alternate: Iron Wire",
        machine: Constructor,
        inputs: [
            (IronIngot, 12.500000),
        ],
        outputs: [
            (Wire, 22.500000),
        ],
    },
    AlternateOCSupercomputer => {
        name: "Alternate: OC Supercomputer",
        machine: Assembler,
        inputs: [
            (RadioControlUnit, 9.000000),
            (CoolingSystem, 9.000000),
        ],
        outputs: [
            (Supercomputer, 3.000000),
        ],
    },
    AlternatePlasticSmartPlating => {
        name: "Alternate: Plastic Smart Plating",
        machine: Manufacturer,
        inputs: [
            (ReinforcedIronPlate, 2.500000),
            (Rotor, 2.500000),
            (Plastic, 7.500000),
        ],
        outputs: [
            (SmartPlating, 5.000000),
        ],
    },
    AlternatePlutoniumFuelUnit => {
        name: "Alternate: Plutonium Fuel Unit",
        machine: Assembler,
        inputs: [
            (EncasedPlutoniumCell, 10.000000),
            (PressureConversionCube, 0.500000),
        ],
        outputs: [
            (PlutoniumFuelRod, 0.500000),
        ],
    },
    AlternatePolyesterFabric => {
        name: "Alternate: Polyester Fabric",
        machine: Refinery,
        inputs: [
            (PolymerResin, 30.000000),
            (Water, 30.000000),
        ],
        outputs: [
            (Fabric, 30.000000),
        ],
    },
    AlternatePolymerResin => {
        name: "Alternate: Polymer Resin",
        machine: Refinery,
        inputs: [
            (CrudeOil, 60.000000),
        ],
        outputs: [
            (PolymerResin, 130.000000),
            (HeavyOilResidue, 20.000000),
        ],
    },
    AlternatePureAluminumIngot => {
        name: "Alternate: Pure Aluminum Ingot",
        machine: Smelter,
        inputs: [
            (AluminumScrap, 60.000000),
        ],
        outputs: [
            (AluminumIngot, 30.000000),
        ],
    },
    AlternatePureCateriumIngot => {
        name: "Alternate: Pure Caterium Ingot",
        machine: Refinery,
        inputs: [
            (CateriumOre, 24.000000),
            (Water, 24.000000),
        ],
        outputs: [
            (CateriumIngot, 12.000000),
        ],
    },
    AlternatePureCopperIngot => {
        name: "Alternate: Pure Copper Ingot",
        machine: Refinery,
        inputs: [
            (CopperOre, 15.000000),
            (Water, 10.000000),
        ],
        outputs: [
            (CopperIngot, 37.500000),
        ],
    },
    AlternatePureIronIngot => {
        name: "Alternate: Pure Iron Ingot",
        machine: Refinery,
        inputs: [
            (IronOre, 35.000000),
            (Water, 20.000000),
        ],
        outputs: [
            (IronIngot, 65.000000),
        ],
    },
    AlternatePureQuartzCrystal => {
        name: "Alternate: Pure Quartz Crystal",
        machine: Refinery,
        inputs: [
            (RawQuartz, 67.500000),
            (Water, 37.500000),
        ],
        outputs: [
            (QuartzCrystal, 52.500000),
        ],
    },
    AlternateQuickwireCable => {
        name: "Alternate: Quickwire Cable",
        machine: Assembler,
        inputs: [
            (Quickwire, 7.500000),
            (Rubber, 5.000000),
        ],
        outputs: [
            (Cable, 27.500000),
        ],
    },
    AlternateQuickwireStator => {
        name: "Alternate: Quickwire Stator",
        machine: Assembler,
        inputs: [
            (SteelPipe, 16.000000),
            (Quickwire, 60.000000),
        ],
        outputs: [
            (Stator, 8.000000),
        ],
    },
    AlternateRadioConnectionUnit => {
        name: "Alternate: Radio Connection Unit",
        machine: Manufacturer,
        inputs: [
            (HeatSink, 15.000000),
            (HighSpeedConnector, 7.500000),
            (QuartzCrystal, 45.000000),
        ],
        outputs: [
            (RadioControlUnit, 3.750000),
        ],
    },
    AlternateRadioControlSystem => {
        name: "Alternate: Radio Control System",
        machine: Manufacturer,
        inputs: [
            (CrystalOscillator, 1.500000),
            (CircuitBoard, 15.000000),
            (AluminumCasing, 90.000000),
            (Rubber, 45.000000),
        ],
        outputs: [
            (RadioControlUnit, 4.500000),
        ],
    },
    AlternateRecycledPlastic => {
        name: "Alternate: Recycled Plastic",
        machine: Refinery,
        inputs: [
            (Rubber, 30.000000),
            (Fuel, 30.000000),
        ],
        outputs: [
            (Plastic, 60.000000),
        ],
    },
    AlternateRecycledRubber => {
        name: "Alternate: Recycled Rubber",
        machine: Refinery,
        inputs: [
            (Plastic, 30.000000),
            (Fuel, 30.000000),
        ],
        outputs: [
            (Rubber, 60.000000),
        ],
    },
    AlternateRigourMotor => {
        name: "Alternate: Rigour Motor",
        machine: Manufacturer,
        inputs: [
            (Rotor, 3.750000),
            (Stator, 3.750000),
            (CrystalOscillator, 1.250000),
        ],
        outputs: [
            (Motor, 7.500000),
        ],
    },
    AlternateRubberConcrete => {
        name: "Alternate: Rubber Concrete",
        machine: Assembler,
        inputs: [
            (Limestone, 50.000000),
            (Rubber, 10.000000),
        ],
        outputs: [
            (Concrete, 45.000000),
        ],
    },
    AlternateSiliconCircuitBoard => {
        name: "Alternate: Silicon Circuit Board",
        machine: Assembler,
        inputs: [
            (CopperSheet, 27.500000),
            (Silica, 27.500000),
        ],
        outputs: [
            (CircuitBoard, 12.500000),
        ],
    },
    AlternateSiliconHighSpeedConnector => {
        name: "Alternate: Silicon High-Speed Connector",
        machine: Manufacturer,
        inputs: [
            (Quickwire, 90.000000),
            (Silica, 37.500000),
            (CircuitBoard, 3.000000),
        ],
        outputs: [
            (HighSpeedConnector, 3.000000),
        ],
    },
    AlternateSloppyAlumina => {
        name: "Alternate: Sloppy Alumina",
        machine: Refinery,
        inputs: [
            (Bauxite, 200.000000),
            (Water, 200.000000),
        ],
        outputs: [
            (AluminaSolution, 240.000000),
        ],
    },
    AlternateSolidSteelIngot => {
        name: "Alternate: Solid Steel Ingot",
        machine: Foundry,
        inputs: [
            (IronIngot, 40.000000),
            (Coal, 40.000000),
        ],
        outputs: [
            (SteelIngot, 60.000000),
        ],
    },
    AlternateSteamedCopperSheet => {
        name: "Alternate: Steamed Copper Sheet",
        machine: Refinery,
        inputs: [
            (CopperIngot, 22.500000),
            (Water, 22.500000),
        ],
        outputs: [
            (CopperSheet, 22.500000),
        ],
    },
    AlternateSteelCanister => {
        name: "Alternate: Steel Canister",
        machine: Constructor,
        inputs: [
            (SteelIngot, 60.000000),
        ],
        outputs: [
            (EmptyCanister, 40.000000),
        ],
    },
    AlternateSteelCoatedPlate => {
        name: "Alternate: Steel Coated Plate",
        machine: Assembler,
        inputs: [
            (SteelIngot, 7.500000),
            (Plastic, 5.000000),
        ],
        outputs: [
            (IronPlate, 45.000000),
        ],
    },
    AlternateSteelRod => {
        name: "Alternate: Steel Rod",
        machine: Constructor,
        inputs: [
            (SteelIngot, 12.000000),
        ],
        outputs: [
            (IronRod, 48.000000),
        ],
    },
    AlternateSteelRotor => {
        name: "Alternate: Steel Rotor",
        machine: Assembler,
        inputs: [
            (SteelPipe, 10.000000),
            (Wire, 30.000000),
        ],
        outputs: [
            (Rotor, 5.000000),
        ],
    },
    AlternateSteelScrew => {
        name: "Alternate: Steel Screw",
        machine: Constructor,
        inputs: [
            (SteelBeam, 5.000000),
        ],
        outputs: [
            (Screw, 260.000000),
        ],
    },
    AlternateSteeledFrame => {
        name: "Alternate: Steeled Frame",
        machine: Assembler,
        inputs: [
            (ReinforcedIronPlate, 2.000000),
            (SteelPipe, 10.000000),
        ],
        outputs: [
            (ModularFrame, 3.000000),
        ],
    },
    AlternateStitchedIronPlate => {
        name: "Alternate: Stitched Iron Plate",
        machine: Assembler,
        inputs: [
            (IronPlate, 18.750000),
            (Wire, 37.500000),
        ],
        outputs: [
            (ReinforcedIronPlate, 5.625000),
        ],
    },
    AlternateSuperStateComputer => {
        name: "Alternate: Super-State Computer",
        machine: Manufacturer,
        inputs: [
            (Computer, 3.600000),
            (ElectromagneticControlRod, 2.400000),
            (Battery, 24.000000),
            (Wire, 54.000000),
        ],
        outputs: [
            (Supercomputer, 2.400000),
        ],
    },
    AlternateTurboBlendFuel => {
        name: "Alternate: Turbo Blend Fuel",
        machine: Blender,
        inputs: [
            (Fuel, 15.000000),
            (HeavyOilResidue, 30.000000),
            (Sulfur, 22.500000),
            (PetroleumCoke, 22.500000),
        ],
        outputs: [
            (Turbofuel, 45.000000),
        ],
    },
    AlternateTurboElectricMotor => {
        name: "Alternate: Turbo Electric Motor",
        machine: Manufacturer,
        inputs: [
            (Motor, 6.562500),
            (RadioControlUnit, 8.437500),
            (ElectromagneticControlRod, 4.687500),
            (Rotor, 6.562500),
        ],
        outputs: [
            (TurboMotor, 2.812500),
        ],
    },
    AlternateTurboHeavyFuel => {
        name: "Alternate: Turbo Heavy Fuel",
        machine: Refinery,
        inputs: [
            (HeavyOilResidue, 37.500000),
            (CompactedCoal, 30.000000),
        ],
        outputs: [
            (Turbofuel, 30.000000),
        ],
    },
    AlternateTurboPressureMotor => {
        name: "Alternate: Turbo Pressure Motor",
        machine: Manufacturer,
        inputs: [
            (Motor, 7.500000),
            (PressureConversionCube, 1.875000),
            (PackagedNitrogenGas, 45.000000),
            (Stator, 15.000000),
        ],
        outputs: [
            (TurboMotor, 3.750000),
        ],
    },
    AlternateUraniumFuelUnit => {
        name: "Alternate: Uranium Fuel Unit",
        machine: Manufacturer,
        inputs: [
            (EncasedUraniumCell, 20.000000),
            (ElectromagneticControlRod, 2.000000),
            (CrystalOscillator, 0.600000),
            (Beacon, 1.200000),
        ],
        outputs: [
            (UraniumFuelRod, 0.600000),
        ],
    },
    AlternateWetConcrete => {
        name: "Alternate: Wet Concrete",
        machine: Refinery,
        inputs: [
            (Limestone, 120.000000),
            (Water, 100.000000),
        ],
        outputs: [
            (Concrete, 80.000000),
        ],
    },
    AluminaSolution => {
        name: "Alumina Solution",
        machine: Refinery,
        inputs: [
            (Bauxite, 120.000000),
            (Water, 180.000000),
        ],
        outputs: [
            (AluminaSolution, 120.000000),
            (Silica, 50.000000),
        ],
    },
    AluminumCasing => {
        name: "Aluminum Casing",
        machine: Constructor,
        inputs: [
            (AluminumIngot, 90.000000),
        ],
        outputs: [
            (AluminumCasing, 60.000000),
        ],
    },
    AluminumIngot => {
        name: "Aluminum Ingot",
        machine: Foundry,
        inputs: [
            (AluminumScrap, 90.000000),
            (Silica, 75.000000),
        ],
        outputs: [
            (AluminumIngot, 60.000000),
        ],
    },
    AluminumScrap => {
        name: "Aluminum Scrap",
        machine: Refinery,
        inputs: [
            (AluminaSolution, 240.000000),
            (Coal, 120.000000),
        ],
        outputs: [
            (AluminumScrap, 360.000000),
            (Water, 120.000000),
        ],
    },
    Assembler => {
        name: "Assembler",
        machine: Constructor,
        inputs: [
            (ReinforcedIronPlate, 480.000000),
            (Rotor, 240.000000),
            (Cable, 600.000000),
        ],
        outputs: [],
    },
    AssemblyDirectorSystem => {
        name: "Assembly Director System",
        machine: Assembler,
        inputs: [
            (AdaptiveControlUnit, 1.500000),
            (Supercomputer, 0.750000),
        ],
        outputs: [
            (AssemblyDirectorSystem, 0.750000),
        ],
    },
    AutomatedGate => {
        name: "Automated Gate",
        machine: Constructor,
        inputs: [
            (IronPlate, 360.000000),
        ],
        outputs: [],
    },
    AutomatedWiring => {
        name: "Automated Wiring",
        machine: Assembler,
        inputs: [
            (Stator, 2.500000),
            (Cable, 50.000000),
        ],
        outputs: [
            (AutomatedWiring, 2.500000),
        ],
    },
    BasicWall1m => {
        name: "Basic Wall 1m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    BasicWall4m => {
        name: "Basic Wall 4m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    Battery => {
        name: "Battery",
        machine: Blender,
        inputs: [
            (SulfuricAcid, 50.000000),
            (AluminaSolution, 40.000000),
            (AluminumCasing, 20.000000),
        ],
        outputs: [
            (Battery, 20.000000),
            (Water, 30.000000),
        ],
    },
    Beacon => {
        name: "Beacon",
        machine: Manual,
        inputs: [
            (IronPlate, 22.500000),
            (IronRod, 7.500000),
            (Wire, 112.500000),
            (Cable, 15.000000),
        ],
        outputs: [
            (Beacon, 7.500000),
        ],
    },
    BeamConnector => {
        name: "Beam Connector",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    BeamConnectorDouble => {
        name: "Beam Connector Double",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    BeamSupport => {
        name: "Beam Support",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    BigConcretePillar => {
        name: "Big Concrete Pillar",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    BigFramePillar => {
        name: "Big Frame Pillar",
        machine: Manual,
        inputs: [
            (SteelBeam, 300.000000),
        ],
        outputs: [],
    },
    BigMetalPillar => {
        name: "Big Metal Pillar",
        machine: Manual,
        inputs: [
            (IronPlate, 120.000000),
            (Concrete, 180.000000),
        ],
        outputs: [],
    },
    BigPillarSupport => {
        name: "Big Pillar Support",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    BiomassAlienProtein => {
        name: "Biomass (Alien Protein)",
        machine: Constructor,
        inputs: [
            (AlienProtein, 15.000000),
        ],
        outputs: [
            (Biomass, 1500.000000),
        ],
    },
    BiomassBurner => {
        name: "Biomass Burner",
        machine: Manual,
        inputs: [
            (IronPlate, 900.000000),
            (IronRod, 900.000000),
            (Wire, 1500.000000),
        ],
        outputs: [],
    },
    BiomassLeaves => {
        name: "Biomass (Leaves)",
        machine: Constructor,
        inputs: [
            (Leaves, 120.000000),
        ],
        outputs: [
            (Biomass, 60.000000),
        ],
    },
    BiomassMycelia => {
        name: "Biomass (Mycelia)",
        machine: Constructor,
        inputs: [
            (Mycelia, 15.000000),
        ],
        outputs: [
            (Biomass, 150.000000),
        ],
    },
    BiomassWood => {
        name: "Biomass (Wood)",
        machine: Constructor,
        inputs: [
            (Wood, 60.000000),
        ],
        outputs: [
            (Biomass, 300.000000),
        ],
    },
    BlackPowder => {
        name: "Black Powder",
        machine: Assembler,
        inputs: [
            (Coal, 15.000000),
            (Sulfur, 15.000000),
        ],
        outputs: [
            (BlackPowder, 30.000000),
        ],
    },
    BladeRunners => {
        name: "Blade Runners",
        machine: Constructor,
        inputs: [
            (Silica, 20.000000),
            (ModularFrame, 3.000000),
            (Rotor, 3.000000),
        ],
        outputs: [
            (BladeRunners, 1.000000),
        ],
    },
    Blender => {
        name: "Blender",
        machine: Constructor,
        inputs: [
            (Motor, 1200.000000),
            (HeavyModularFrame, 600.000000),
            (AluminumCasing, 3000.000000),
            (RadioControlUnit, 300.000000),
        ],
        outputs: [],
    },
    BlockSignal => {
        name: "Block Signal",
        machine: Constructor,
        inputs: [
            (SteelPipe, 120.000000),
            (CopperSheet, 60.000000),
            (CircuitBoard, 120.000000),
        ],
        outputs: [],
    },
    BlueprintDesigner => {
        name: "Blueprint Designer",
        machine: Constructor,
        inputs: [
            (ModularFrame, 900.000000),
            (Cable, 1500.000000),
            (Concrete, 6000.000000),
            (SteelBeam, 6000.000000),
        ],
        outputs: [],
    },
    Cable => {
        name: "Cable",
        machine: Constructor,
        inputs: [
            (Wire, 60.000000),
        ],
        outputs: [
            (Cable, 30.000000),
        ],
    },
    CandyCane => {
        name: "Candy Cane",
        machine: Constructor,
        inputs: [],
        outputs: [],
    },
    CateriumIngot => {
        name: "Caterium Ingot",
        machine: Smelter,
        inputs: [
            (CateriumOre, 45.000000),
        ],
        outputs: [
            (CateriumIngot, 15.000000),
        ],
    },
    CatwalkCorner => {
        name: "Catwalk Corner",
        machine: Constructor,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 60.000000),
        ],
        outputs: [],
    },
    CatwalkCrossing => {
        name: "Catwalk Crossing",
        machine: Constructor,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 60.000000),
        ],
        outputs: [],
    },
    CatwalkRamp => {
        name: "Catwalk Ramp",
        machine: Constructor,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 60.000000),
        ],
        outputs: [],
    },
    CatwalkStairs => {
        name: "Catwalk Stairs",
        machine: Constructor,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 60.000000),
        ],
        outputs: [],
    },
    CatwalkStraight => {
        name: "Catwalk Straight",
        machine: Constructor,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 60.000000),
        ],
        outputs: [],
    },
    CatwalkTCrossing => {
        name: "Catwalk T-Crossing",
        machine: Manual,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 60.000000),
        ],
        outputs: [],
    },
    CeilingLight => {
        name: "Ceiling Light",
        machine: Manual,
        inputs: [
            (Quickwire, 3000.000000),
            (Wire, 960.000000),
            (SteelBeam, 360.000000),
        ],
        outputs: [],
    },
    CenterDoorWall => {
        name: "Center Door Wall",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    Chainsaw => {
        name: "Chainsaw",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 5.000000),
            (IronRod, 25.000000),
            (Screw, 160.000000),
            (Cable, 15.000000),
        ],
        outputs: [
            (Chainsaw, 1.000000),
        ],
    },
    CircuitBoard => {
        name: "Circuit Board",
        machine: Assembler,
        inputs: [
            (CopperSheet, 15.000000),
            (Plastic, 30.000000),
        ],
        outputs: [
            (CircuitBoard, 7.500000),
        ],
    },
    ClusterNobelisk => {
        name: "Cluster Nobelisk",
        machine: Assembler,
        inputs: [
            (Nobelisk, 7.500000),
            (SmokelessPowder, 10.000000),
        ],
        outputs: [
            (ClusterNobelisk, 2.500000),
        ],
    },
    CoalGenerator => {
        name: "Coal Generator",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 1200.000000),
            (Rotor, 600.000000),
            (Cable, 1800.000000),
        ],
        outputs: [],
    },
    ColorCartridge => {
        name: "Color Cartridge",
        machine: Constructor,
        inputs: [
            (FlowerPetals, 50.000000),
        ],
        outputs: [
            (ColorCartridge, 100.000000),
        ],
    },
    Computer => {
        name: "Computer",
        machine: Manufacturer,
        inputs: [
            (CircuitBoard, 25.000000),
            (Cable, 22.500000),
            (Plastic, 45.000000),
            (Screw, 130.000000),
        ],
        outputs: [
            (Computer, 2.500000),
        ],
    },
    Concrete => {
        name: "Concrete",
        machine: Constructor,
        inputs: [
            (Limestone, 45.000000),
        ],
        outputs: [
            (Concrete, 15.000000),
        ],
    },
    Constructor => {
        name: "Constructor",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 120.000000),
            (Cable, 480.000000),
        ],
        outputs: [],
    },
    ConveyorBeltMk1 => {
        name: "Conveyor Belt Mk.1",
        machine: Manual,
        inputs: [
            (IronPlate, 60.000000),
        ],
        outputs: [],
    },
    ConveyorBeltMk2 => {
        name: "Conveyor Belt Mk.2",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 60.000000),
        ],
        outputs: [],
    },
    ConveyorBeltMk3 => {
        name: "Conveyor Belt Mk.3",
        machine: Manual,
        inputs: [
            (SteelBeam, 60.000000),
        ],
        outputs: [],
    },
    ConveyorBeltMk4 => {
        name: "Conveyor Belt Mk.4",
        machine: Manual,
        inputs: [
            (EncasedIndustrialBeam, 60.000000),
        ],
        outputs: [],
    },
    ConveyorBeltMk5 => {
        name: "Conveyor Belt Mk.5",
        machine: Manual,
        inputs: [
            (AlcladAluminumSheet, 60.000000),
        ],
        outputs: [],
    },
    ConveyorCeilingMount => {
        name: "Conveyor Ceiling Mount",
        machine: Manual,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 120.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    ConveyorLiftFloorHole => {
        name: "Conveyor Lift Floor Hole",
        machine: Manual,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 120.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    ConveyorLiftMk1 => {
        name: "Conveyor Lift Mk.1",
        machine: Manual,
        inputs: [
            (IronPlate, 120.000000),
        ],
        outputs: [],
    },
    ConveyorLiftMk2 => {
        name: "Conveyor Lift Mk.2",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 120.000000),
        ],
        outputs: [],
    },
    ConveyorLiftMk3 => {
        name: "Conveyor Lift Mk.3",
        machine: Manual,
        inputs: [
            (SteelBeam, 120.000000),
        ],
        outputs: [],
    },
    ConveyorLiftMk4 => {
        name: "Conveyor Lift Mk.4",
        machine: Manual,
        inputs: [
            (EncasedIndustrialBeam, 120.000000),
        ],
        outputs: [],
    },
    ConveyorLiftMk5 => {
        name: "Conveyor Lift Mk.5",
        machine: Manual,
        inputs: [
            (AlcladAluminumSheet, 120.000000),
        ],
        outputs: [],
    },
    ConveyorMerger => {
        name: "Conveyor Merger",
        machine: Manual,
        inputs: [
            (IronPlate, 120.000000),
            (IronRod, 120.000000),
        ],
        outputs: [],
    },
    ConveyorPole => {
        name: "Conveyor Pole",
        machine: Manual,
        inputs: [
            (IronRod, 60.000000),
            (IronPlate, 60.000000),
            (Concrete, 60.000000),
        ],
        outputs: [],
    },
    ConveyorSplitter => {
        name: "Conveyor Splitter",
        machine: Manual,
        inputs: [
            (IronPlate, 120.000000),
            (Cable, 120.000000),
        ],
        outputs: [],
    },
    ConveyorWallMount => {
        name: "Conveyor Wall Mount",
        machine: Manual,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 120.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    ConveyorWallX1 => {
        name: "Conveyor Wall x1",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    ConveyorWallX2 => {
        name: "Conveyor Wall x2",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    ConveyorWallX3 => {
        name: "Conveyor Wall x3",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    CoolingSystem => {
        name: "Cooling System",
        machine: Blender,
        inputs: [
            (HeatSink, 12.000000),
            (Rubber, 12.000000),
            (Water, 30.000000),
            (NitrogenGas, 150.000000),
        ],
        outputs: [
            (CoolingSystem, 6.000000),
        ],
    },
    CopperIngot => {
        name: "Copper Ingot",
        machine: Smelter,
        inputs: [
            (CopperOre, 30.000000),
        ],
        outputs: [
            (CopperIngot, 30.000000),
        ],
    },
    CopperPowder => {
        name: "Copper Powder",
        machine: Constructor,
        inputs: [
            (CopperIngot, 300.000000),
        ],
        outputs: [
            (CopperPowder, 50.000000),
        ],
    },
    CopperSheet => {
        name: "Copper Sheet",
        machine: Constructor,
        inputs: [
            (CopperIngot, 20.000000),
        ],
        outputs: [
            (CopperSheet, 10.000000),
        ],
    },
    CraftBench => {
        name: "Craft Bench",
        machine: Manual,
        inputs: [
            (IronPlate, 180.000000),
            (IronRod, 180.000000),
        ],
        outputs: [],
    },
    CrystalOscillator => {
        name: "Crystal Oscillator",
        machine: Manufacturer,
        inputs: [
            (QuartzCrystal, 18.000000),
            (Cable, 14.000000),
            (ReinforcedIronPlate, 2.500000),
        ],
        outputs: [
            (CrystalOscillator, 1.000000),
        ],
    },
    CyberWagon => {
        name: "Cyber Wagon",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 600.000000),
        ],
        outputs: [],
    },
    DisplaySign => {
        name: "Display Sign",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 120.000000),
            (QuartzCrystal, 300.000000),
        ],
        outputs: [],
    },
    DoubleRamp2m => {
        name: "Double Ramp 2m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    DoubleRamp4m => {
        name: "Double Ramp 4m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    DoubleRamp8m => {
        name: "Double Ramp 8m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    DoubleWallOutletMk1 => {
        name: "Double Wall Outlet Mk.1",
        machine: Manual,
        inputs: [
            (Wire, 480.000000),
            (IronRod, 120.000000),
        ],
        outputs: [],
    },
    DoubleWallOutletMk2 => {
        name: "Double Wall Outlet Mk.2",
        machine: Manual,
        inputs: [
            (Quickwire, 960.000000),
            (IronRod, 240.000000),
        ],
        outputs: [],
    },
    DoubleWallOutletMk3 => {
        name: "Double Wall Outlet Mk.3",
        machine: Manual,
        inputs: [
            (HighSpeedConnector, 360.000000),
            (SteelPipe, 360.000000),
        ],
        outputs: [],
    },
    DownCornerRamp1m => {
        name: "Down Corner Ramp 1m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    DownCornerRamp2m => {
        name: "Down Corner Ramp 2m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    DownCornerRamp4m => {
        name: "Down Corner Ramp 4m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    Drone => {
        name: "Drone",
        machine: Manual,
        inputs: [
            (Motor, 240.000000),
            (AlcladAluminumSheet, 600.000000),
            (RadioControlUnit, 60.000000),
            (AILimiter, 120.000000),
            (PortableMiner, 60.000000),
        ],
        outputs: [],
    },
    DronePort => {
        name: "Drone Port",
        machine: Manual,
        inputs: [
            (HeavyModularFrame, 1200.000000),
            (HighSpeedConnector, 1200.000000),
            (AlcladAluminumSheet, 3000.000000),
            (AluminumCasing, 3000.000000),
            (RadioControlUnit, 600.000000),
        ],
        outputs: [],
    },
    ElectricLocomotive => {
        name: "Electric Locomotive",
        machine: Manual,
        inputs: [
            (HeavyModularFrame, 300.000000),
            (Motor, 600.000000),
            (SteelPipe, 900.000000),
            (Computer, 300.000000),
        ],
        outputs: [],
    },
    ElectromagneticControlRod => {
        name: "Electromagnetic Control Rod",
        machine: Assembler,
        inputs: [
            (Stator, 6.000000),
            (AILimiter, 4.000000),
        ],
        outputs: [
            (ElectromagneticControlRod, 4.000000),
        ],
    },
    EmptyCanister => {
        name: "Empty Canister",
        machine: Constructor,
        inputs: [
            (Plastic, 30.000000),
        ],
        outputs: [
            (EmptyCanister, 60.000000),
        ],
    },
    EmptyFluidTank => {
        name: "Empty Fluid Tank",
        machine: Constructor,
        inputs: [
            (AluminumIngot, 60.000000),
        ],
        outputs: [
            (EmptyFluidTank, 60.000000),
        ],
    },
    EmptyPlatform => {
        name: "Empty Platform",
        machine: Manual,
        inputs: [
            (HeavyModularFrame, 360.000000),
            (Concrete, 3000.000000),
        ],
        outputs: [],
    },
    EmptyPlatformWithCatwalk => {
        name: "Empty Platform With Catwalk",
        machine: Manual,
        inputs: [
            (HeavyModularFrame, 360.000000),
            (Concrete, 3000.000000),
        ],
        outputs: [],
    },
    EncasedIndustrialBeam => {
        name: "Encased Industrial Beam",
        machine: Assembler,
        inputs: [
            (SteelBeam, 24.000000),
            (Concrete, 30.000000),
        ],
        outputs: [
            (EncasedIndustrialBeam, 6.000000),
        ],
    },
    EncasedPlutoniumCell => {
        name: "Encased Plutonium Cell",
        machine: Assembler,
        inputs: [
            (PlutoniumPellet, 10.000000),
            (Concrete, 20.000000),
        ],
        outputs: [
            (EncasedPlutoniumCell, 5.000000),
        ],
    },
    EncasedUraniumCell => {
        name: "Encased Uranium Cell",
        machine: Blender,
        inputs: [
            (Uranium, 50.000000),
            (Concrete, 15.000000),
            (SulfuricAcid, 40.000000),
        ],
        outputs: [
            (EncasedUraniumCell, 25.000000),
            (SulfuricAcid, 10.000000),
        ],
    },
    EquipmentWorkshop => {
        name: "Equipment Workshop",
        machine: Manual,
        inputs: [
            (IronPlate, 360.000000),
            (IronRod, 240.000000),
        ],
        outputs: [],
    },
    Explorer => {
        name: "Explorer",
        machine: Manual,
        inputs: [
            (CrystalOscillator, 300.000000),
            (Motor, 300.000000),
            (SteelPipe, 900.000000),
            (HeavyModularFrame, 300.000000),
        ],
        outputs: [],
    },
    ExplosiveRebar => {
        name: "Explosive Rebar",
        machine: Manufacturer,
        inputs: [
            (IronRebar, 10.000000),
            (SmokelessPowder, 10.000000),
            (SteelPipe, 10.000000),
        ],
        outputs: [
            (ExplosiveRebar, 5.000000),
        ],
    },
    FICSMASGiftTree => {
        name: "FICSMAS Gift Tree",
        machine: Constructor,
        inputs: [],
        outputs: [],
    },
    FICSMASPowerLight => {
        name: "FICSMAS Power Light",
        machine: Manual,
        inputs: [
            (Cable, 60.000000),
        ],
        outputs: [],
    },
    FICSMASSnowDispenser => {
        name: "FICSMAS Snow Dispenser",
        machine: Constructor,
        inputs: [],
        outputs: [],
    },
    FICSMASWreath => {
        name: "FICSMAS Wreath",
        machine: Constructor,
        inputs: [],
        outputs: [],
    },
    Fabric => {
        name: "Fabric",
        machine: Assembler,
        inputs: [
            (Mycelia, 15.000000),
            (Biomass, 75.000000),
        ],
        outputs: [
            (Fabric, 15.000000),
        ],
    },
    FactoryCart => {
        name: "Factory Cart™",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 12.000000),
            (IronRod, 12.000000),
            (Rotor, 6.000000),
        ],
        outputs: [
            (FactoryCart, 3.000000),
        ],
    },
    FloodLightTower => {
        name: "Flood Light Tower",
        machine: Manual,
        inputs: [
            (Quickwire, 1500.000000),
            (CopperSheet, 240.000000),
            (EncasedIndustrialBeam, 480.000000),
        ],
        outputs: [],
    },
    FluidBuffer => {
        name: "Fluid Buffer",
        machine: Manual,
        inputs: [
            (CopperSheet, 600.000000),
            (ModularFrame, 300.000000),
        ],
        outputs: [],
    },
    FluidFreightPlatform => {
        name: "Fluid Freight Platform",
        machine: Manual,
        inputs: [
            (HeavyModularFrame, 360.000000),
            (Computer, 120.000000),
            (Concrete, 3000.000000),
            (Cable, 1500.000000),
            (Motor, 300.000000),
        ],
        outputs: [],
    },
    Foundation1m => {
        name: "Foundation 1m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    Foundation2m => {
        name: "Foundation 2m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    Foundation4m => {
        name: "Foundation 4m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    Foundry => {
        name: "Foundry",
        machine: Manual,
        inputs: [
            (ModularFrame, 600.000000),
            (Rotor, 600.000000),
            (Concrete, 1200.000000),
        ],
        outputs: [],
    },
    FrameFloor => {
        name: "Frame Floor",
        machine: Manual,
        inputs: [
            (SteelBeam, 120.000000),
        ],
        outputs: [],
    },
    FrameFoundation => {
        name: "Frame Foundation",
        machine: Manual,
        inputs: [
            (SteelBeam, 300.000000),
        ],
        outputs: [],
    },
    FrameRamp => {
        name: "Frame Ramp",
        machine: Manual,
        inputs: [
            (SteelBeam, 300.000000),
        ],
        outputs: [],
    },
    FrameWall => {
        name: "Frame Wall",
        machine: Manual,
        inputs: [
            (SteelBeam, 120.000000),
        ],
        outputs: [],
    },
    FrameWindow => {
        name: "Frame Window",
        machine: Manual,
        inputs: [
            (Silica, 120.000000),
        ],
        outputs: [],
    },
    FreightCar => {
        name: "Freight Car",
        machine: Manual,
        inputs: [
            (HeavyModularFrame, 240.000000),
            (SteelPipe, 600.000000),
        ],
        outputs: [],
    },
    FreightPlatform => {
        name: "Freight Platform",
        machine: Manual,
        inputs: [
            (HeavyModularFrame, 360.000000),
            (Computer, 120.000000),
            (Concrete, 3000.000000),
            (Cable, 1500.000000),
            (Motor, 300.000000),
        ],
        outputs: [],
    },
    Fuel => {
        name: "Fuel",
        machine: Refinery,
        inputs: [
            (CrudeOil, 60.000000),
        ],
        outputs: [
            (Fuel, 40.000000),
            (PolymerResin, 30.000000),
        ],
    },
    FuelGenerator => {
        name: "Fuel Generator",
        machine: Manual,
        inputs: [
            (Computer, 300.000000),
            (HeavyModularFrame, 600.000000),
            (Motor, 900.000000),
            (Rubber, 3000.000000),
            (Quickwire, 3000.000000),
        ],
        outputs: [],
    },
    FullFrameWindow => {
        name: "Full Frame Window",
        machine: Manual,
        inputs: [
            (Silica, 120.000000),
            (SteelBeam, 120.000000),
        ],
        outputs: [],
    },
    FusedModularFrame => {
        name: "Fused Modular Frame",
        machine: Blender,
        inputs: [
            (HeavyModularFrame, 1.500000),
            (AluminumCasing, 75.000000),
            (NitrogenGas, 37.500000),
        ],
        outputs: [
            (FusedModularFrame, 1.500000),
        ],
    },
    GasFilter => {
        name: "Gas Filter",
        machine: Manufacturer,
        inputs: [
            (Coal, 37.500000),
            (Rubber, 15.000000),
            (Fabric, 15.000000),
        ],
        outputs: [
            (GasFilter, 7.500000),
        ],
    },
    GasMask => {
        name: "Gas Mask",
        machine: Manual,
        inputs: [
            (Rubber, 100.000000),
            (Plastic, 100.000000),
            (Fabric, 100.000000),
        ],
        outputs: [
            (GasMask, 1.000000),
        ],
    },
    GasNobelisk => {
        name: "Gas Nobelisk",
        machine: Assembler,
        inputs: [
            (Nobelisk, 5.000000),
            (Biomass, 50.000000),
        ],
        outputs: [
            (GasNobelisk, 5.000000),
        ],
    },
    GateHoleWall => {
        name: "Gate Hole Wall",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    GeothermalGenerator => {
        name: "Geothermal Generator",
        machine: Manual,
        inputs: [
            (Supercomputer, 480.000000),
            (HeavyModularFrame, 960.000000),
            (HighSpeedConnector, 960.000000),
            (CopperSheet, 2400.000000),
            (Rubber, 4800.000000),
        ],
        outputs: [],
    },
    GiantFICSMASTree => {
        name: "Giant FICSMAS Tree",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 3000.000000),
            (Concrete, 30000.000000),
        ],
        outputs: [],
    },
    GlassFrameFoundation => {
        name: "Glass Frame Foundation",
        machine: Manual,
        inputs: [
            (SteelBeam, 120.000000),
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    GoldenFactoryCart => {
        name: "Golden Factory Cart™",
        machine: Manual,
        inputs: [
            (CateriumIngot, 45.000000),
            (IronRod, 12.000000),
            (Rotor, 6.000000),
        ],
        outputs: [
            (GoldenFactoryCart, 3.000000),
        ],
    },
    Half1mFoundation => {
        name: "Half 1m Foundation",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    Half2mFoundation => {
        name: "Half 2m Foundation",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    Half4mFoundation => {
        name: "Half 4m Foundation",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    HatcherProtein => {
        name: "Hatcher Protein",
        machine: Constructor,
        inputs: [
            (HatcherRemains, 20.000000),
        ],
        outputs: [
            (AlienProtein, 20.000000),
        ],
    },
    HazardStorageBox => {
        name: "Hazard Storage Box",
        machine: Manual,
        inputs: [
            (IronPlate, 360.000000),
            (IronRod, 360.000000),
        ],
        outputs: [],
    },
    HazmatSuit => {
        name: "Hazmat Suit",
        machine: Manual,
        inputs: [
            (Rubber, 25.000000),
            (Plastic, 25.000000),
            (AlcladAluminumSheet, 25.000000),
            (Fabric, 25.000000),
        ],
        outputs: [
            (HazmatSuit, 0.500000),
        ],
    },
    HeatSink => {
        name: "Heat Sink",
        machine: Assembler,
        inputs: [
            (AlcladAluminumSheet, 37.500000),
            (CopperSheet, 22.500000),
        ],
        outputs: [
            (HeatSink, 7.500000),
        ],
    },
    HeavyModularFrame => {
        name: "Heavy Modular Frame",
        machine: Manufacturer,
        inputs: [
            (ModularFrame, 10.000000),
            (SteelPipe, 30.000000),
            (EncasedIndustrialBeam, 10.000000),
            (Screw, 200.000000),
        ],
        outputs: [
            (HeavyModularFrame, 2.000000),
        ],
    },
    HexFrameWindow => {
        name: "Hex Frame Window",
        machine: Manual,
        inputs: [
            (Silica, 120.000000),
            (SteelBeam, 120.000000),
        ],
        outputs: [],
    },
    HighSpeedConnector => {
        name: "High-Speed Connector",
        machine: Manufacturer,
        inputs: [
            (Quickwire, 210.000000),
            (Cable, 37.500000),
            (CircuitBoard, 3.750000),
        ],
        outputs: [
            (HighSpeedConnector, 3.750000),
        ],
    },
    HogProtein => {
        name: "Hog Protein",
        machine: Constructor,
        inputs: [
            (HogRemains, 20.000000),
        ],
        outputs: [
            (AlienProtein, 20.000000),
        ],
    },
    HomingRifleAmmo => {
        name: "Homing Rifle Ammo",
        machine: Assembler,
        inputs: [
            (RifleAmmo, 50.000000),
            (HighSpeedConnector, 2.500000),
        ],
        outputs: [
            (HomingRifleAmmo, 25.000000),
        ],
    },
    HoverPack => {
        name: "Hover Pack",
        machine: Constructor,
        inputs: [
            (Motor, 4.000000),
            (HeavyModularFrame, 2.000000),
            (Computer, 4.000000),
            (AlcladAluminumSheet, 20.000000),
        ],
        outputs: [
            (HoverPack, 0.500000),
        ],
    },
    Hypertube => {
        name: "Hypertube",
        machine: Manual,
        inputs: [
            (CopperSheet, 60.000000),
            (SteelPipe, 60.000000),
        ],
        outputs: [],
    },
    HypertubeEntrance => {
        name: "Hypertube Entrance",
        machine: Manual,
        inputs: [
            (EncasedIndustrialBeam, 240.000000),
            (Rotor, 240.000000),
            (SteelPipe, 600.000000),
        ],
        outputs: [],
    },
    HypertubeFloorHole => {
        name: "Hypertube Floor Hole",
        machine: Manual,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 120.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    HypertubeSupport => {
        name: "Hypertube Support",
        machine: Manual,
        inputs: [
            (IronPlate, 120.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    HypertubeWallHole => {
        name: "Hypertube Wall Hole",
        machine: Manual,
        inputs: [
            (IronPlate, 120.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    HypertubeWallSupport => {
        name: "Hypertube Wall Support",
        machine: Manual,
        inputs: [
            (IronPlate, 120.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    IndustrialFluidBuffer => {
        name: "Industrial Fluid Buffer",
        machine: Manual,
        inputs: [
            (Plastic, 1800.000000),
            (HeavyModularFrame, 180.000000),
        ],
        outputs: [],
    },
    IndustrialStorageContainer => {
        name: "Industrial Storage Container",
        machine: Manual,
        inputs: [
            (SteelBeam, 1200.000000),
            (SteelPipe, 1200.000000),
        ],
        outputs: [],
    },
    InnerCornerExtension1m => {
        name: "Inner Corner Extension 1m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InnerCornerExtension2m => {
        name: "Inner Corner Extension 2m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InnerCornerExtension4m => {
        name: "Inner Corner Extension 4m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InnerCornerQuarterPipe => {
        name: "Inner Corner Quarter Pipe",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InnerCornerRoof1m => {
        name: "Inner Corner Roof 1m",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
        ],
        outputs: [],
    },
    InnerCornerRoof2m => {
        name: "Inner Corner Roof 2m",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
        ],
        outputs: [],
    },
    InnerCornerRoof4m => {
        name: "Inner Corner Roof 4m",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
        ],
        outputs: [],
    },
    InvDownCorner1m => {
        name: "Inv. Down Corner 1m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InvDownCorner2m => {
        name: "Inv. Down Corner 2m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InvDownCorner4m => {
        name: "Inv. Down Corner 4m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InvRamp1m => {
        name: "Inv. Ramp 1m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InvRamp2m => {
        name: "Inv. Ramp 2m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InvRamp4m => {
        name: "Inv. Ramp 4m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InvRampWall1m => {
        name: "Inv. Ramp Wall 1m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    InvRampWall2m => {
        name: "Inv. Ramp Wall 2m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    InvRampWall4m => {
        name: "Inv. Ramp Wall 4m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    InvRampWall8m => {
        name: "Inv. Ramp Wall 8m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    InvUpCorner1m => {
        name: "Inv. Up Corner 1m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InvUpCorner2m => {
        name: "Inv. Up Corner 2m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InvUpCorner4m => {
        name: "Inv. Up Corner 4m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InvertedFrameRamp => {
        name: "Inverted Frame Ramp",
        machine: Manual,
        inputs: [
            (SteelBeam, 300.000000),
        ],
        outputs: [],
    },
    InvertedInnerCornerQuarterPipe => {
        name: "Inverted Inner Corner Quarter Pipe",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InvertedOuterCornerQuarterPipe => {
        name: "Inverted Outer Corner Quarter Pipe",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    InvertedQuarterPipe => {
        name: "Inverted Quarter Pipe",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    IodineInfusedFilter => {
        name: "Iodine Infused Filter",
        machine: Manual,
        inputs: [
            (GasFilter, 3.750000),
            (Quickwire, 30.000000),
            (AluminumCasing, 3.750000),
        ],
        outputs: [
            (IodineInfusedFilter, 3.750000),
        ],
    },
    IronIngot => {
        name: "Iron Ingot",
        machine: Smelter,
        inputs: [
            (IronOre, 30.000000),
        ],
        outputs: [
            (IronIngot, 30.000000),
        ],
    },
    IronPlate => {
        name: "Iron Plate",
        machine: Constructor,
        inputs: [
            (IronIngot, 30.000000),
        ],
        outputs: [
            (IronPlate, 20.000000),
        ],
    },
    IronRebar => {
        name: "Iron Rebar",
        machine: Constructor,
        inputs: [
            (IronRod, 15.000000),
        ],
        outputs: [
            (IronRebar, 15.000000),
        ],
    },
    IronRod => {
        name: "Iron Rod",
        machine: Constructor,
        inputs: [
            (IronIngot, 15.000000),
        ],
        outputs: [
            (IronRod, 15.000000),
        ],
    },
    Jetpack => {
        name: "Jetpack",
        machine: Manual,
        inputs: [
            (Plastic, 25.000000),
            (Rubber, 25.000000),
            (CircuitBoard, 7.500000),
            (Motor, 2.500000),
        ],
        outputs: [
            (Jetpack, 0.500000),
        ],
    },
    JumpPad => {
        name: "Jump Pad",
        machine: Manual,
        inputs: [
            (Rotor, 120.000000),
            (IronPlate, 900.000000),
            (Cable, 600.000000),
        ],
        outputs: [],
    },
    LabelSign2m => {
        name: "Label Sign 2m",
        machine: Manual,
        inputs: [
            (IronPlate, 120.000000),
            (QuartzCrystal, 120.000000),
        ],
        outputs: [],
    },
    LabelSign3m => {
        name: "Label Sign 3m",
        machine: Manual,
        inputs: [
            (IronPlate, 180.000000),
            (QuartzCrystal, 180.000000),
        ],
        outputs: [],
    },
    LabelSign4m => {
        name: "Label Sign 4m",
        machine: Manual,
        inputs: [
            (IronPlate, 240.000000),
            (QuartzCrystal, 240.000000),
        ],
        outputs: [],
    },
    Ladder => {
        name: "Ladder",
        machine: Manual,
        inputs: [
            (IronRod, 120.000000),
        ],
        outputs: [],
    },
    LargeBillboard => {
        name: "Large Billboard",
        machine: Manual,
        inputs: [
            (EncasedIndustrialBeam, 720.000000),
            (CopperSheet, 1200.000000),
            (CrystalOscillator, 300.000000),
        ],
        outputs: [],
    },
    LightsControlPanel => {
        name: "Lights Control Panel",
        machine: Manual,
        inputs: [
            (Cable, 600.000000),
            (ReinforcedIronPlate, 300.000000),
            (AILimiter, 180.000000),
        ],
        outputs: [],
    },
    LiquidBiofuel => {
        name: "Liquid Biofuel",
        machine: Refinery,
        inputs: [
            (SolidBiofuel, 90.000000),
            (Water, 45.000000),
        ],
        outputs: [
            (LiquidBiofuel, 60.000000),
        ],
    },
    LookoutTower => {
        name: "Lookout Tower",
        machine: Manual,
        inputs: [
            (IronPlate, 300.000000),
            (IronRod, 300.000000),
        ],
        outputs: [],
    },
    MAM => {
        name: "MAM",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 300.000000),
            (Cable, 900.000000),
            (Wire, 2700.000000),
        ],
        outputs: [],
    },
    MagneticFieldGenerator => {
        name: "Magnetic Field Generator",
        machine: Manufacturer,
        inputs: [
            (VersatileFramework, 2.500000),
            (ElectromagneticControlRod, 1.000000),
            (Battery, 5.000000),
        ],
        outputs: [
            (MagneticFieldGenerator, 1.000000),
        ],
    },
    Manufacturer => {
        name: "Manufacturer",
        machine: Manual,
        inputs: [
            (Motor, 300.000000),
            (HeavyModularFrame, 600.000000),
            (Cable, 3000.000000),
            (Plastic, 3000.000000),
        ],
        outputs: [],
    },
    MedicalStorageBox => {
        name: "Medical Storage Box",
        machine: Manual,
        inputs: [
            (IronPlate, 360.000000),
            (IronRod, 360.000000),
        ],
        outputs: [],
    },
    MetalBeam => {
        name: "Metal Beam",
        machine: Manual,
        inputs: [
            (SteelBeam, 120.000000),
        ],
        outputs: [],
    },
    MinerMk1 => {
        name: "Miner Mk.1",
        machine: Manual,
        inputs: [
            (PortableMiner, 60.000000),
            (IronPlate, 600.000000),
            (Concrete, 600.000000),
        ],
        outputs: [],
    },
    MinerMk2 => {
        name: "Miner Mk.2",
        machine: Manual,
        inputs: [
            (PortableMiner, 120.000000),
            (EncasedIndustrialBeam, 600.000000),
            (SteelPipe, 1200.000000),
            (ModularFrame, 600.000000),
        ],
        outputs: [],
    },
    MinerMk3 => {
        name: "Miner Mk.3",
        machine: Manual,
        inputs: [
            (PortableMiner, 180.000000),
            (SteelPipe, 3000.000000),
            (Supercomputer, 300.000000),
            (FusedModularFrame, 600.000000),
            (TurboMotor, 180.000000),
        ],
        outputs: [],
    },
    ModernRailing => {
        name: "Modern Railing",
        machine: Manual,
        inputs: [
            (IronRod, 120.000000),
        ],
        outputs: [],
    },
    ModularEngine => {
        name: "Modular Engine",
        machine: Manufacturer,
        inputs: [
            (Motor, 2.000000),
            (Rubber, 15.000000),
            (SmartPlating, 2.000000),
        ],
        outputs: [
            (ModularEngine, 1.000000),
        ],
    },
    ModularFrame => {
        name: "Modular Frame",
        machine: Assembler,
        inputs: [
            (ReinforcedIronPlate, 3.000000),
            (IronRod, 12.000000),
        ],
        outputs: [
            (ModularFrame, 2.000000),
        ],
    },
    Motor => {
        name: "Motor",
        machine: Assembler,
        inputs: [
            (Rotor, 10.000000),
            (Stator, 10.000000),
        ],
        outputs: [
            (Motor, 5.000000),
        ],
    },
    NitricAcid => {
        name: "Nitric Acid",
        machine: Blender,
        inputs: [
            (NitrogenGas, 120.000000),
            (Water, 30.000000),
            (IronPlate, 10.000000),
        ],
        outputs: [
            (NitricAcid, 30.000000),
        ],
    },
    Nobelisk => {
        name: "Nobelisk",
        machine: Assembler,
        inputs: [
            (BlackPowder, 20.000000),
            (SteelPipe, 20.000000),
        ],
        outputs: [
            (Nobelisk, 10.000000),
        ],
    },
    NobeliskDetonator => {
        name: "Nobelisk Detonator",
        machine: Manual,
        inputs: [
            (ObjectScanner, 0.750000),
            (SteelBeam, 7.500000),
            (Cable, 37.500000),
        ],
        outputs: [
            (NobeliskDetonator, 0.750000),
        ],
    },
    NonFissileUranium => {
        name: "Non-fissile Uranium",
        machine: Blender,
        inputs: [
            (UraniumWaste, 37.500000),
            (Silica, 25.000000),
            (NitricAcid, 15.000000),
            (SulfuricAcid, 15.000000),
        ],
        outputs: [
            (NonFissileUranium, 50.000000),
            (Water, 15.000000),
        ],
    },
    NuclearPasta => {
        name: "Nuclear Pasta",
        machine: ParticleAccelerator,
        inputs: [
            (CopperPowder, 100.000000),
            (PressureConversionCube, 0.500000),
        ],
        outputs: [
            (NuclearPasta, 0.500000),
        ],
    },
    NuclearPowerPlant => {
        name: "Nuclear Power Plant",
        machine: Manual,
        inputs: [
            (Concrete, 15000.000000),
            (HeavyModularFrame, 1500.000000),
            (Supercomputer, 300.000000),
            (Cable, 6000.000000),
            (AlcladAluminumSheet, 6000.000000),
        ],
        outputs: [],
    },
    NukeNobelisk => {
        name: "Nuke Nobelisk",
        machine: Manufacturer,
        inputs: [
            (Nobelisk, 2.500000),
            (EncasedUraniumCell, 10.000000),
            (SmokelessPowder, 5.000000),
            (AILimiter, 3.000000),
        ],
        outputs: [
            (NukeNobelisk, 0.500000),
        ],
    },
    NutritionalInhaler => {
        name: "Nutritional Inhaler",
        machine: Constructor,
        inputs: [
            (BaconAgaric, 3.000000),
            (Paleberry, 6.000000),
            (BerylNut, 15.000000),
        ],
        outputs: [
            (MedicinalInhaler, 3.000000),
        ],
    },
    ObjectScanner => {
        name: "Object Scanner",
        machine: Constructor,
        inputs: [
            (ReinforcedIronPlate, 6.000000),
            (Wire, 30.000000),
            (Screw, 75.000000),
        ],
        outputs: [
            (ObjectScanner, 1.500000),
        ],
    },
    OilExtractor => {
        name: "Oil Extractor",
        machine: Manual,
        inputs: [
            (Motor, 900.000000),
            (EncasedIndustrialBeam, 1200.000000),
            (Cable, 3600.000000),
        ],
        outputs: [],
    },
    OuterCornerExtension1m => {
        name: "Outer Corner Extension 1m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    OuterCornerExtension2m => {
        name: "Outer Corner Extension 2m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    OuterCornerExtension4m => {
        name: "Outer Corner Extension 4m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    OuterCornerQuarterPipe => {
        name: "Outer Corner Quarter Pipe",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    OuterCornerRoof1m => {
        name: "Outer Corner Roof 1m",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
        ],
        outputs: [],
    },
    OuterCornerRoof2m => {
        name: "Outer Corner Roof 2m",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
        ],
        outputs: [],
    },
    OuterCornerRoof4m => {
        name: "Outer Corner Roof 4m",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
        ],
        outputs: [],
    },
    PackagedAluminaSolution => {
        name: "Packaged Alumina Solution",
        machine: Packager,
        inputs: [
            (AluminaSolution, 120.000000),
            (EmptyCanister, 120.000000),
        ],
        outputs: [
            (PackagedAluminaSolution, 120.000000),
        ],
    },
    PackagedFuel => {
        name: "Packaged Fuel",
        machine: Packager,
        inputs: [
            (Fuel, 40.000000),
            (EmptyCanister, 40.000000),
        ],
        outputs: [
            (PackagedFuel, 40.000000),
        ],
    },
    PackagedHeavyOilResidue => {
        name: "Packaged Heavy Oil Residue",
        machine: Packager,
        inputs: [
            (HeavyOilResidue, 30.000000),
            (EmptyCanister, 30.000000),
        ],
        outputs: [
            (PackagedHeavyOilResidue, 30.000000),
        ],
    },
    PackagedLiquidBiofuel => {
        name: "Packaged Liquid Biofuel",
        machine: Packager,
        inputs: [
            (LiquidBiofuel, 40.000000),
            (EmptyCanister, 40.000000),
        ],
        outputs: [
            (PackagedLiquidBiofuel, 40.000000),
        ],
    },
    PackagedNitricAcid => {
        name: "Packaged Nitric Acid",
        machine: Packager,
        inputs: [
            (NitricAcid, 30.000000),
            (EmptyFluidTank, 30.000000),
        ],
        outputs: [
            (PackagedNitricAcid, 30.000000),
        ],
    },
    PackagedNitrogenGas => {
        name: "Packaged Nitrogen Gas",
        machine: Packager,
        inputs: [
            (NitrogenGas, 240.000000),
            (EmptyFluidTank, 60.000000),
        ],
        outputs: [
            (PackagedNitrogenGas, 60.000000),
        ],
    },
    PackagedOil => {
        name: "Packaged Oil",
        machine: Packager,
        inputs: [
            (CrudeOil, 30.000000),
            (EmptyCanister, 30.000000),
        ],
        outputs: [
            (PackagedOil, 30.000000),
        ],
    },
    PackagedSulfuricAcid => {
        name: "Packaged Sulfuric Acid",
        machine: Packager,
        inputs: [
            (SulfuricAcid, 40.000000),
            (EmptyCanister, 40.000000),
        ],
        outputs: [
            (PackagedSulfuricAcid, 40.000000),
        ],
    },
    PackagedTurbofuel => {
        name: "Packaged Turbofuel",
        machine: Packager,
        inputs: [
            (Turbofuel, 20.000000),
            (EmptyCanister, 20.000000),
        ],
        outputs: [
            (PackagedTurbofuel, 20.000000),
        ],
    },
    PackagedWater => {
        name: "Packaged Water",
        machine: Packager,
        inputs: [
            (Water, 60.000000),
            (EmptyCanister, 60.000000),
        ],
        outputs: [
            (PackagedWater, 60.000000),
        ],
    },
    Packager => {
        name: "Packager",
        machine: Manual,
        inputs: [
            (SteelBeam, 1200.000000),
            (Rubber, 600.000000),
            (Plastic, 600.000000),
        ],
        outputs: [],
    },
    PaintedBeam => {
        name: "Painted Beam",
        machine: Manual,
        inputs: [
            (SteelBeam, 120.000000),
        ],
        outputs: [],
    },
    PanelWindow => {
        name: "Panel Window",
        machine: Manual,
        inputs: [
            (Silica, 120.000000),
        ],
        outputs: [],
    },
    Parachute => {
        name: "Parachute",
        machine: Constructor,
        inputs: [
            (Fabric, 30.000000),
            (Cable, 15.000000),
        ],
        outputs: [
            (Parachute, 1.500000),
        ],
    },
    ParticleAccelerator => {
        name: "Particle Accelerator",
        machine: Manual,
        inputs: [
            (RadioControlUnit, 1500.000000),
            (ElectromagneticControlRod, 6000.000000),
            (Supercomputer, 600.000000),
            (CoolingSystem, 3000.000000),
            (FusedModularFrame, 1200.000000),
            (TurboMotor, 600.000000),
        ],
        outputs: [],
    },
    PathSignal => {
        name: "Path Signal",
        machine: Manual,
        inputs: [
            (SteelPipe, 120.000000),
            (CopperSheet, 60.000000),
            (Computer, 60.000000),
        ],
        outputs: [],
    },
    PersonalStorageBox => {
        name: "Personal Storage Box",
        machine: Manual,
        inputs: [
            (IronPlate, 360.000000),
            (IronRod, 360.000000),
        ],
        outputs: [],
    },
    PetroleumCoke => {
        name: "Petroleum Coke",
        machine: Refinery,
        inputs: [
            (HeavyOilResidue, 40.000000),
        ],
        outputs: [
            (PetroleumCoke, 120.000000),
        ],
    },
    PipelineFloorHole => {
        name: "Pipeline Floor Hole",
        machine: Manual,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 120.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    PipelineJunctionCross => {
        name: "Pipeline Junction Cross",
        machine: Manual,
        inputs: [
            (CopperSheet, 300.000000),
        ],
        outputs: [],
    },
    PipelineMk1 => {
        name: "Pipeline Mk.1",
        machine: Manual,
        inputs: [
            (CopperSheet, 60.000000),
        ],
        outputs: [],
    },
    PipelineMk1NoIndicator => {
        name: "Pipeline Mk.1 (No Indicator)",
        machine: Manual,
        inputs: [
            (CopperSheet, 60.000000),
        ],
        outputs: [],
    },
    PipelineMk2 => {
        name: "Pipeline Mk.2",
        machine: Manual,
        inputs: [
            (CopperSheet, 120.000000),
            (Plastic, 60.000000),
        ],
        outputs: [],
    },
    PipelineMk2NoIndicator => {
        name: "Pipeline Mk.2 (No Indicator)",
        machine: Manual,
        inputs: [
            (CopperSheet, 120.000000),
            (Plastic, 60.000000),
        ],
        outputs: [],
    },
    PipelinePumpMk1 => {
        name: "Pipeline Pump Mk.1",
        machine: Manual,
        inputs: [
            (CopperSheet, 120.000000),
            (Rotor, 120.000000),
        ],
        outputs: [],
    },
    PipelinePumpMk2 => {
        name: "Pipeline Pump Mk.2",
        machine: Manual,
        inputs: [
            (Motor, 120.000000),
            (EncasedIndustrialBeam, 240.000000),
            (Plastic, 480.000000),
        ],
        outputs: [],
    },
    PipelineSupport => {
        name: "Pipeline Support",
        machine: Manual,
        inputs: [
            (IronPlate, 120.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    PipelineWallHole => {
        name: "Pipeline Wall Hole",
        machine: Manual,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 60.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    PipelineWallSupport => {
        name: "Pipeline Wall Support",
        machine: Manual,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 60.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    Plastic => {
        name: "Plastic",
        machine: Refinery,
        inputs: [
            (CrudeOil, 30.000000),
        ],
        outputs: [
            (Plastic, 20.000000),
            (HeavyOilResidue, 10.000000),
        ],
    },
    PlutoniumFuelRod => {
        name: "Plutonium Fuel Rod",
        machine: Manufacturer,
        inputs: [
            (EncasedPlutoniumCell, 7.500000),
            (SteelBeam, 4.500000),
            (ElectromagneticControlRod, 1.500000),
            (HeatSink, 2.500000),
        ],
        outputs: [
            (PlutoniumFuelRod, 0.250000),
        ],
    },
    PlutoniumPellet => {
        name: "Plutonium Pellet",
        machine: ParticleAccelerator,
        inputs: [
            (NonFissileUranium, 100.000000),
            (UraniumWaste, 25.000000),
        ],
        outputs: [
            (PlutoniumPellet, 30.000000),
        ],
    },
    PortableMiner => {
        name: "Portable Miner",
        machine: Assembler,
        inputs: [
            (IronPlate, 3.000000),
            (IronRod, 6.000000),
        ],
        outputs: [
            (PortableMiner, 1.500000),
        ],
    },
    PortraitSign => {
        name: "Portrait Sign",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 240.000000),
            (QuartzCrystal, 600.000000),
        ],
        outputs: [],
    },
    PowerLine => {
        name: "Power Line",
        machine: Manual,
        inputs: [
            (Cable, 60.000000),
        ],
        outputs: [],
    },
    PowerPoleMk1 => {
        name: "Power Pole Mk.1",
        machine: Manual,
        inputs: [
            (Wire, 180.000000),
            (IronRod, 60.000000),
            (Concrete, 60.000000),
        ],
        outputs: [],
    },
    PowerPoleMk2 => {
        name: "Power Pole Mk.2",
        machine: Manual,
        inputs: [
            (Quickwire, 360.000000),
            (IronRod, 120.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    PowerPoleMk3 => {
        name: "Power Pole Mk.3",
        machine: Manual,
        inputs: [
            (HighSpeedConnector, 120.000000),
            (SteelPipe, 120.000000),
            (Rubber, 180.000000),
        ],
        outputs: [],
    },
    PowerShard1 => {
        name: "Power Shard (1)",
        machine: Constructor,
        inputs: [
            (BluePowerSlug, 7.500000),
        ],
        outputs: [
            (PowerShard, 7.500000),
        ],
    },
    PowerShard2 => {
        name: "Power Shard (2)",
        machine: Constructor,
        inputs: [
            (YellowPowerSlug, 5.000000),
        ],
        outputs: [
            (PowerShard, 10.000000),
        ],
    },
    PowerShard5 => {
        name: "Power Shard (5)",
        machine: Constructor,
        inputs: [
            (PurplePowerSlug, 2.500000),
        ],
        outputs: [
            (PowerShard, 12.500000),
        ],
    },
    PowerStorage => {
        name: "Power Storage",
        machine: Manual,
        inputs: [
            (Wire, 6000.000000),
            (ModularFrame, 600.000000),
            (Stator, 300.000000),
        ],
        outputs: [],
    },
    PowerSwitch => {
        name: "Power Switch",
        machine: Manual,
        inputs: [
            (Quickwire, 1200.000000),
            (SteelBeam, 240.000000),
            (AILimiter, 60.000000),
        ],
        outputs: [],
    },
    PowerTower => {
        name: "Power Tower",
        machine: Manual,
        inputs: [
            (Concrete, 600.000000),
            (Wire, 1200.000000),
            (SteelBeam, 1200.000000),
        ],
        outputs: [],
    },
    PowerTowerPlatform => {
        name: "Power Tower Platform",
        machine: Manual,
        inputs: [
            (Concrete, 600.000000),
            (Wire, 1200.000000),
            (SteelBeam, 1200.000000),
        ],
        outputs: [],
    },
    PressureConversionCube => {
        name: "Pressure Conversion Cube",
        machine: Assembler,
        inputs: [
            (FusedModularFrame, 1.000000),
            (RadioControlUnit, 2.000000),
        ],
        outputs: [
            (PressureConversionCube, 1.000000),
        ],
    },
    PriorityPowerSwitch => {
        name: "Priority Power Switch",
        machine: Manual,
        inputs: [
            (CircuitBoard, 240.000000),
            (SteelBeam, 360.000000),
            (HighSpeedConnector, 120.000000),
        ],
        outputs: [],
    },
    ProgrammableSplitter => {
        name: "Programmable Splitter",
        machine: Manual,
        inputs: [
            (HeavyModularFrame, 60.000000),
            (Motor, 60.000000),
            (Supercomputer, 60.000000),
        ],
        outputs: [],
    },
    ProteinInhaler => {
        name: "Protein Inhaler",
        machine: Constructor,
        inputs: [
            (AlienProtein, 3.000000),
            (BerylNut, 30.000000),
        ],
        outputs: [
            (MedicinalInhaler, 3.000000),
        ],
    },
    PulseNobelisk => {
        name: "Pulse Nobelisk",
        machine: Assembler,
        inputs: [
            (Nobelisk, 5.000000),
            (CrystalOscillator, 1.000000),
        ],
        outputs: [
            (PulseNobelisk, 5.000000),
        ],
    },
    QuarterPipe => {
        name: "Quarter Pipe",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    QuartzCrystal => {
        name: "Quartz Crystal",
        machine: Constructor,
        inputs: [
            (RawQuartz, 37.500000),
        ],
        outputs: [
            (QuartzCrystal, 22.500000),
        ],
    },
    Quickwire => {
        name: "Quickwire",
        machine: Constructor,
        inputs: [
            (CateriumIngot, 12.000000),
        ],
        outputs: [
            (Quickwire, 60.000000),
        ],
    },
    RadarTower => {
        name: "Radar Tower",
        machine: Manual,
        inputs: [
            (HeavyModularFrame, 1800.000000),
            (CrystalOscillator, 1800.000000),
            (CircuitBoard, 600.000000),
            (Cable, 6000.000000),
        ],
        outputs: [],
    },
    RadioControlUnit => {
        name: "Radio Control Unit",
        machine: Manufacturer,
        inputs: [
            (AluminumCasing, 40.000000),
            (CrystalOscillator, 1.250000),
            (Computer, 1.250000),
        ],
        outputs: [
            (RadioControlUnit, 2.500000),
        ],
    },
    Railway => {
        name: "Railway",
        machine: Manual,
        inputs: [
            (SteelPipe, 60.000000),
            (SteelBeam, 60.000000),
        ],
        outputs: [],
    },
    Ramp1m => {
        name: "Ramp 1m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    Ramp2m => {
        name: "Ramp 2m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    Ramp4m => {
        name: "Ramp 4m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    RampWall1m => {
        name: "Ramp Wall 1m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    RampWall2m => {
        name: "Ramp Wall 2m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    RampWall4m => {
        name: "Ramp Wall 4m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    RampWall8m => {
        name: "Ramp Wall 8m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    RebarGun => {
        name: "Rebar Gun",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 6.000000),
            (IronRod, 16.000000),
            (Screw, 100.000000),
        ],
        outputs: [
            (RebarGun, 1.000000),
        ],
    },
    Refinery => {
        name: "Refinery",
        machine: Manual,
        inputs: [
            (Motor, 600.000000),
            (EncasedIndustrialBeam, 600.000000),
            (SteelPipe, 1800.000000),
            (CopperSheet, 1200.000000),
        ],
        outputs: [],
    },
    ReinforcedIronPlate => {
        name: "Reinforced Iron Plate",
        machine: Assembler,
        inputs: [
            (IronPlate, 30.000000),
            (Screw, 60.000000),
        ],
        outputs: [
            (ReinforcedIronPlate, 5.000000),
        ],
    },
    ReinforcedWindow => {
        name: "Reinforced Window",
        machine: Manual,
        inputs: [
            (Silica, 120.000000),
        ],
        outputs: [],
    },
    ResidualFuel => {
        name: "Residual Fuel",
        machine: Refinery,
        inputs: [
            (HeavyOilResidue, 60.000000),
        ],
        outputs: [
            (Fuel, 40.000000),
        ],
    },
    ResidualPlastic => {
        name: "Residual Plastic",
        machine: Refinery,
        inputs: [
            (PolymerResin, 60.000000),
            (Water, 20.000000),
        ],
        outputs: [
            (Plastic, 20.000000),
        ],
    },
    ResidualRubber => {
        name: "Residual Rubber",
        machine: Refinery,
        inputs: [
            (PolymerResin, 40.000000),
            (Water, 40.000000),
        ],
        outputs: [
            (Rubber, 20.000000),
        ],
    },
    ResourceWellExtractor => {
        name: "Resource Well Extractor",
        machine: Manual,
        inputs: [
            (SteelBeam, 600.000000),
            (Plastic, 600.000000),
        ],
        outputs: [],
    },
    ResourceWellPressurizer => {
        name: "Resource Well Pressurizer",
        machine: Manual,
        inputs: [
            (Wire, 12000.000000),
            (Rubber, 3000.000000),
            (EncasedIndustrialBeam, 3000.000000),
            (Motor, 3000.000000),
        ],
        outputs: [],
    },
    Rifle => {
        name: "Rifle",
        machine: Constructor,
        inputs: [
            (Motor, 1.000000),
            (Rubber, 5.000000),
            (SteelPipe, 12.500000),
            (Screw, 125.000000),
        ],
        outputs: [
            (Rifle, 0.500000),
        ],
    },
    RifleAmmo => {
        name: "Rifle Ammo",
        machine: Assembler,
        inputs: [
            (CopperSheet, 15.000000),
            (SmokelessPowder, 10.000000),
        ],
        outputs: [
            (RifleAmmo, 75.000000),
        ],
    },
    RoadBarrier => {
        name: "Road Barrier",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    Roof1m => {
        name: "Roof 1m",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
        ],
        outputs: [],
    },
    Roof2m => {
        name: "Roof 2m",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
        ],
        outputs: [],
    },
    Roof4m => {
        name: "Roof 4m",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
        ],
        outputs: [],
    },
    RoofFlat => {
        name: "Roof Flat",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
        ],
        outputs: [],
    },
    Rotor => {
        name: "Rotor",
        machine: Assembler,
        inputs: [
            (IronRod, 20.000000),
            (Screw, 100.000000),
        ],
        outputs: [
            (Rotor, 4.000000),
        ],
    },
    Rubber => {
        name: "Rubber",
        machine: Refinery,
        inputs: [
            (CrudeOil, 30.000000),
        ],
        outputs: [
            (Rubber, 20.000000),
            (HeavyOilResidue, 20.000000),
        ],
    },
    Screw => {
        name: "Screw",
        machine: Constructor,
        inputs: [
            (IronRod, 10.000000),
        ],
        outputs: [
            (Screw, 40.000000),
        ],
    },
    ShatterRebar => {
        name: "Shatter Rebar",
        machine: Assembler,
        inputs: [
            (IronRebar, 10.000000),
            (QuartzCrystal, 15.000000),
        ],
        outputs: [
            (ShatterRebar, 5.000000),
        ],
    },
    SideDoorWall => {
        name: "Side Door Wall",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    Silica => {
        name: "Silica",
        machine: Constructor,
        inputs: [
            (RawQuartz, 22.500000),
        ],
        outputs: [
            (Silica, 37.500000),
        ],
    },
    SingleWindow => {
        name: "Single Window",
        machine: Manual,
        inputs: [
            (Silica, 120.000000),
        ],
        outputs: [],
    },
    SmallBillboard => {
        name: "Small Billboard",
        machine: Manual,
        inputs: [
            (EncasedIndustrialBeam, 180.000000),
            (CopperSheet, 240.000000),
            (CrystalOscillator, 60.000000),
        ],
        outputs: [],
    },
    SmallConcretePillar => {
        name: "Small Concrete Pillar",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
        ],
        outputs: [],
    },
    SmallFramePillar => {
        name: "Small Frame Pillar",
        machine: Manual,
        inputs: [
            (SteelBeam, 180.000000),
        ],
        outputs: [],
    },
    SmallMetalPillar => {
        name: "Small Metal Pillar",
        machine: Manual,
        inputs: [
            (IronPlate, 60.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    SmallPillarSupport => {
        name: "Small Pillar Support",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
        ],
        outputs: [],
    },
    SmartPlating => {
        name: "Smart Plating",
        machine: Assembler,
        inputs: [
            (ReinforcedIronPlate, 2.000000),
            (Rotor, 2.000000),
        ],
        outputs: [
            (SmartPlating, 2.000000),
        ],
    },
    SmartSplitter => {
        name: "Smart Splitter",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 120.000000),
            (Rotor, 120.000000),
            (AILimiter, 60.000000),
        ],
        outputs: [],
    },
    Smelter => {
        name: "Smelter",
        machine: Manual,
        inputs: [
            (IronRod, 300.000000),
            (Wire, 480.000000),
        ],
        outputs: [],
    },
    SmokelessPowder => {
        name: "Smokeless Powder",
        machine: Refinery,
        inputs: [
            (BlackPowder, 20.000000),
            (HeavyOilResidue, 10.000000),
        ],
        outputs: [
            (SmokelessPowder, 20.000000),
        ],
    },
    Snowman => {
        name: "Snowman",
        machine: Manual,
        inputs: [],
        outputs: [],
    },
    SolidBiofuel => {
        name: "Solid Biofuel",
        machine: Constructor,
        inputs: [
            (Biomass, 120.000000),
        ],
        outputs: [
            (SolidBiofuel, 60.000000),
        ],
    },
    SpaceElevator => {
        name: "Space Elevator",
        machine: Manual,
        inputs: [
            (Concrete, 30000.000000),
            (IronPlate, 15000.000000),
            (IronRod, 24000.000000),
            (Wire, 90000.000000),
        ],
        outputs: [],
    },
    SpitterProtein => {
        name: "Spitter Protein",
        machine: Constructor,
        inputs: [
            (PlasmaSpitterRemains, 20.000000),
        ],
        outputs: [
            (AlienProtein, 20.000000),
        ],
    },
    SquareSign05m => {
        name: "Square Sign 0.5m",
        machine: Manual,
        inputs: [
            (IronPlate, 60.000000),
            (QuartzCrystal, 60.000000),
        ],
        outputs: [],
    },
    SquareSign1m => {
        name: "Square Sign 1m",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 60.000000),
            (QuartzCrystal, 300.000000),
        ],
        outputs: [],
    },
    SquareSign2m => {
        name: "Square Sign 2m",
        machine: Manual,
        inputs: [
            (ReinforcedIronPlate, 180.000000),
            (QuartzCrystal, 600.000000),
        ],
        outputs: [],
    },
    StackableConveyorPole => {
        name: "Stackable Conveyor Pole",
        machine: Manual,
        inputs: [
            (IronRod, 120.000000),
            (IronPlate, 120.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    StackableHypertubeSupport => {
        name: "Stackable Hypertube Support",
        machine: Manual,
        inputs: [
            (IronPlate, 240.000000),
            (IronRod, 120.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    StackablePipelineSupport => {
        name: "Stackable Pipeline Support",
        machine: Manual,
        inputs: [
            (IronPlate, 240.000000),
            (IronRod, 120.000000),
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    StairsLeft => {
        name: "Stairs Left",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
            (IronPlate, 180.000000),
        ],
        outputs: [],
    },
    StairsRight => {
        name: "Stairs Right",
        machine: Manual,
        inputs: [
            (Concrete, 180.000000),
            (IronPlate, 180.000000),
        ],
        outputs: [],
    },
    Stator => {
        name: "Stator",
        machine: Assembler,
        inputs: [
            (SteelPipe, 15.000000),
            (Wire, 40.000000),
        ],
        outputs: [
            (Stator, 5.000000),
        ],
    },
    SteelBeam => {
        name: "Steel Beam",
        machine: Constructor,
        inputs: [
            (SteelIngot, 60.000000),
        ],
        outputs: [
            (SteelBeam, 15.000000),
        ],
    },
    SteelIngot => {
        name: "Steel Ingot",
        machine: Foundry,
        inputs: [
            (IronOre, 45.000000),
            (Coal, 45.000000),
        ],
        outputs: [
            (SteelIngot, 45.000000),
        ],
    },
    SteelPipe => {
        name: "Steel Pipe",
        machine: Constructor,
        inputs: [
            (SteelIngot, 30.000000),
        ],
        outputs: [
            (SteelPipe, 20.000000),
        ],
    },
    StingerProtein => {
        name: "Stinger Protein",
        machine: Constructor,
        inputs: [
            (StingerRemains, 20.000000),
        ],
        outputs: [
            (AlienProtein, 20.000000),
        ],
    },
    StorageContainer => {
        name: "Storage Container",
        machine: Manual,
        inputs: [
            (IronPlate, 600.000000),
            (IronRod, 600.000000),
        ],
        outputs: [],
    },
    StreetLight => {
        name: "Street Light",
        machine: Manual,
        inputs: [
            (Quickwire, 600.000000),
            (Wire, 240.000000),
            (IronRod, 240.000000),
        ],
        outputs: [],
    },
    StunRebar => {
        name: "Stun Rebar",
        machine: Assembler,
        inputs: [
            (IronRebar, 10.000000),
            (Quickwire, 50.000000),
        ],
        outputs: [
            (StunRebar, 10.000000),
        ],
    },
    SulfuricAcid => {
        name: "Sulfuric Acid",
        machine: Refinery,
        inputs: [
            (Sulfur, 50.000000),
            (Water, 50.000000),
        ],
        outputs: [
            (SulfuricAcid, 50.000000),
        ],
    },
    Supercomputer => {
        name: "Supercomputer",
        machine: Manufacturer,
        inputs: [
            (Computer, 3.750000),
            (AILimiter, 3.750000),
            (HighSpeedConnector, 5.625000),
            (Plastic, 52.500000),
        ],
        outputs: [
            (Supercomputer, 1.875000),
        ],
    },
    TheHUB => {
        name: "The HUB",
        machine: Manual,
        inputs: [
            (HUBParts, 60.000000),
        ],
        outputs: [],
    },
    TherapeuticInhaler => {
        name: "Therapeutic Inhaler",
        machine: Constructor,
        inputs: [
            (Mycelia, 45.000000),
            (AlienProtein, 3.000000),
            (BaconAgaric, 3.000000),
        ],
        outputs: [
            (MedicinalInhaler, 3.000000),
        ],
    },
    ThermalPropulsionRocket => {
        name: "Thermal Propulsion Rocket",
        machine: Manufacturer,
        inputs: [
            (ModularEngine, 2.500000),
            (TurboMotor, 1.000000),
            (CoolingSystem, 3.000000),
            (FusedModularFrame, 1.000000),
        ],
        outputs: [
            (ThermalPropulsionRocket, 1.000000),
        ],
    },
    TiltedConcaveWall4m => {
        name: "Tilted Concave Wall 4m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    TiltedConcaveWall8m => {
        name: "Tilted Concave Wall 8m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    TiltedCornerWall4m => {
        name: "Tilted Corner Wall 4m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    TiltedCornerWall8m => {
        name: "Tilted Corner Wall 8m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    TiltedWall4m => {
        name: "Tilted Wall 4m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    TiltedWall8m => {
        name: "Tilted Wall 8m",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    Tractor => {
        name: "Tractor",
        machine: Manual,
        inputs: [
            (ModularFrame, 300.000000),
            (Rotor, 300.000000),
            (ReinforcedIronPlate, 600.000000),
        ],
        outputs: [],
    },
    TrainStation => {
        name: "Train Station",
        machine: Manual,
        inputs: [
            (HeavyModularFrame, 240.000000),
            (Computer, 480.000000),
            (Concrete, 3000.000000),
            (Cable, 1500.000000),
        ],
        outputs: [],
    },
    Truck => {
        name: "Truck",
        machine: Manual,
        inputs: [
            (Motor, 900.000000),
            (CircuitBoard, 600.000000),
            (HeavyModularFrame, 300.000000),
            (Rubber, 3000.000000),
            (EncasedIndustrialBeam, 1200.000000),
        ],
        outputs: [],
    },
    TruckStation => {
        name: "Truck Station",
        machine: Manual,
        inputs: [
            (ModularFrame, 900.000000),
            (Rotor, 1200.000000),
            (Cable, 3000.000000),
        ],
        outputs: [],
    },
    TurboMotor => {
        name: "Turbo Motor",
        machine: Manufacturer,
        inputs: [
            (CoolingSystem, 7.500000),
            (RadioControlUnit, 3.750000),
            (Motor, 7.500000),
            (Rubber, 45.000000),
        ],
        outputs: [
            (TurboMotor, 1.875000),
        ],
    },
    TurboRifleAmmo => {
        name: "Turbo Rifle Ammo",
        machine: Manufacturer,
        inputs: [
            (RifleAmmo, 125.000000),
            (AluminumCasing, 15.000000),
            (PackagedTurbofuel, 15.000000),
        ],
        outputs: [
            (TurboRifleAmmo, 250.000000),
        ],
    },
    Turbofuel => {
        name: "Turbofuel",
        machine: Refinery,
        inputs: [
            (Fuel, 22.500000),
            (CompactedCoal, 15.000000),
        ],
        outputs: [
            (Turbofuel, 18.750000),
        ],
    },
    UJellyLandingPad => {
        name: "U-Jelly Landing Pad",
        machine: Manual,
        inputs: [
            (Rotor, 120.000000),
            (Cable, 1200.000000),
            (Biomass, 12000.000000),
        ],
        outputs: [],
    },
    UnpackageAluminaSolution => {
        name: "Unpackage Alumina Solution",
        machine: Packager,
        inputs: [
            (PackagedAluminaSolution, 120.000000),
        ],
        outputs: [
            (AluminaSolution, 120.000000),
            (EmptyCanister, 120.000000),
        ],
    },
    UnpackageFuel => {
        name: "Unpackage Fuel",
        machine: Packager,
        inputs: [
            (PackagedFuel, 60.000000),
        ],
        outputs: [
            (Fuel, 60.000000),
            (EmptyCanister, 60.000000),
        ],
    },
    UnpackageHeavyOilResidue => {
        name: "Unpackage Heavy Oil Residue",
        machine: Packager,
        inputs: [
            (PackagedHeavyOilResidue, 20.000000),
        ],
        outputs: [
            (HeavyOilResidue, 20.000000),
            (EmptyCanister, 20.000000),
        ],
    },
    UnpackageLiquidBiofuel => {
        name: "Unpackage Liquid Biofuel",
        machine: Packager,
        inputs: [
            (PackagedLiquidBiofuel, 60.000000),
        ],
        outputs: [
            (LiquidBiofuel, 60.000000),
            (EmptyCanister, 60.000000),
        ],
    },
    UnpackageNitricAcid => {
        name: "Unpackage Nitric Acid",
        machine: Packager,
        inputs: [
            (PackagedNitricAcid, 20.000000),
        ],
        outputs: [
            (NitricAcid, 20.000000),
            (EmptyFluidTank, 20.000000),
        ],
    },
    UnpackageNitrogenGas => {
        name: "Unpackage Nitrogen Gas",
        machine: Packager,
        inputs: [
            (PackagedNitrogenGas, 60.000000),
        ],
        outputs: [
            (NitrogenGas, 240.000000),
            (EmptyFluidTank, 60.000000),
        ],
    },
    UnpackageOil => {
        name: "Unpackage Oil",
        machine: Packager,
        inputs: [
            (PackagedOil, 60.000000),
        ],
        outputs: [
            (CrudeOil, 60.000000),
            (EmptyCanister, 60.000000),
        ],
    },
    UnpackageSulfuricAcid => {
        name: "Unpackage Sulfuric Acid",
        machine: Packager,
        inputs: [
            (PackagedSulfuricAcid, 60.000000),
        ],
        outputs: [
            (SulfuricAcid, 60.000000),
            (EmptyCanister, 60.000000),
        ],
    },
    UnpackageTurbofuel => {
        name: "Unpackage Turbofuel",
        machine: Packager,
        inputs: [
            (PackagedTurbofuel, 20.000000),
        ],
        outputs: [
            (Turbofuel, 20.000000),
            (EmptyCanister, 20.000000),
        ],
    },
    UnpackageWater => {
        name: "Unpackage Water",
        machine: Packager,
        inputs: [
            (PackagedWater, 120.000000),
        ],
        outputs: [
            (Water, 120.000000),
            (EmptyCanister, 120.000000),
        ],
    },
    UpCornerRamp1m => {
        name: "Up Corner Ramp 1m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    UpCornerRamp2m => {
        name: "Up Corner Ramp 2m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    UpCornerRamp4m => {
        name: "Up Corner Ramp 4m",
        machine: Manual,
        inputs: [
            (Concrete, 300.000000),
        ],
        outputs: [],
    },
    UraniumFuelRod => {
        name: "Uranium Fuel Rod",
        machine: Manufacturer,
        inputs: [
            (EncasedUraniumCell, 20.000000),
            (EncasedIndustrialBeam, 1.200000),
            (ElectromagneticControlRod, 2.000000),
        ],
        outputs: [
            (UraniumFuelRod, 0.400000),
        ],
    },
    Valve => {
        name: "Valve",
        machine: Manual,
        inputs: [
            (Rubber, 240.000000),
            (SteelBeam, 240.000000),
        ],
        outputs: [],
    },
    VersatileFramework => {
        name: "Versatile Framework",
        machine: Assembler,
        inputs: [
            (ModularFrame, 2.500000),
            (SteelBeam, 30.000000),
        ],
        outputs: [
            (VersatileFramework, 5.000000),
        ],
    },
    VitaminInhaler => {
        name: "Vitamin Inhaler",
        machine: Constructor,
        inputs: [
            (Mycelia, 30.000000),
            (Paleberry, 15.000000),
        ],
        outputs: [
            (MedicinalInhaler, 3.000000),
        ],
    },
    Wall1a => {
        name: "Wall 1a",
        machine: Manual,
        inputs: [
            (Concrete, 120.000000),
        ],
        outputs: [],
    },
    WallMountedFloodLight => {
        name: "Wall Mounted Flood Light",
        machine: Manual,
        inputs: [
            (Quickwire, 1500.000000),
            (CopperSheet, 240.000000),
            (EncasedIndustrialBeam, 120.000000),
        ],
        outputs: [],
    },
    WallOutletMk1 => {
        name: "Wall Outlet Mk.1",
        machine: Manual,
        inputs: [
            (Wire, 240.000000),
            (IronRod, 60.000000),
        ],
        outputs: [],
    },
    WallOutletMk2 => {
        name: "Wall Outlet Mk.2",
        machine: Manual,
        inputs: [
            (Quickwire, 480.000000),
            (IronRod, 120.000000),
        ],
        outputs: [],
    },
    WallOutletMk3 => {
        name: "Wall Outlet Mk.3",
        machine: Manual,
        inputs: [
            (HighSpeedConnector, 180.000000),
            (SteelPipe, 180.000000),
        ],
        outputs: [],
    },
    WaterExtractor => {
        name: "Water Extractor",
        machine: Manual,
        inputs: [
            (CopperSheet, 1200.000000),
            (ReinforcedIronPlate, 600.000000),
            (Rotor, 600.000000),
        ],
        outputs: [],
    },
    Wire => {
        name: "Wire",
        machine: Constructor,
        inputs: [
            (CopperIngot, 15.000000),
        ],
        outputs: [
            (Wire, 30.000000),
        ],
    },
    XenoBasher => {
        name: "Xeno-Basher",
        machine: Constructor,
        inputs: [
            (ModularFrame, 3.750000),
            (XenoZapper, 1.500000),
            (Cable, 18.750000),
            (Wire, 375.000000),
        ],
        outputs: [
            (XenoBasher, 0.750000),
        ],
    },
    XenoZapper => {
        name: "Xeno-Zapper",
        machine: Constructor,
        inputs: [
            (IronRod, 15.000000),
            (ReinforcedIronPlate, 3.000000),
            (Cable, 22.500000),
            (Wire, 75.000000),
        ],
        outputs: [
            (XenoZapper, 1.500000),
        ],
    },
    Zipline => {
        name: "Zipline",
        machine: Constructor,
        inputs: [
            (XenoZapper, 1.500000),
            (Quickwire, 45.000000),
            (IronRod, 4.500000),
            (Cable, 15.000000),
        ],
        outputs: [
            (Zipline, 1.500000),
        ],
    },
        }
    };
}
