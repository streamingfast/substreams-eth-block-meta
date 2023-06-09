# Changelog

## v0.4.3

* Updated `graph_out` to use new `substreams_entity_change::tables:Tables` abstraction (output format stays the same).

## v0.4.2

* Updated `db_out` output type from `proto:sf.substreams.databse.v1.DatabaseChanges` to `proto:sf.substreams.sink.database.v1.DatabaseChanges`.

  > **Note** You need to update to latest `substreams-sink-postgres`, `substreams-sink-mongodb` to use this new package id.

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
