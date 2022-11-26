use substreams::{
    store::{self, DeltaProto},
    Hex,
};
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

use crate::{block_timestamp::BlockTimestamp, pb::block_meta::BlockMeta};

pub fn block_meta_to_database_changes(
    changes: &mut DatabaseChanges,
    deltas: store::Deltas<DeltaProto<BlockMeta>>,
) {
    use substreams::pb::substreams::store_delta::Operation;

    for delta in deltas.deltas {
        match delta.operation {
            Operation::Create => push_create(
                changes,
                &delta.key,
                BlockTimestamp::from_key(&delta.key),
                delta.ordinal,
                delta.new_value,
            ),
            Operation::Update => push_update(
                changes,
                &delta.key,
                delta.ordinal,
                delta.old_value,
                delta.new_value,
            ),
            Operation::Delete => todo!(),
            x => panic!("unsupported opeation {:?}", x),
        }
    }
}

fn push_create(
    changes: &mut DatabaseChanges,
    key: &str,
    timestamp: BlockTimestamp,
    ordinal: u64,
    value: BlockMeta,
) {
    changes
        .push_change("block_meta", key, ordinal, Operation::Create)
        .change("at", (None, timestamp))
        .change("number", (None, value.number))
        .change("hash", (None, Hex(value.hash)))
        .change("parent_hash", (None, Hex(value.parent_hash)))
        .change("timestamp", (None, value.timestamp.unwrap()));
}

fn push_update(
    changes: &mut DatabaseChanges,
    key: &str,
    ordinal: u64,
    old_value: BlockMeta,
    new_value: BlockMeta,
) {
    changes
        .push_change("block_meta", key, ordinal, Operation::Update)
        .change("number", (old_value.number, new_value.number))
        .change("hash", (Hex(old_value.hash), Hex(new_value.hash)))
        .change(
            "parent_hash",
            (Hex(old_value.parent_hash), Hex(new_value.parent_hash)),
        )
        .change(
            "timestamp",
            (&old_value.timestamp.unwrap(), &new_value.timestamp.unwrap()),
        );
}
