use crate::{block_timestamp::BlockTimestamp, pb::eth::block_meta::v1::BlockMeta};
use substreams::{
    store::{DeltaProto, Deltas},
    Hex,
};
use substreams_database_change::tables::Tables;

pub fn add_block_meta_to_tables(tables: &mut Tables, deltas: Deltas<DeltaProto<BlockMeta>>) {
    use substreams::pb::substreams::store_delta::Operation;

    for delta in deltas.into_iter() {
        match delta.operation {
            Operation::Create => push_create(
                tables,
                &delta.key,
                BlockTimestamp::from_key(&delta.key),
                delta.new_value,
            ),
            Operation::Update => push_update(tables, &delta.key, delta.new_value),
            Operation::Delete => panic!("delete should not happen"),
            x => panic!("unsupported opeation {:?}", x),
        }
    }
}

fn push_create(tables: &mut Tables, key: &str, timestamp: BlockTimestamp, value: BlockMeta) {
    tables
        .create_row("block_meta", key)
        .set("at", timestamp)
        .set("number", value.number)
        .set("hash", Hex(value.hash))
        .set("parent_hash", Hex(value.parent_hash))
        .set("timestamp", value.timestamp.unwrap());
}

fn push_update(tables: &mut Tables, key: &str, value: BlockMeta) {
    tables
        .update_row("block_meta", key)
        .set("number", value.number)
        .set("hash", Hex(value.hash))
        .set("parent_hash", Hex(value.parent_hash))
        .set("timestamp", value.timestamp.unwrap());
}
