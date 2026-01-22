use std::collections::HashMap;

use hyperloglog::HyperLogLog;
use crate::hll::HyperLogLog as myHll;

use crate::{aggregation_result::AggregationResult, event::Event, event_aggregator::EventAggregator};

pub struct InMemoryEventAggregator { 
    pub timestamps: HashMap<String, Vec<u64>>,
    pub tshll: HashMap<String, HyperLogLog>,
    pub result: HashMap<String, AggregationResult>,
    pub my_hll: HashMap<String, myHll>,
    use_hll: bool,
    hard_timestamp: bool

}

impl InMemoryEventAggregator {
    pub fn new(hll: bool, timestamps: bool) -> InMemoryEventAggregator {
        InMemoryEventAggregator {
            timestamps: HashMap::new(),
            tshll: HashMap::new(),
            result: HashMap::new(),
            my_hll: HashMap::new(),
            use_hll: hll,
            hard_timestamp: timestamps
        }
    }
}

impl EventAggregator for InMemoryEventAggregator {
    fn collect_event(&mut self, event: crate::event::Event) {
        let key = event.name.clone() + &event.event_name + &event.event_name + &event.name;
        // We put the event_name twice in the key to make sure something like
        // game1/event1 and ga/me1event1 are not considered the same.

        let aggregation = self.result.entry(key.clone()).or_insert(AggregationResult {
            name: event.name.clone(),
            event_name: event.event_name.clone(),
            total_events: 0,
            earlist_timestamp_millis: std::u64::MAX,
            latest_timestamp_millis: 0,
            unique_timestamps: 0,
            unique_timestamps_hll: 0,
            unique_timestamps_my_hll: 0
        });

        if self.use_hll {
            //let hll = self.tshll.entry(key.clone()).or_insert_with(|| HyperLogLog::new(0.023));
            //hll.insert(&event.timestamp_millis);
            //aggregation.unique_timestamps_hll = hll.len() as u64;

            let my_hll = self.my_hll.entry(key.clone()).or_insert_with(|| myHll::new(2048));
            my_hll.insert(&event.timestamp_millis);
            //aggregation.unique_timestamps_my_hll = my_hll.len() as u64;
        } 
        if self.hard_timestamp {
            let timestamp = self.timestamps.entry(key.clone()).or_insert(Vec::new());
            if ! timestamp.contains(&event.timestamp_millis) {
                timestamp.push(event.timestamp_millis);
            }
            aggregation.unique_timestamps = timestamp.len() as u64;
        }

        aggregation.total_events += 1;
        aggregation.earlist_timestamp_millis = std::cmp::min(aggregation.earlist_timestamp_millis, event.timestamp_millis);
        aggregation.latest_timestamp_millis = std::cmp::max(aggregation.latest_timestamp_millis, event.timestamp_millis);
    }

    fn get_aggregation_result(&self, name: &str, event_name: &str) -> Option<crate::aggregation_result::AggregationResult> {
        let key = name.to_string() + event_name + event_name + name;
        self.result.get(&key).map(|x| {
            let mut res = x.clone();
            let my_hll = self.my_hll.get(&key);
            if my_hll.is_some() {
                res.unique_timestamps_my_hll = my_hll.unwrap().len() as u64;
            }

            let thll = self.tshll.get(&key);
            if thll.is_some() {
                res.unique_timestamps_hll = thll.unwrap().len() as u64;
            }

            res
        })
    }
}
