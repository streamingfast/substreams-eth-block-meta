# Substreams Ethereum Block Meta

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A Substreams extracting Ethereum block metadata per day (start/end) and per month (start/end). Using this Substreams, you will essentials be able to answer these questions:

- What is the block at the start of day June 30th, 2022?
- What is the block at the end of July 2022?

## Requirements

Follow [Installation Requirements](https://substreams.streamingfast.io/developer-guide/installation-requirements#local-installation) instructions on official Substreams documentation website.

## Building the spkg

* `make package` will compile the substreams rust code and package it into an spkg file

## Running the `graph_out` module into a Subgraph

The `graph_out` module output element in format expected for ingestion into a Subgraph. It maps our store output to entities as defined by `schema.graphql`.

This repository contains both the Substreams code and the Subgraph definition.

```
# Compile & run module `store_block_meta_start` and `store_block_meta_end`
make stream

# Compile & run module `graph_out`
make stream_graph

# Compile, package & deploy to local 'graph-node' instance`, available configuration:
# - IPFS_ENDPOINT (defaults to 'http://localhost:5001')
# - GRAPH_NODE_ENDPOINT (defaults to 'http://localhost:8020')
make deploy_graph_node
```

> You need a recent enough version of [graph-cli](https://github.com/graphprotocol/graph-tooling)

## Running the `db_out` module

The `db_out` module output element in format expected for ingestion into [substreams-sink-postgres](https://github.com/streamingfast/substreams-sink-postgres) or [substreams-sink-mongodb](https://github.com/streamingfast/substreams-sink-mongodb).

### Into PostgresSQL (via `substreams-sink-postgres`)

This repository contains also the `schema.sql` used to bootstrap the database. Here the instructions to launch `substreams-sink-postgres`:

```
# Install binary from source
git clone https://github.com/streamingfast/substreams-sink-postgres.git
cd substreams-sink-postgres

go install ./cmd/substreams-sink-postgres
export PATH="`go env GOPATH`/bin:$PATH"

# Don't forget to update `PATH` if you spin up another terminal (or make the change persistent)
```

Then in another terminal spin up Docker Compose:

```
# Change where you cloned the repository
cd substreams-sink-postgres

docker compose up
```

And finally within this project run:

```
# Compiles packages this Substreams and then launch `substreams-sink-postgres`
# - POSTGRESQL_DSN (defaults to 'psql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node?sslmode=disable')
make sink_postgres
```

> **Warning** This is meant for show case purposes, running a sink in production should **always** be run on a persistently stored compiled Substreams package (extension `.spkg`).

## Running other output modules

* `kv_out` will output the block meta data in a format to be saved in a key/value store. See its integration in https://github.com/streamingfast/substreams-sink-kv (note: this module outputs one entry per blockHash instead of per day/month).
