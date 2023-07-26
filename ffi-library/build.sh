cargo build
cp ../target/debug/libffi_library.so ./
gcc -g call_rust.c -o call_rust -lffi_library -L../target/debug