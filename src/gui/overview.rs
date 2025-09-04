use dioxus::prelude::*;
use crate::models::*;

#[component]
pub fn Overview(summaries: Vec<ItemSummary>) -> Element {
    rsx! {
        div {
            class: "p-4",
            h2 { class: "text-2xl font-bold mb-4", "Production Overview" }
            
            if summaries.is_empty() {
                div { class: "text-gray-500", "No production data yet. Add some factories and production lines!" }
            } else {
                div { class: "overflow-x-auto",
                    table { class: "min-w-full bg-white border border-gray-300",
                        thead { class: "bg-gray-50",
                            tr {
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Item" }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Produced/min" }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Consumed/min" }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Available/min" }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Status" }
                            }
                        }
                        tbody { class: "divide-y divide-gray-200",
                            for summary in summaries {
                                tr { class: "hover:bg-gray-50",
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900", 
                                        "{summary.item_name}"
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        "{summary.total_produced_per_min:.1}"
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        "{summary.total_consumed_per_min:.1}"
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        "{summary.available_per_min:.1}"
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm",
                                        match summary.status {
                                            ProductionStatus::Balanced => rsx! { span { class: "text-green-600", "✅ Balanced" } },
                                            ProductionStatus::Overflow => rsx! { span { class: "text-yellow-600", "⚠️ Overflow" } },
                                            ProductionStatus::Underflow => rsx! { span { class: "text-red-600", "❌ Underflow" } }
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