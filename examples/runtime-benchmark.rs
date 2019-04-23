#![feature(async_await, await_macro, futures_api)]

use c10k_rs::runtime_io::runtime_io;
use c10k_rs::runtime_io::timer;
use c10k_rs::bucket::Buckets;
use futures::future::ok;
use std::thread;
use rand::Rng;
use runtime::net::tcp::{TcpListener, Incoming};
use std::net::SocketAddr;
use futures::prelude::*;

#[runtime::main]
async fn main() {

    runtime::spawn(async move {
        println!("hello Word");
    });

    await!(runtime_io());
}
