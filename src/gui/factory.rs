use dioxus::prelude::*;
use crate::models::*;

#[component]
pub fn FactoryView(factories: Vec<Factory>) -> Element {
    rsx! {
        div {
            class: "p-4",
            h2 { class: "text-2xl font-bold mb-4", "Factory Management" }
            
            div { class: "mb-4",
                button {
                    class: "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded",
                    onclick: |_| {
                        // TODO: Open dialog to add new factory
                    },
                    "Add New Factory"
                }
            }
            
            div { class: "space-y-4",
                for factory in factories {
                    div { class: "bg-white border border-gray-300 rounded-lg p-6 shadow-sm",
                        div { class: "flex justify-between items-center mb-4",
                            h3 { class: "text-xl font-semibold text-gray-900", "{factory.name}" }
                            div { class: "space-x-2",
                                button {
                                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-3 rounded text-sm",
                                    onclick: |_| {
                                        // TODO: Open factory editor
                                    },
                                    "Edit"
                                }
                                button {
                                    class: "bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-3 rounded text-sm",
                                    onclick: |_| {
                                        // TODO: Delete factory with confirmation
                                    },
                                    "Delete"
                                }
                            }
                        }
                        
                        div { class: "grid grid-cols-3 gap-4 mb-4",
                            div { class: "text-sm text-gray-600",
                                "Raw Inputs: ", strong { "{factory.raw_inputs.len()}" }
                            }
                            div { class: "text-sm text-gray-600",
                                "Production Lines: ", strong { "{factory.production_lines.len()}" }
                            }
                            div { class: "text-sm text-gray-600",
                                "Logistics Inputs: ", strong { "{factory.logistics_inputs.len()}" }
                            }
                        }
                        
                        if !factory.production_lines.is_empty() {
                            details {
                                summary { class: "cursor-pointer font-medium text-gray-700 hover:text-gray-900",
                                    "Production Lines"
                                }
                                div { class: "mt-2 space-y-2",
                                    for line in &factory.production_lines {
                                        div { class: "flex items-center space-x-4 text-sm text-gray-600 bg-gray-50 p-2 rounded",
                                            span { class: "font-medium", "{line.id}" }
                                            span { "•" }
                                            span { "{line.recipe_name}" }
                                            span { "•" }
                                            span { "{line.machine_count}x machines" }
                                            span { "•" }
                                            span { "{line.clock_ratio * 100.0:.1}% clock" }
                                            if let Some(ref group) = line.group_name {
                                                span { "•" }
                                                span { class: "bg-blue-100 text-blue-800 px-2 py-1 rounded-full text-xs", "{group}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}