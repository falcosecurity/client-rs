#!/usr/bin/env bash
# Please see the grpc rust plugin installation guide before running this script
# https://github.com/tikv/grpc-rs#option-1---manual-generation

curdir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
proto_dir="$(dirname "$curdir")/protos"
src_dir="$(dirname "$curdir")/src"
protoc --rust_out=${src_dir}/api --grpc_out=${src_dir}/api --plugin=protoc-gen-grpc=`which grpc_rust_plugin` --proto_path ${proto_dir} schema.proto output.proto
