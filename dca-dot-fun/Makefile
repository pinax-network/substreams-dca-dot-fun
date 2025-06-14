ENDPOINT ?= basesepolia.substreams.pinax.network:443
START_BLOCK ?= 26620234

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack: build
	substreams pack

.PHONY: noop
noop: build
	substreams-sink-noop $(ENDPOINT) substreams.yaml map_events -H "X-Sf-Substreams-Parallel-Jobs: 500" $(START_BLOCK):

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_events -s $(START_BLOCK)
