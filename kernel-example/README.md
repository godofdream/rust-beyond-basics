
# Rust im Linux Kernel
* see also https://www.kernel.org/doc/html/next/rust/quick-start.html

RustDoc für das kernel crate: https://rust-for-linux.github.io/docs/kernel/


Wir entwickeln ein hello world modul in Rust für den Kernel
- basierend auf https://www.jackos.io/rust-kernel/rust-for-linux.html



1. wir starten einen Docker Container mit der Entwicklungsumgebung
```
docker run -it jackosio/rust-linux:latest
```
2. Als Nächstes Starten wir Vscode in diesem Container
  1. Dafür klicken wir auf `open a remote window` (links unten)
  2. Attach to running container
  3. wir wählen den rust-linux container
3. Wir aktivieren die rust-analyzer extension für den container
4. Open Folder /linux/
5. Add example to samples/rust/
6. `rustup update`
7. `make menuconfig`
