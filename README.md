# rust_or_bust_game

## Setup

### Install Rust

```
sh <(curl --fail --silent --show-error --location https://sh.rustup.rs) --no-modify-path -y
. ~/.cargo/env
cargo install rustfmt
rustup install nightly
rustup default nightly
```

### Checkout

```
git clone git@github.com:trainman419/rust_or_bust_game.git
```

### Dependencies

```
mkdir -p ~/.cargo ~/forks
git clone git@github.com:chpatton013/piston.git ~/forks/piston
(cd ~/forks/piston && cargo build)
echo 'paths = ["forks/piston"]' > ~/.cargo/config
```

### Build

```
cd rust_or_bust_game
cargo build
```

### Play

```
cd rust_or_bust_game
cargo run
```
