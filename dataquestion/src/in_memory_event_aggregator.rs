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
        let key = event.name.clone() + &event.event_name;

        let timestamp = self.timestamps.entry(key.clone()).or_insert(Vec::new());

        if ! timestamp.contains(&event.timestamp_millis) {
            timestamp.push(event.timestamp_millis);
        }

        let aggregation = self.result.entry(key).or_insert(AggregationResult {
            name: event.name.clone(),
            event_name: event.event_name.clone(),
            total_events: 0,
            earlist_timestamp_millis: std::u64::MAX,
            latest_timestamp_millis: 0,
            unique_timestamps: 0
        });

        aggregation.total_events += 1;
        aggregation.earlist_timestamp_millis = std::cmp::min(aggregation.earlist_timestamp_millis, event.timestamp_millis);
        aggregation.latest_timestamp_millis = std::cmp::max(aggregation.latest_timestamp_millis, event.timestamp_millis);
        aggregation.unique_timestamps = timestamp.len() as u64;

        println!("Aggregation: {:?}", aggregation);
    }

    fn get_aggregation_result(&self, name: &str, event_name: &str) -> Option<crate::aggregation_result::AggregationResult> {
        let key = name.to_string() + event_name;
        self.result.get(&key).map(|x| x.clone())
    }
}
