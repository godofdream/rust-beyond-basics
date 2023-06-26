# rust-beyond-basics
Schulungsunterlagen für den Kurs "Rust für Fortgeschrittene"


# Vorbereitungen am Tag 1
```
# installieren von rust mit rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

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

