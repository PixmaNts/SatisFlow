mod models;
mod gui;
mod engine;
mod data;

use dioxus::prelude::*;
use std::collections::HashMap;

use models::*;
use engine::*;
use gui::*;

#[derive(Clone, PartialEq)]
pub enum AppView {
    Overview,
    Logistics,
    Factory,
}

#[component]
fn App() -> Element {
    let mut tracker = use_signal(|| ProductionTracker::new());
    let mut is_loading = use_signal(|| true);
    let mut current_view = use_signal(|| AppView::Overview);
    
    // Initialize data loading
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            let mut t = ProductionTracker::new();
            if let Err(e) = t.load_game_data().await {
                web_sys::console::error_1(&format!("Failed to load game data: {}", e).into());
            }
            // Add demo data
            add_demo_data(&mut t);
            tracker.set(t);
            is_loading.set(false);
        });
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut t = ProductionTracker::new();
            if let Err(e) = t.load_game_data() {
                eprintln!("Failed to load game data: {}", e);
            }
            // Add demo data
            add_demo_data(&mut t);
            tracker.set(t);
            is_loading.set(false);
        }
    });
    
    rsx! {
        div {
            class: "min-h-screen bg-gray-100",
            
            // Navigation
            nav {
                class: "bg-blue-600 text-white shadow-md",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div {
                        class: "flex justify-between h-16",
                        div {
                            class: "flex items-center space-x-8",
                            h1 { class: "text-xl font-bold", "SatisFlow" }
                            
                            div { class: "flex space-x-4",
                                button {
                                    class: if *current_view.read() == AppView::Overview { 
                                        "px-3 py-2 rounded-md text-sm font-medium bg-blue-700" 
                                    } else { 
                                        "px-3 py-2 rounded-md text-sm font-medium hover:bg-blue-700" 
                                    },
                                    onclick: move |_| current_view.set(AppView::Overview),
                                    "Overview"
                                }
                                button {
                                    class: if *current_view.read() == AppView::Logistics { 
                                        "px-3 py-2 rounded-md text-sm font-medium bg-blue-700" 
                                    } else { 
                                        "px-3 py-2 rounded-md text-sm font-medium hover:bg-blue-700" 
                                    },
                                    onclick: move |_| current_view.set(AppView::Logistics),
                                    "Logistics"
                                }
                                button {
                                    class: if *current_view.read() == AppView::Factory { 
                                        "px-3 py-2 rounded-md text-sm font-medium bg-blue-700" 
                                    } else { 
                                        "px-3 py-2 rounded-md text-sm font-medium hover:bg-blue-700" 
                                    },
                                    onclick: move |_| current_view.set(AppView::Factory),
                                    "Factory"
                                }
                            }
                        }
                        
                        div { class: "flex items-center space-x-4",
                            button {
                                class: "bg-green-500 hover:bg-green-600 px-4 py-2 rounded-md text-sm font-medium",
                                onclick: move |_| {
                                    if let Err(e) = crate::engine::save_tracker(&tracker.read()) {
                                        eprintln!("Failed to save: {}", e);
                                    }
                                },
                                "Save"
                            }
                            button {
                                class: "bg-blue-500 hover:bg-blue-600 px-4 py-2 rounded-md text-sm font-medium",
                                onclick: move |_| {
                                    match crate::engine::load_tracker() {
                                        Ok(loaded_tracker) => *tracker.write() = loaded_tracker,
                                        Err(e) => eprintln!("Failed to load: {}", e),
                                    }
                                },
                                "Load"
                            }
                        }
                    }
                }
            }
            
            // Main content
            main {
                class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
                if *is_loading.read() {
                    div {
                        class: "flex justify-center items-center h-64",
                        div {
                            class: "text-center",
                            div {
                                class: "inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"
                            }
                            div {
                                class: "mt-4 text-gray-600",
                                "Loading game data..."
                            }
                        }
                    }
                } else {
                    match *current_view.read() {
                        AppView::Overview => rsx! {
                            Overview { summaries: tracker.read().calculate_overview() }
                        },
                        AppView::Logistics => rsx! {
                            Logistics { fluxes: tracker.read().logistics_fluxes.values().cloned().collect() }
                        },
                        AppView::Factory => rsx! {
                            FactoryView { factories: tracker.read().factories.values().cloned().collect() }
                        }
                    }
                }
            }
        }
    }
}

fn add_demo_data(tracker: &mut ProductionTracker) {
    let demo_factory = Factory {
        id: "demo_factory_1".to_string(),
        name: "Iron Processing Plant".to_string(),
        raw_inputs: vec![
            RawInput {
                item: "Iron Ore".to_string(),
                quantity_per_min: 120.0,
                source_type: "Iron Miner Mk.1".to_string(),
            },
        ],
        logistics_inputs: vec![],
        production_lines: vec![
            ProductionLine {
                id: "iron_ingot_line_1".to_string(),
                factory_id: "demo_factory_1".to_string(),
                recipe_name: "Iron Ingot".to_string(),
                machine_count: 2,
                clock_ratio: 1.0,
                group_name: Some("Smelting".to_string()),
                output_routing: HashMap::new(),
            },
            ProductionLine {
                id: "iron_plate_line_1".to_string(),
                factory_id: "demo_factory_1".to_string(),
                recipe_name: "Iron Plate".to_string(),
                machine_count: 1,
                clock_ratio: 1.0,
                group_name: Some("Parts".to_string()),
                output_routing: HashMap::new(),
            },
            ProductionLine {
                id: "iron_rod_line_1".to_string(),
                factory_id: "demo_factory_1".to_string(),
                recipe_name: "Iron Rod".to_string(),
                machine_count: 1,
                clock_ratio: 1.0,
                group_name: Some("Parts".to_string()),
                output_routing: HashMap::new(),
            },
        ],
    };
    
    tracker.add_factory(demo_factory);
    
    let demo_flux = LogisticsFlux {
        id: "LG-BUS-001-01".to_string(),
        from_factory: "demo_factory_1".to_string(),
        to_factory: "steel_factory".to_string(),
        item: "Iron Plate".to_string(),
        quantity_per_min: 15.0,
        transport_type: TransportType::Conveyor,
        transport_details: "Bus 001, Conveyor 01".to_string(),
    };
    
    tracker.add_logistics_flux(demo_flux);
}

fn main() {
    dioxus::launch(App);
}
