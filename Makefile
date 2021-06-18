SHELL := /bin/bash

PROTOC ?= $(shell which protoc)
GRPC_RUST_PLUGIN ?= $(shell which grpc_rust_plugin)

PROTOS := protos/schema.proto protos/outputs.proto protos/version.proto
PROTO_URLS := https://raw.githubusercontent.com/falcosecurity/falco/master/userspace/falco/schema.proto https://raw.githubusercontent.com/falcosecurity/falco/master/userspace/falco/outputs.proto https://raw.githubusercontent.com/falcosecurity/falco/master/userspace/falco/version.proto
PROTO_SHAS := 1adf7fbb2b92793a3cf490204314af7788ffd81655c4cedb40587a22db9c1915 5e3bdc564c4d38f7d70a8fe50e6022a733ed93197edff6b824a24c6a45fed6c3 fc470546c00273bafe20b53ab6b7e0784206b8f6f9a705df92994e89035a5dc4

PROTO_DIRS := $(dir ${PROTOS})
PROTO_DIRS_INCLUDES := $(patsubst %/, -I %, ${PROTO_DIRS})

.PHONY: protos
protos: ${PROTOS}

# $(1): the proto path
# $(2): the proto URL
# $(3): the proto SHA256
define download_rule
$(1):
	@rm -f $(1)
	@mkdir -p ${PROTO_DIRS}
	@curl --silent -Lo $(1) $(2)
	@echo $(3) $(1) | sha256sum -c
	@sed -i '/option go_package/d' $(1)
	@${PROTOC} ${PROTO_DIRS_INCLUDES} --rust_out=src/api --grpc_out=src/api --plugin=protoc-gen-grpc=${GRPC_RUST_PLUGIN} $(1)
endef
$(foreach PROTO,$(PROTOS),\
	$(eval $(call download_rule,$(PROTO),$(firstword $(PROTO_URLS)),$(firstword $(PROTO_SHAS))))\
	$(eval PROTO_URLS := $(wordlist 2,$(words $(PROTO_URLS)),$(PROTO_URLS)))\
	$(eval PROTO_SHAS := $(wordlist 2,$(words $(PROTO_SHAS)),$(PROTO_SHAS)))\
)

.PHONY: clean
clean: ${PROTO_DIRS}
	@rm -rf $^
