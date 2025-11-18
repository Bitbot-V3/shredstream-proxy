
# Get submodules

git submodule update --init

# Install build dependency

sudo apt install build-essential
sudo apt install clang

# Run binary

RUST_LOG=info ./target/release/jito-shredstream-proxy forward-only --src-bind-addr=0.0.0.0 --src-bind-port=20020 --dest-ip-ports 127.0.0.1:20040 --grpc-service-port 20041

