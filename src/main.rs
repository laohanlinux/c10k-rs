use tokio::prelude::*;
use tokio::net::TcpListener;
use futures::{future, Future};

fn main() {
    let addr = "0.0.0.0:3941".parse().unwrap();
    let lis = TcpListener::bind(&addr).unwrap();
    println!("listen: {}", addr.to_string());

    let incomming = lis.incoming();

    let server = incomming.map_err(|e| eprintln!("accept failed = {:?}", e))
        .for_each(|socket|{
            let (rd, wt) = socket.split();
            let bytes_copied = tokio::io::copy(rd, wt);
            let callback = bytes_copied.map(|(_,_, _) | {
            }).map_err(|e| eprintln!("IO error {:?}", e));

            tokio::spawn(callback);
            future::ok(())
        });

    tokio_io_pool::run(server);
}
