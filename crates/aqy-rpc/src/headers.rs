/// Chain ID of the current chain
pub const X_AQY_CHAIN_ID: &str = "x-aqy-chain-id";

/// Chain name of the current chain
pub const X_AQY_CHAIN: &str = "x-aqy-chain";

/// Current checkpoint height
pub const X_AQY_CHECKPOINT_HEIGHT: &str = "x-aqy-checkpoint-height";

/// Lowest available checkpoint for which transaction and checkpoint data can be requested.
///
/// Specifically this is the lowest checkpoint for which the following data can be requested:
///  - checkpoints
///  - transactions
///  - effects
///  - events
pub const X_AQY_LOWEST_AVAILABLE_CHECKPOINT: &str = "x-aqy-lowest-available-checkpoint";

/// Lowest available checkpoint for which object data can be requested.
///
/// Specifically this is the lowest checkpoint for which input/output object data will be
/// available.
pub const X_AQY_LOWEST_AVAILABLE_CHECKPOINT_OBJECTS: &str =
    "x-aqy-lowest-available-checkpoint-objects";

/// Current epoch of the chain
pub const X_AQY_EPOCH: &str = "x-aqy-epoch";

/// Current timestamp of the chain - represented as number of milliseconds from the Unix epoch
pub const X_AQY_TIMESTAMP_MS: &str = "x-aqy-timestamp-ms";

/// Current timestamp of the chain - encoded in the [RFC 3339] format.
///
/// [RFC 3339]: https://www.ietf.org/rfc/rfc3339.txt
pub const X_AQY_TIMESTAMP: &str = "x-aqy-timestamp";
