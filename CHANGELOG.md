# Changelog

## v0.4.1

* Updated `kv_out` output type from `proto:sf.substreams.kv.v1.KVOperations` to `proto:sf.substreams.sink.kv.v1.KVOperations`

## v0.4.0

* Changed key format to human-readable syntax:
  * `day:first:20150205`
  * `day:last:20150205`
  * `month:first:201506`
  * `month:last:201604`

## v0.3.0

* Changed all protobuf namespaces from 'substreams....' to 'sf.substreams...', ex: sf.substreams.database.v1.DatabaseChanges
