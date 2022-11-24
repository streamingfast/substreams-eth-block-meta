use substreams::store::{self, DeltaProto};
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChanges};

use crate::{block_timestamp::BlockTimestamp, pb::block_meta::BlockMeta};

pub fn block_meta_to_entities_changes(
    changes: &mut EntityChanges,
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
    changes: &mut EntityChanges,
    key: &str,
    timestamp: BlockTimestamp,
    ordinal: u64,
    value: BlockMeta,
) {
    changes
        .push_change("BlockMeta", key, ordinal, Operation::Create)
        .change::<&str, String>("at", timestamp.into())
        .change("number", value.number)
        .change("hash", value.hash)
        .change("parent_hash", value.parent_hash)
        .change("timestamp", value.timestamp.unwrap());
}

fn push_update(
    changes: &mut EntityChanges,
    key: &str,
    ordinal: u64,
    old_value: BlockMeta,
    new_value: BlockMeta,
) {
    changes
        .push_change("BlockMeta", key, ordinal, Operation::Update)
        .change("number", (old_value.number, new_value.number))
        .change("hash", (old_value.hash, new_value.hash))
        .change(
            "parent_hash",
            (old_value.parent_hash, new_value.parent_hash),
        )
        .change(
            "timestamp",
            (&old_value.timestamp.unwrap(), &new_value.timestamp.unwrap()),
        );
}
