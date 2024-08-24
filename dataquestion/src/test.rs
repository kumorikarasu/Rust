
#[cfg(test)]
mod tests {
    use crate::{event::Event, event_aggregator::EventAggregator, in_memory_event_aggregator::InMemoryEventAggregator};

    #[test]
    fn consume_and_aggregate_single_event() {
        // Arrange
        let mut event_aggregator = InMemoryEventAggregator::new(false, true);
        let event = Event {
            name: "game1".to_string(),
            event_name: "event1".to_string(),
            timestamp_millis: 1000,
        };

        // Act
        event_aggregator.collect_event(event);

        // Assert
        let result = event_aggregator.get_aggregation_result("noGame", "shouldBeNone");
        assert_eq!(result.is_none(), true);

        let result = event_aggregator.get_aggregation_result("game1", "event1");
        assert_eq!(result.is_some(), true);
        assert_eq!(result.clone().unwrap().total_events, 1);
        assert_eq!(result.clone().unwrap().earlist_timestamp_millis, 1000);
        assert_eq!(result.clone().unwrap().latest_timestamp_millis, 1000);
        assert_eq!(result.clone().unwrap().unique_timestamps, 1);
    }

    #[test]
    fn consume_and_aggregate_multiple_events(){
        // Arrange
        let mut event_aggregator = InMemoryEventAggregator::new(false, true);

        // Act
        event_aggregator.collect_event(Event { name: "game1".to_string(), event_name: "event1".to_string(), timestamp_millis: 500, });
        event_aggregator.collect_event(Event { name: "game1".to_string(), event_name: "event1".to_string(), timestamp_millis: 750, });
        event_aggregator.collect_event(Event { name: "game1".to_string(), event_name: "event1".to_string(), timestamp_millis: 250, });
        event_aggregator.collect_event(Event { name: "game1".to_string(), event_name: "event1".to_string(), timestamp_millis: 250, });
        event_aggregator.collect_event(Event { name: "game1".to_string(), event_name: "event1".to_string(), timestamp_millis: 1000, });
        event_aggregator.collect_event(Event { name: "game2".to_string(), event_name: "event1".to_string(), timestamp_millis: 1250, });
        event_aggregator.collect_event(Event { name: "game2".to_string(), event_name: "event1".to_string(), timestamp_millis: 750, });
        event_aggregator.collect_event(Event { name: "game1".to_string(), event_name: "event2".to_string(), timestamp_millis: 1250, });
        event_aggregator.collect_event(Event { name: "game1".to_string(), event_name: "event2".to_string(), timestamp_millis: 600, });
        event_aggregator.collect_event(Event { name: "game1".to_string(), event_name: "event2".to_string(), timestamp_millis: 600, });
        event_aggregator.collect_event(Event { name: "game1".to_string(), event_name: "event2".to_string(), timestamp_millis: 1200, });
        event_aggregator.collect_event(Event { name: "game2".to_string(), event_name: "event2".to_string(), timestamp_millis: 1200, });

        // Assert
        let mut result = event_aggregator.get_aggregation_result("noGame", "shouldBeNone");
        assert_eq!(result.is_none(), true);

        result = event_aggregator.get_aggregation_result("game1", "event1");
        assert_eq!(result.is_some(), true);
        assert_eq!(result.clone().unwrap().name, "game1");
        assert_eq!(result.clone().unwrap().event_name, "event1");
        assert_eq!(result.clone().unwrap().total_events, 5);
        assert_eq!(result.clone().unwrap().earlist_timestamp_millis, 250);
        assert_eq!(result.clone().unwrap().latest_timestamp_millis, 1000);
        assert_eq!(result.clone().unwrap().unique_timestamps, 4);

        result = event_aggregator.get_aggregation_result("game2", "event1");
        assert_eq!(result.is_some(), true);
        assert_eq!(result.clone().unwrap().name, "game2");
        assert_eq!(result.clone().unwrap().event_name, "event1");
        assert_eq!(result.clone().unwrap().total_events, 2);
        assert_eq!(result.clone().unwrap().earlist_timestamp_millis, 750);
        assert_eq!(result.clone().unwrap().latest_timestamp_millis, 1250);
        assert_eq!(result.clone().unwrap().unique_timestamps, 2);

        result = event_aggregator.get_aggregation_result("game1", "event2");
        assert_eq!(result.is_some(), true);
        assert_eq!(result.clone().unwrap().name, "game1");
        assert_eq!(result.clone().unwrap().event_name, "event2");
        assert_eq!(result.clone().unwrap().total_events, 4);
        assert_eq!(result.clone().unwrap().earlist_timestamp_millis, 600);
        assert_eq!(result.clone().unwrap().latest_timestamp_millis, 1250);
        assert_eq!(result.clone().unwrap().unique_timestamps, 3);

        result = event_aggregator.get_aggregation_result("game2", "event2");
        assert_eq!(result.is_some(), true);
        assert_eq!(result.clone().unwrap().name, "game2");
        assert_eq!(result.clone().unwrap().event_name, "event2");
        assert_eq!(result.clone().unwrap().total_events, 1);
        assert_eq!(result.clone().unwrap().earlist_timestamp_millis, 1200);
        assert_eq!(result.clone().unwrap().latest_timestamp_millis, 1200);
        assert_eq!(result.clone().unwrap().unique_timestamps, 1);

    }

    #[test]
    fn small_hll_test(){
        // Arrange
        let mut event_aggregator = InMemoryEventAggregator::new(true, true);

        // Act
        // Generate 10 million events
        // Create a random timestamp between 500 and 3000
        for i in 0..1000 {
            let random_timestamp = rand::random::<u64>() % 2500 + 500;
            event_aggregator.collect_event(Event { name: "game1".to_string(), event_name: "event1".to_string(), timestamp_millis: random_timestamp, });
        }

        // assert
        let result = event_aggregator.get_aggregation_result("game1", "event1");
        println!("Result: {:?}", result);
        assert_eq!(result.is_some(), true);

    }

    #[test]
    fn massive_event_test(){
        // Arrange
        let mut event_aggregator = InMemoryEventAggregator::new(true, false);

        // Act
        // Generate 10 million events
        // Create a random timestamp between 500 and 3000
        for i in 0..1_000_000 {
            let random_timestamp = rand::random::<u64>() % 2500 + 500;
            event_aggregator.collect_event(Event { name: "game1".to_string(), event_name: "event1".to_string(), timestamp_millis: random_timestamp, });
        }

        // assert
        let result = event_aggregator.get_aggregation_result("game1", "event1");
        println!("Result: {:?}", result);
        assert_eq!(result.is_some(), true);
    }
}
