use std::time::Instant;

#[derive(Clone)]
pub(crate) struct Snapshot {
    pub block_number: u64,
    pub block_time_offset_seconds: i64,
    pub next_block_base_fee_per_gas: Option<u128>,
    pub next_block_timestamp: Option<u64>,
    pub time: Instant,
}
