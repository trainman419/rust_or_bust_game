# rust_or_bust_game

## Setup

### Install Rust

```
sh <(curl --fail --silent --show-error --location https://sh.rustup.rs) --no-modify-path -y
[ -f ~/.cargo/bin/rustfmt ] || cargo install rustfmt
rustup install nightly
rustup default nightly
```

### Checkout

git clone git@github.com:trainman419/rust_or_bust_game.git

### Build

cd rust_or_bust_game
cargo build

### Play

cd rust_or_bust_game
cargo run
