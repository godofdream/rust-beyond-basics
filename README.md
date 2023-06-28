# rust-beyond-basics
Schulungsunterlagen für den Kurs "Rust für Fortgeschrittene"


### Folien:
https://docs.google.com/presentation/d/1zKrAIYu4JKYaCPUAr73H02BAcXxT_MhTmAZyzHK0UyQ/edit?usp=sharing

### Slack beitreten:

https://join.slack.com/t/happy-it-workspace/shared_invite/zt-1y0o1ph42-SHmlMMzrVL5X~4YrKaxKkQ

# Vorbereitungen am Tag 1
```
# installieren/updaten von rust mit rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#für manche Beispiele Brauchen wir nightly features
rustup toolchain install nightly
rustup default nightly
rustup component add rust-src

# installieren der verschiedenen Architekturen für die einzelnen Beispiele
rustup target add thumbv7m-none-eabi
rustup target add wasm32-wasi
rustup target add wasm32-unknown-unknown

# alle example Projekte mit abhängigkeiten holen und bauen
cargo build

# download rust-linux als docker-container (für Tag 3)
docker pull jackosio/rust-linux:latest
```

* installieren von vscode
  * siehe https://code.visualstudio.com/
  * plugin rust-analyzer (The Rust Programming Language)
  * plugin crates (Seray Uzgur)
  * plugin Remote Development (Microsoft; damit können wir in einem Container entwickeln)

