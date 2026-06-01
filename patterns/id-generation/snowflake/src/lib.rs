use chrono::Utc;
use std::sync::Mutex;

// ┌─────────────────────────────────────────────────────┐
// │  63    62           22    17         7           0  │
// │   0 │  41-bit ms  │ 5-bit │ 5-bit seq│  12-bit seq │
// │sign  │ timestamp  │datactr│  machine │             │
// └─────────────────────────────────────────────────────┘
//
// Layout (Twitter Snowflake compatible):
//   bit 63     : always 0 (sign bit → positive i64)
//   bits 62-22 : 41-bit millisecond timestamp (relative to EPOCH)
//   bits 21-17 : 5-bit datacenter id  (0-31)
//   bits 16-12 : 5-bit machine id     (0-31)
//   bits 11-0  : 12-bit sequence      (0-4095 per ms per machine)

/// Custom epoch: 2020-01-01T00:00:00Z (ms since Unix epoch).
pub const EPOCH_MS: u64 = 1_577_836_800_000;

const DATACENTER_BITS: u8 = 5;
const MACHINE_BITS: u8 = 5;
const SEQUENCE_BITS: u8 = 12;

pub const MAX_DATACENTER_ID: u64 = (1 << DATACENTER_BITS) - 1; // 31
pub const MAX_MACHINE_ID: u64 = (1 << MACHINE_BITS) - 1; // 31
const MAX_SEQUENCE: u64 = (1 << SEQUENCE_BITS) - 1; // 4095

const MACHINE_SHIFT: u8 = SEQUENCE_BITS;
const DATACENTER_SHIFT: u8 = SEQUENCE_BITS + MACHINE_BITS;
const TIMESTAMP_SHIFT: u8 = SEQUENCE_BITS + MACHINE_BITS + DATACENTER_BITS;

/// Thread-safe Snowflake ID generator.
///
/// Produces 64-bit, time-ordered, globally unique IDs without coordination.
/// Up to 4 096 IDs per millisecond per (datacenter, machine) pair.
pub struct SnowflakeGenerator {
    datacenter_id: u64,
    machine_id: u64,
    inner: Mutex<Inner>,
}

struct Inner {
    last_timestamp_ms: u64,
    sequence: u64,
}

impl SnowflakeGenerator {
    /// Create a new generator.
    ///
    /// # Panics
    /// Panics if `datacenter_id > 31` or `machine_id > 31`.
    pub fn new(datacenter_id: u64, machine_id: u64) -> Self {
        assert!(
            datacenter_id <= MAX_DATACENTER_ID,
            "datacenter_id must be 0-{MAX_DATACENTER_ID}"
        );
        assert!(
            machine_id <= MAX_MACHINE_ID,
            "machine_id must be 0-{MAX_MACHINE_ID}"
        );
        SnowflakeGenerator {
            datacenter_id,
            machine_id,
            inner: Mutex::new(Inner {
                last_timestamp_ms: 0,
                sequence: 0,
            }),
        }
    }

    fn now_ms() -> u64 {
        Utc::now().timestamp_millis() as u64
    }

    /// Generate the next unique ID.
    ///
    /// Blocks (spin-waits) for up to 1 ms if the sequence overflows within
    /// the current millisecond.
    pub fn next_id(&self) -> u64 {
        let mut inner = self.inner.lock().expect("mutex poisoned");
        let mut ts = Self::now_ms();

        if ts == inner.last_timestamp_ms {
            inner.sequence = (inner.sequence + 1) & MAX_SEQUENCE;
            if inner.sequence == 0 {
                // Sequence overflow — spin until the next millisecond.
                while ts <= inner.last_timestamp_ms {
                    ts = Self::now_ms();
                }
            }
        } else {
            inner.sequence = 0;
        }

        inner.last_timestamp_ms = ts;

        ((ts - EPOCH_MS) << TIMESTAMP_SHIFT)
            | (self.datacenter_id << DATACENTER_SHIFT)
            | (self.machine_id << MACHINE_SHIFT)
            | inner.sequence
    }

    /// Decompose a Snowflake ID into its constituent fields.
    pub fn parse(id: u64) -> ParsedId {
        ParsedId {
            timestamp_ms: EPOCH_MS + (id >> TIMESTAMP_SHIFT),
            datacenter_id: (id >> DATACENTER_SHIFT) & MAX_DATACENTER_ID,
            machine_id: (id >> MACHINE_SHIFT) & MAX_MACHINE_ID,
            sequence: id & MAX_SEQUENCE,
        }
    }
}

/// Decomposed fields of a Snowflake ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedId {
    /// Wall-clock millisecond when the ID was issued (absolute Unix ms).
    pub timestamp_ms: u64,
    pub datacenter_id: u64,
    pub machine_id: u64,
    pub sequence: u64,
}
