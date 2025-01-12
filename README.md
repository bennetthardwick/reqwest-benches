# Reqwest runtime benches

These benchmarks download a 16GiB file of bytes set to `1` from a local server and calculate the average download speed.

To run the benchmarks yourself, clone the repo and run:

```sh
./bench.sh
```

You will need a Rust toolchain installed.

There is also a Go benchmark for comparison.

## Results

| bench | speed |
| reqwest_current_thread | 3276 MiB/s |
| reqwest_multi_thread | 680 MiB/s |
| go | 2804 MiB/s |

Running on a multi threaded tokio runtime is considerably slower than the current thread runtime and Go.
