mod block_timestamp;
mod pb;

use block_timestamp::BlockTimestamp;
use pb::block_meta::BlockMeta;
use substreams::prelude::*;
use substreams::store::{StoreSetIfNotExistsProto, StoreSetProto};
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
