mod block_timestamp;
#[path = "db_out.rs"]
mod db;
#[path = "graph_out.rs"]
mod graph;
mod pb;
mod schema;

use block_timestamp::BlockTimestamp;
use pb::block_meta::BlockMeta;
use substreams::errors::Error;
use substreams::store::{DeltaProto, StoreSetIfNotExistsProto, StoreSetProto};
use substreams::{prelude::*, store};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::pb::eth::v2 as eth;

#[substreams::handlers::store]
fn store_block_meta_start(blk: eth::Block, s: StoreSetIfNotExistsProto<BlockMeta>) {
    let (timestamp, meta) = block_to_block_meta(blk);

    s.set_if_not_exists(meta.number, timestamp.start_of_day_key(), &meta);
    s.set_if_not_exists(meta.number, timestamp.start_of_month_key(), &meta);
}

#[substreams::handlers::store]
fn store_block_meta_end(blk: eth::Block, s: StoreSetProto<BlockMeta>) {
    let (timestamp, meta) = block_to_block_meta(blk);

    s.set(meta.number, timestamp.end_of_day_key(), &meta);
    s.set(meta.number, timestamp.end_of_month_key(), &meta);
}

#[substreams::handlers::map]
pub fn db_out(
    block_meta_start: store::Deltas<DeltaProto<BlockMeta>>,
    block_meta_end: store::Deltas<DeltaProto<BlockMeta>>,
) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();
    db::block_meta_to_database_changes(&mut database_changes, block_meta_start);
    db::block_meta_to_database_changes(&mut database_changes, block_meta_end);

    Ok(database_changes)
}

#[substreams::handlers::map]
pub fn graph_out(
    block_meta_start: store::Deltas<DeltaProto<BlockMeta>>,
    block_meta_end: store::Deltas<DeltaProto<BlockMeta>>,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    graph::block_meta_to_entities_changes(&mut entity_changes, block_meta_start);
    graph::block_meta_to_entities_changes(&mut entity_changes, block_meta_end);

    Ok(entity_changes)
}

fn block_to_block_meta(blk: eth::Block) -> (BlockTimestamp, BlockMeta) {
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
