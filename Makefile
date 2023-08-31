ENDPOINT ?= mainnet.eth.streamingfast.io:443
STOP_BLOCK ?= +10

# Deployement to `substreams-sink-postgres` config
IPFS_ENDPOINT ?= http://127.0.0.1:5001
GRAPH_NODE_ENDPOINT ?= http://127.0.0.1:8020
GRAPHMAN_CONFIG ?= ../graph-node-dev/config/graphman.toml

# Deployement to `graph-node` config (defaults is for a local deployment)
POSTGRESQL_DSN ?= psql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node?sslmode=disable

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: db_out
db_out: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out -t $(STOP_BLOCK) --debug-modules-output=store_block_meta_start,db_out

.PHONY: graph_out
graph_out: build
	substreams run -e $(ENDPOINT) substreams.yaml graph_out -t $(STOP_BLOCK)

.PHONY: kv_out
kv_out: build
	substreams run -e $(ENDPOINT) substreams.yaml kv_out -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="google,sf/substreams,substreams/sink/kv,database.proto"

.PHONE: pack
pack: build
	substreams pack -o substreams.spkg substreams.yaml

.PHONE: deploy_graph_node
deploy_graph_node: pack
	graph build --ipfs $(IPFS_ENDPOINT) subgraph.yaml
	graph create block_meta --node $(GRAPH_NODE_ENDPOINT)
	graph deploy --node $(GRAPH_NODE_ENDPOINT) --ipfs $(IPFS_ENDPOINT) --version-label v0.0.1 block_meta subgraph.yaml

.PHONE: undeploy_graph_node
undeploy_graph_node:
	graphman --config "$(GRAPHMAN_CONFIG)" drop --force block_meta

.PHONE: sink_postgres
sink_postgres: pack
	substreams-sink-postgres setup --ignore-duplicate-table-errors "$(POSTGRESQL_DSN)" schema.sql
	substreams-sink-postgres run $(POSTGRESQL_DSN) $(ENDPOINT) "substreams.spkg" db_out
