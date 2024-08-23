use std::collections::HashMap;

use crate::{aggregation_result::AggregationResult, event::Event, event_aggregator::EventAggregator};

pub struct InMemoryEventAggregator { 
    pub timestamps: HashMap<String, Vec<u64>>,
    pub result: HashMap<String, AggregationResult>

}

impl InMemoryEventAggregator {
    pub fn new() -> InMemoryEventAggregator {
        InMemoryEventAggregator {
            timestamps: HashMap::new(),
            result: HashMap::new()
        }
    }
}

impl EventAggregator for InMemoryEventAggregator {
    fn collect_event(&mut self, event: crate::event::Event) {
        todo!();
    }

    fn get_aggregation_result(&self, name: &str, event_name: &str) -> Option<crate::aggregation_result::AggregationResult> {
        todo!();
    }
}
