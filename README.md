# Jito Shredstream Proxy

ShredStream provides the lowest latency to shreds from leaders on Solana. 

See more at https://docs.jito.wtf/lowlatencytxnfeed/


# Command

Enable grpc service
```
RUST_LOG=info ./target/debug/jito-shredstream-proxy forward-only --src-bind-addr=0.0.0.0 --src-bind-port=20020 --dest-ip-ports 127.0.0.1:20040 --grpc-service-port 20041

./target/debug/examples/deshred --src-bind-addr=0.0.0.0 --src-bind-port=20020 --dest-ip-ports 127.0.0.1:20040 --grpc-service-port 20041 
```

# Rust config
`export RUST_LOG=info` allows rust to print info log to console
