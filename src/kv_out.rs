use substreams::proto;
use substreams::store::{DeltaProto, Deltas};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

use crate::pb::eth::block_meta::v1::BlockMeta;

pub fn block_meta_to_kv_ops(ops: &mut KvOperations, deltas: Deltas<DeltaProto<BlockMeta>>) {
    use substreams::pb::substreams::store_delta::Operation;

    for delta in deltas.into_iter() {
        match delta.operation {
            Operation::Create | Operation::Update => {
                let val = proto::encode(&delta.new_value).unwrap();
                ops.push_new(delta.key, val, delta.ordinal);
            }
            Operation::Delete => ops.push_delete(&delta.key, delta.ordinal),
            x => panic!("unsupported opeation {:?}", x),
        }
    }
}
