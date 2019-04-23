#![feature(async_await, await_macro, futures_api, existential_type, custom_attribute)]

pub mod bucket;
pub mod runtime_io;
pub mod proto;

use futures::{future, Future};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use std::net::SocketAddr;

fn tokio_mutil_io(addr: SocketAddr) {
    //    let lis = TcpListener::bind(&addr).unwrap();
    //    println!("listen: {}", addr.to_string());
    //
    //    let incomming = lis.incoming();
    //
    //    let server = incomming.map_err(|e| eprintln!("accept failed = {:?}", e))
    //        .for_each(|socket| {
    //            let (rd, wt) = socket.split();
    //            let bytes_copied = tokio::io::copy(rd, wt);
    //            let callback = bytes_copied.map(|(_, _, _)| {}).map_err(|e| eprintln!("IO error {:?}", e));
    //
    //            tokio::spawn(callback);
    //            future::ok(())
    //        });
    //
    //    tokio_io_pool::run(server);
}
