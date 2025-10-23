#! /usr/bin/env bash

BIN_NAME=$0

source ./scripts/liblog.sh

# print help
phelp () {
    echo "./run.sh [ -r | -d | -rd ]"
    echo "-r  run release server"
    echo "-d  run detached server"
    echo "-rd run detached release server [alias=\"-dr\"]"
}

if [[ $BIN_NAME != "./run.sh" ]]; then
    err "run script from project root dir"
    err "found $BIN_NAME not ./run.sh"
    phelp
    exit 1
fi

case "$1" in
    "")
        info "running server..."
        RUST_LOG=debug cargo run
    ;;
    -d)
        info "running detached server..."
        RUST_LOG=error,abizeitungsumfrage=debug nohup cargo run &
    ;;
    -r)
        info "running release server..."
        RUST_LOG=error,abizeitungsumfrage=debug cargo run --release
    ;;
    -dr | -rd)
        info "running detached release server..."
        RUST_LOG=error,abizeitungsumfrage=debug nohup cargo run --release &
    ;;
    *)
        err "invalid arg: $1"
        phelp
    ;;
esac

