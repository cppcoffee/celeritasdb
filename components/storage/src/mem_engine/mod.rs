use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Mutex;

mod memdb;
pub use memdb::*;

pub type MemBT = HashMap<&'static str, BTreeMap<Vec<u8>, Vec<u8>>>;

/// MemEngine is a in-memory storage for testing or non-persistent environment.
///
/// A storage deals with concurrency itself.
/// Replicas keeps a `Arc` reference to storage engine.
///
/// ```text
/// Replica-1 → Engine
///           ↗
/// Replica-2
/// ```
pub struct MemEngine {
    pub _db: Mutex<MemBT>,
}
