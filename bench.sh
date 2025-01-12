#!/usr/bin/env bash

set -e

cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

# build cargo workspace in release mode
cargo build --release

# if go is installed, then build the go bench
if type "go" > /dev/null; then
	(cd go && go build main.go)
fi

# if there is already a dummy server running kill it
killall bench_dummy_server 2> /dev/null || true

./target/release/bench_dummy_server &

server_pid=$!

# wait for server to start up
sleep 1

./target/release/reqwest_current_thread
./target/release/reqwest_multi_thread

# if go was built then run the go bench
if [ -f "go/main" ]; then
	./go/main
fi

kill $server_pid
