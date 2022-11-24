# Substreams Ethereum Block Meta

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A Substreams extracting block metadata per day (start/end) and per month (start/end). Using this Substreams, you will essentials be able to answer these questions:

- What is the block at the start of day June 30th, 2022?
- What is the block at the end of July 2022?

## Running

Follow [Installation Requirements](https://substreams.streamingfast.io/developer-guide/installation-requirements#local-installation) instructions on official Substreams documentation website.

```
# Compile & run module `store_block_meta_start` and `store_block_meta_end`
make stream

# Compile & run module `graph_out`
make stream_graph

# Compile, package & deploy to local 'graph-node' instance`
make deploy_local
```

> You need an unreleased version of [graph-cli](https://github.com/graphprotocol/graph-cli), easiest way is to clone the repository, perform `yarn install` in it and then add it to you `PATH` environment variable.

### Modules

The `graph_out` module output element in format expected for ingestion into a Subgraph. It maps our store output to entities as defined by `schema.graphql`.
