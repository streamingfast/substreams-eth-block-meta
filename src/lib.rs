use substreams::errors::Error;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn db_out(blk: Block) -> Result<DatabaseChanges, Error> {
    let mut tables = Tables::new();
    add_block_entity(&mut tables, blk);

    Ok(tables.to_database_changes())
}

fn add_block_entity(tables: &mut Tables, blk: Block) {
    let block_hash = Hex(&blk.hash).to_string();

    tables
        .create_row("block_meta", &block_hash)
        .set("number", blk.number)
        .set("hash", &block_hash)
        .set(
            "parent_hash",
            Hex(&blk.header.as_ref().unwrap().parent_hash).to_string(),
        )
        .set(
            "timestamp",
            blk.header.as_ref().unwrap().timestamp.as_ref().unwrap(),
        );
}
