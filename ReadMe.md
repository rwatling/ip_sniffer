# Rust Port Sniffer

This project is adapted from Tensor Programming's Youtube Videos to learn more about Rust. This program searches through the ports available at a particular address using threads. Version 1 uses OS threads. Version 2 uses a library with lighter weight threads that are better suited for the use case here.

The GitHub code is readily available, but I optimized or cleaned up the code after typing it myself to enhance my understanding. I'm not entirely confident that this program avoids race conditions but I am a bit unclear on the synchronization mechanisms with the tokio library. In the future I hope to look at a MPI barrier type of synzhronization if that isn't strictly enforced.

## Useful commands
Build program
`cargo build`

Run program
`cargo run -- -h`
`cargo run -- -s 1 -e 65535 -a 127.0.0.1`

Clean up environment
`cargo install cargo-cache`
`cargo-cache -a`
`cargo-cache -r all`

Tensor Programming's GitHub Repo can be found [here](https://github.com/tensor-programming/Rust_Port_Sniffer/tree/v1)
