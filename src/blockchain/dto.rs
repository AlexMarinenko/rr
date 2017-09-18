use exonum::crypto::{PublicKey, Hash};

use TIMESTAMPING_SERVICE;

pub const TX_TIMESTAMP_ID: u16 = 0;

encoding_struct! {
    /// Information about payment.
    struct Timestamp {
        const SIZE = 40;
        /// Hash of content.
        field content_hash:             &Hash       [00 => 32]
        /// Additional metadata.
        field metadata:                 &str        [32 => 40]
    }
}

encoding_struct! {
    /// Timestamp entry
    struct TimestampEntry {
        const SIZE = 40;
        /// Timestamp value.
        field timestamp:                Timestamp   [00 => 08]
        /// Hash of tx.
        field tx_hash:                  &Hash       [08 => 40]
    }
}

message! {
    /// A timestamp transaction.
    struct TxTimestamp {
        const TYPE = TIMESTAMPING_SERVICE;
        const ID = TX_TIMESTAMP_ID;
        const SIZE = 40;

        /// Public key of transaction.
        field pub_key:                  &PublicKey  [00 => 32]
        /// Timestamp content.
        field content:                  Timestamp   [32 => 40]
    }
}