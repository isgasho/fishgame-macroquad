"Fish Game" for Macroquad
=====================

![fishgame](https://user-images.githubusercontent.com/910977/106779317-7dbb4e00-660c-11eb-84e6-83c9f5e73758.gif)


**"Fish Game" for Macroquad** is an online multiplayer game, created as a
demostration of [Nakama](https://heroiclabs.com/), an open-source scalable game
server, using [Rust programming language](https://www.rust-lang.org/) and
the [Macroquad](https://github.com/not-fl3/macroquad/) game engine.

Playing the game online
----------------------------

The latest web build for online play is available [here](https://fedorgames.itch.io/fish-game?secret=UAVcggHn332a).

Playing the game from source
----------------------------

Depedencies:

The main depdency: the rust compiler.   
To get it, follow [rustup.rs](https://rustup.rs/) instructions.

On web, windows and mac os no other external depdendecies are required.
On linux followed libs may be required: 
```
apt install libx11-dev libxi-dev libgl1-mesa-dev
```

### Running the game:

### Native PC build: 

*note that nakama networking is not yet supported on PC and PC build is intenteded only for single player dev builds*

```
cargo run --release
```
from this repo root.

### Building HTML5 build:

```
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/fishgame.wasm web/fishgame.wasm
wasm-strip web/fishgame.wasm
```

To serve the web build some web server will be required. One of the options: [devserver](https://github.com/kettle11/devserver) 

```
cargo install devserver
cd web
devserver .
```

And than open `http://localhost:8080`
