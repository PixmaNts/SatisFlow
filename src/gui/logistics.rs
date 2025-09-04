use dioxus::prelude::*;
use crate::models::*;

#[component]
pub fn Logistics(fluxes: Vec<LogisticsFlux>) -> Element {
    rsx! {
        div {
            class: "p-4",
            h2 { class: "text-2xl font-bold mb-4", "Logistics Management" }
            
            div { class: "mb-4",
                button {
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    onclick: |_| {
                        // TODO: Open dialog to add new logistics flux
                    },
                    "Add New Logistics Flux"
                }
            }
            
            div { class: "overflow-x-auto",
                table { class: "min-w-full bg-white border border-gray-300",
                    thead { class: "bg-gray-50",
                        tr {
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "ID" }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "From" }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "To" }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Item" }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Qty/min" }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Transport" }
                        }
                    }
                    tbody { class: "divide-y divide-gray-200",
                        for flux in fluxes {
                            tr { class: "hover:bg-gray-50",
                                td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900", 
                                    "{flux.id}"
                                }
                                td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                    "{flux.from_factory}"
                                }
                                td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                    "{flux.to_factory}"
                                }
                                td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                    "{flux.item}"
                                }
                                td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                    "{flux.quantity_per_min:.1}"
                                }
                                td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                    match flux.transport_type {
                                        TransportType::Conveyor => "🔗 Conveyor",
                                        TransportType::Train => "🚂 Train",
                                        TransportType::Truck => "🚚 Truck",
                                        TransportType::Drone => "🚁 Drone"
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