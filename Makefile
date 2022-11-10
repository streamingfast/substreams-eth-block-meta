.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream:
	substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml store_block_meta_start,store_block_meta_end -t +10

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"
