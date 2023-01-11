# Substreams Ethereum Block Meta

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A Substreams extracting Ethereum block metadata per day (start/end) and per month (start/end). Using this Substreams, you will essentials be able to answer these questions:

- What is the block at the start of day June 30th, 2022?
- What is the block at the end of July 2022?

## Requirements

Follow [Installation Requirements](https://substreams.streamingfast.io/developer-guide/installation-requirements#local-installation) instructions on official Substreams documentation website.

## Building the spkg

* `make package` will compile the substreams rust code and package it into an spkg file

## Running the `graph_out` module into a subgraph

The `graph_out` module output element in format expected for ingestion into a Subgraph. It maps our store output to entities as defined by `schema.graphql`.

This repository contains both the substreams code and the subgraph definition.


```
# Compile & run module `store_block_meta_start` and `store_block_meta_end`
make stream

# Compile & run module `graph_out`
make stream_graph

# Compile, package & deploy to local 'graph-node' instance`
make deploy_local
```

> You need an unreleased version of [graph-cli](https://github.com/graphprotocol/graph-cli), easiest way is to clone the repository, perform `yarn install` in it and then add it to you `PATH` environment variable.

## Running other output modules

* `db_out` will output the blockmeta data in tables with columns, to be saved in a database. See some integrations in https://github.com/streamingfast/substreams-sink-postgres and https://github.com/streamingfast/substreams-sink-mongodb
* `kv_out` will output the blockmeta data in a format to be saved in a key/value store.  See its integration in https://github.com/streamingfast/substreams-sink-kv (note: this module outputs one entry per blockHash instead of per day/month).
