// Minimal WASM bootstrap and singleton access to WebTracker
// Expects wasm-pack output in `frontend/src/wasm/satisflow_wasm.js`

let trackerPromise: Promise<any> | null = null;

export async function getTracker() {
  if (!trackerPromise) {
    trackerPromise = (async () => {
      // Dynamically import the wasm module
      const wasmModule = await import('../wasm/satisflow_wasm.js');
      
      // Initialize the WASM module first
      await wasmModule.default();
      
      // Now we can create the WebTracker
      const wt = new wasmModule.WebTracker();
      wt.load_static_data();
      return wt;
    })();
  }
  return trackerPromise;
}

export async function getCounts() {
  const t = await getTracker();
  return {
    factories: t.get_factory_count(),
    lines: t.get_production_line_count(),
    items: t.get_item_count?.() ?? 0,
  };
}

export async function exportJson(): Promise<string> {
  const t = await getTracker();
  return t.export_json();
}

export async function importJson(json: string): Promise<void> {
  const t = await getTracker();
  await t.import_json(json);
}

export async function loadSample(): Promise<void> {
  const t = await getTracker();
  t.load_sample_data();
}

export type OverviewItem = {
  item_id: string
  item_name: string
  total_produced_per_min: number
  total_consumed_per_min: number
  available_per_min: number
  status: 'Balanced' | 'Overflow' | 'Underflow'
}

export async function getOverview(): Promise<OverviewItem[]> {
  const t = await getTracker();
  const jsv = await t.get_overview();
  // serde-wasm-bindgen returns native JS values already
  return jsv as OverviewItem[];
}

export async function getFactories(): Promise<any[]> {
  const t = await getTracker();
  return t.get_factories();
}

export async function getRecipes(): Promise<any[]> {
  const t = await getTracker();
  return t.get_recipes();
}

export async function getItems(): Promise<any[]> {
  const t = await getTracker();
  return t.get_items();
}

export async function createFactory(name: string): Promise<any> {
  const t = await getTracker();
  return t.create_factory(name);
}

export async function addProductionLine(lineData: any): Promise<void> {
  const t = await getTracker();
  return t.add_production_line(lineData);
}

export async function updateProductionLine(lineId: string, lineData: any): Promise<void> {
  const t = await getTracker();
  return t.update_production_line(lineId, lineData);
}

export async function addLogisticsFlux(fluxData: any): Promise<void> {
  const t = await getTracker();
  return t.add_logistics_flux(fluxData);
}

export async function generateLineId(factoryId: string, recipeName: string): Promise<string> {
  const t = await getTracker();
  return t.generate_line_id(factoryId, recipeName);
}

export async function generateLogisticsId(transportType: string): Promise<string> {
  const t = await getTracker();
  return t.generate_logistics_id(transportType);
}

export async function getLogisticsFluxes(): Promise<any[]> {
  const t = await getTracker();
  return t.get_logistics_fluxes();
}

export async function updateLogisticsFlux(fluxId: string, fluxData: any): Promise<void> {
  const t = await getTracker();
  return t.update_logistics_flux(fluxId, fluxData);
}

export async function removeLogisticsFlux(fluxId: string): Promise<void> {
  const t = await getTracker();
  return t.remove_logistics_flux(fluxId);
}

export async function addRawInput(factoryId: string, rawInputData: any): Promise<void> {
  const t = await getTracker();
  return t.add_raw_input(factoryId, rawInputData);
}

export async function removeRawInput(factoryId: string, itemId: string): Promise<void> {
  const t = await getTracker();
  return t.remove_raw_input(factoryId, itemId);
}

export async function updateRawInput(factoryId: string, itemId: string, rawInputData: any): Promise<void> {
  const t = await getTracker();
  return t.update_raw_input(factoryId, itemId, rawInputData);
}
