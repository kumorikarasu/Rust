use crate::{event::Event, event_aggregator::EventAggregator, in_memory_event_aggregator::InMemoryEventAggregator};

mod event;
mod aggregation_result;
mod event_aggregator;
mod test;
mod in_memory_event_aggregator;
mod hll;

fn main() {
        let mut event_aggregator = InMemoryEventAggregator::new(true, false);

        // Act
        // Generate 10 million events
        // Create a random timestamp between 500 and 3000
        for i in 0..10_000_000 {
            let random_timestamp = rand::random::<u64>() % 5000;
            event_aggregator.collect_event(Event { name: "game1".to_string(), event_name: "event1".to_string(), timestamp_millis: random_timestamp, });
        }

        // assert
        let result = event_aggregator.get_aggregation_result("game1", "event1");
        println!("Result: {:?}", result);
        assert_eq!(result.is_some(), true);
}
