use std::collections::BTreeMap;
use std::sync::{RwLock, Arc};
use std::time::Instant;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};

#[derive(Clone, Debug)]
pub struct Snapshot {
	pub block_number: u64,
	pub next_block_base_fee_per_gas: Option<u128>,
	pub next_block_timestamp: Option<u64>,
	pub time: Instant,
}


pub struct SnapshotManager {
	next_snapshot_id: RwLock<u64>,
	snapshots: RwLock<BTreeMap<u64, Snapshot>>,
}

impl SnapshotManager {
	pub fn new() -> Self {
		Self {
            // Start with 1 to mimic Ganache
			next_snapshot_id: RwLock::new(1),
			snapshots: RwLock::new(BTreeMap::new()),
		}
	}
}

#[rpc(server)]
pub trait SnapshotRpc {
	#[method(name = "evm_snapshot")]
	fn snapshot(&self) -> RpcResult<u64>;
}

impl SnapshotRpcServer for SnapshotManager {
	fn snapshot(&self) -> RpcResult<u64> {
        let mut id = self.next_snapshot_id.write().unwrap();
        let mut snapshots = self.snapshots.write().unwrap();

        let snapshot = Snapshot {
            block_number: 0,
            next_block_base_fee_per_gas: None,
            next_block_timestamp: None,
            time: Instant::now(),
        };

        let current_id = *id;
        snapshots.insert(current_id, snapshot);
        *id += 1;

        Ok(current_id)
    }
}