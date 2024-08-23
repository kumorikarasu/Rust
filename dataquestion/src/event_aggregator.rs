use crate::{aggregation_result::AggregationResult, event::Event};


pub trait EventAggregator {
    fn collect_event(&mut self, event: Event);
    fn get_aggregation_result(&self, name: &str, event_name: &str) -> Option<AggregationResult>;
}
