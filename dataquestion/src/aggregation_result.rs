#[derive(Clone, Debug)]
pub struct AggregationResult {
    pub name: String,
    pub event_name: String,
    pub earlist_timestamp_millis: u64,
    pub latest_timestamp_millis: u64,
    pub total_events: u64,
    pub unique_timestamps: u64
}

