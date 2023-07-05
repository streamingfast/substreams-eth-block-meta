mod block_timestamp;
#[path = "db_out.rs"]
mod db;
#[path = "graph_out.rs"]
mod graph;
#[path = "kv_out.rs"]
mod kv;
mod pb;

use block_timestamp::BlockTimestamp;
use pb::eth::block_meta::v1::BlockMeta;
use substreams::errors::Error;
use substreams::store::{
    DeltaProto, Deltas, StoreNew, StoreSet, StoreSetIfNotExists, StoreSetIfNotExistsProto,
    StoreSetProto,
};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

#[substreams::handlers::store]
fn store_block_meta_start(blk: Block, s: StoreSetIfNotExistsProto<BlockMeta>) {
    let (timestamp, meta) = block_to_block_meta(blk);

    s.set_if_not_exists(meta.number, timestamp.start_of_day_key(), &meta);
    s.set_if_not_exists(meta.number, timestamp.start_of_month_key(), &meta);
}

#[substreams::handlers::store]
fn store_block_meta_end(blk: Block, s: StoreSetProto<BlockMeta>) {
    let (timestamp, meta) = block_to_block_meta(blk);

    s.set(meta.number, timestamp.end_of_day_key(), &meta);
    s.set(meta.number, timestamp.end_of_month_key(), &meta);
}

#[substreams::handlers::map]
pub fn db_out(
    block_meta_start: Deltas<DeltaProto<BlockMeta>>,
    block_meta_end: Deltas<DeltaProto<BlockMeta>>,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();
    db::add_block_meta_to_tables(&mut tables, block_meta_start);
    db::add_block_meta_to_tables(&mut tables, block_meta_end);

    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
pub fn graph_out(
    block_meta_start: Deltas<DeltaProto<BlockMeta>>,
    block_meta_end: Deltas<DeltaProto<BlockMeta>>,
) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();
    graph::add_block_meta_to_tables(&mut tables, block_meta_start);
    graph::add_block_meta_to_tables(&mut tables, block_meta_end);

    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn kv_out(
    block_meta_start: Deltas<DeltaProto<BlockMeta>>,
    block_meta_end: Deltas<DeltaProto<BlockMeta>>,
) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();
    kv::block_meta_to_kv_ops(&mut kv_ops, block_meta_start);
    kv::block_meta_to_kv_ops(&mut kv_ops, block_meta_end);

    Ok(kv_ops)
}

fn block_to_block_meta(blk: Block) -> (BlockTimestamp, BlockMeta) {
    let timestamp = BlockTimestamp::from_block(&blk);
    let header = blk.header.unwrap();

    (
        timestamp,
        BlockMeta {
            number: blk.number,
            hash: blk.hash,
            parent_hash: header.parent_hash,
            timestamp: Some(header.timestamp.unwrap()),
        },
    )
}
