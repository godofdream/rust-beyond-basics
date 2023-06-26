# Performance mit flamegraph

```
cargo install flamegraph
sudo sysctl -w kernel.perf_event_paranoid=1 # allow profiling in kernel
echo 0 | sudo tee /proc/sys/kernel/kptr_restrict #allow more profiling info
```


* Abhängigkeiten Installieren (siehe https://github.com/flamegraph-rs/flamegraph )


```
cargo build --release
flamegraph -o flame.svg -- target/release/flamegraph-example
convert flame.svg flame.png
```


