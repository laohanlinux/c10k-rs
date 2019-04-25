#![feature(async_await, await_macro)]

use c10k_rs::runtime_io::runtime_io;

#[runtime::main]
async fn main() {
    runtime::spawn(async move {
        println!("hello Word");
    });

    let result  = await!(runtime_io());
    if result.is_err() {
        println!("{:?}", result);
    }
}
