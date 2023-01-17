ENDPOINT ?= mainnet.eth.streamingfast.io:443
GRAPH_CONFIG ?= ../graph-node-dev/config/graphman.toml
STOP_BLOCK ?= +10

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream_db
stream_db: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out -t $(STOP_BLOCK)

.PHONY: stream_graph
stream_graph: build
	substreams run -e $(ENDPOINT) substreams.yaml graph_out -t $(STOP_BLOCK)

.PHONY: stream_kv
stream_kv: build
	substreams run -e $(ENDPOINT) substreams.yaml kv_out -t $(STOP_BLOCK)

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONE: package
package: build
	substreams pack -o substreams.spkg substreams.yaml

.PHONE: deploy_local
deploy_local: package
	graph build --ipfs http://localhost:5001 subgraph.yaml
	graph create block_meta --node http://127.0.0.1:8020
	graph deploy --node http://127.0.0.1:8020 --ipfs http://127.0.0.1:5001 --version-label v0.0.1 block_meta subgraph.yaml

.PHONE: undeploy_local
undeploy_local:
	graphman --config "$(GRAPH_CONFIG)" drop --force block_meta
