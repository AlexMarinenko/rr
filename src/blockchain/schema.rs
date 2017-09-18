use exonum::crypto::Hash;
use exonum::storage::{ProofMapIndex, Snapshot, Fork};
use exonum::blockchain::gen_prefix;

use TIMESTAMPING_SERVICE;
use blockchain::dto::TimestampEntry;

pub const INITIAL_TIMESTAMPS: i64 = 10;

#[derive(Debug)]
pub struct Schema<T> {
    view: T,
}

/// Timestamping information schema.
impl<T> Schema<T> {
    pub fn new(snapshot: T) -> Schema<T> {
        Schema { view: snapshot }
    }

    pub fn into_view(self) -> T {
        self.view
    }
}

impl<T> Schema<T>
where
    T: AsRef<Snapshot>,
{
    pub fn timestamps(&self) -> ProofMapIndex<&T, Hash, TimestampEntry> {
        let prefix = gen_prefix(TIMESTAMPING_SERVICE, 1, &());
        ProofMapIndex::new(prefix, &self.view)
    }

    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.timestamps().root_hash()]
    }
}

impl<'a> Schema<&'a mut Fork> {
    
    pub fn timestamps_mut(&mut self) -> ProofMapIndex<&mut Fork, Hash, TimestampEntry> {
        let prefix = gen_prefix(TIMESTAMPING_SERVICE, 1, &());
        ProofMapIndex::new(prefix, &mut self.view)
    }

    pub fn add_timestamp(&mut self, timestamp_entry: TimestampEntry) {
        
        let timestamp = timestamp_entry.timestamp();
        let content_hash = *timestamp.content_hash();
        // Check that timestamp with given content_hash does not exist.
        if self.timestamps().contains(&content_hash) {
            return;
        }
        
        // Add timestamp
        self.timestamps_mut().put(&content_hash, timestamp_entry);        
    }
}