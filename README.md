# Merkle Storage
Merkle Storage with [sled](https://github.com/spacejam/sled) as persistent storage
### How to build

````shell script
cargo build
````

### How to Test

````shell script
cargo test
````

## Benchmarking

The benchmark simulates connected clients ``c`` making ``n`` sets commands on merkle storage,
the benchmark show the number of set commands that has been committed to storage 

````shell script
cd benchmark
````

````shell script
cargo run
````