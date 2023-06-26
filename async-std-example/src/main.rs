use async_std::fs::File;
use async_std::prelude::*;

/// our say_hello function actually doesn't need to be async, as we don't wait for anything
/// however we keep it async for the example
async fn say_hello() {
    println!("Hello, world!");
}

#[async_std::main]
async fn main() {
    // wait for the async function to complete
    say_hello().await;

    // second example writing to a file async
    let mut file = File::create("a.txt")
        .await
        .expect("file should be createable");
    file.write_all(b"Hello, world!")
        .await
        .expect("file should be writeable");
    file.flush().await.expect("flushing should be possible");
}
