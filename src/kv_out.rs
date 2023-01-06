use substreams::{
    store::{self, DeltaProto},
    Hex,
};
use substreams_sink_kv::pb::kv::KvOperations;

use crate::pb::block_meta::BlockMeta;

pub fn block_meta_to_kv_ops(ops: &mut KvOperations, deltas: store::Deltas<DeltaProto<BlockMeta>>) {
    use substreams::pb::substreams::store_delta::Operation;

    for delta in deltas.deltas {
        match delta.operation {
            Operation::Create | Operation::Update => {
                ops.push_new(&delta.key, delta.new_value.hash.clone(), delta.ordinal);
                ops.push_new(
                    Hex::encode(delta.new_value.hash),
                    delta.new_value.parent_hash.to_vec(),
                    delta.ordinal,
                );
            }
            Operation::Delete => ops.push_delete(&delta.key, delta.ordinal),
            x => panic!("unsupported opeation {:?}", x),
        }
    }
}
//
//fn push_create(
//    changes: &mut DatabaseChanges,
//    key: &str,
//    timestamp: BlockTimestamp,
//    ordinal: u64,
//    value: BlockMeta,
//) {
//    changes
//        .push_change("block_meta", key, ordinal, Operation::Create)
//        .change("at", (None, timestamp))
//        .change("number", (None, value.number))
//        .change("hash", (None, Hex(value.hash)))
//        .change("parent_hash", (None, Hex(value.parent_hash)))
//        .change("timestamp", (None, value.timestamp.unwrap()));
//}
//
//fn push_update(
//    changes: &mut DatabaseChanges,
//    key: &str,
//    ordinal: u64,
//    old_value: BlockMeta,
//    new_value: BlockMeta,
//) {
//    changes
//        .push_change("block_meta", key, ordinal, Operation::Update)
//        .change("number", (old_value.number, new_value.number))
//        .change("hash", (Hex(old_value.hash), Hex(new_value.hash)))
//        .change(
//            "parent_hash",
//            (Hex(old_value.parent_hash), Hex(new_value.parent_hash)),
//        )
//        .change(
//            "timestamp",
//            (&old_value.timestamp.unwrap(), &new_value.timestamp.unwrap()),
//        );
//}
