// Minimal WASM bootstrap and singleton access to WebTracker
// Expects wasm-pack output in `frontend/src/wasm/satisflow_wasm.js`

let trackerPromise: Promise<any> | null = null;

export async function getTracker() {
  if (!trackerPromise) {
    trackerPromise = (async () => {
      // Dynamically import the wasm module
      const wasm = await import('../wasm/satisflow_wasm.js');
      const wt = new wasm.WebTracker();
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
