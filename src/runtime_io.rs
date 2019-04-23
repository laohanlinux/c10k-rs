use futures::prelude::*;
use runtime::net::tcp::{TcpStream, TcpListener};
use runtime::net::tcp::Incoming;
use futures::future::ok;
use async_timer::{Delay, Timed};
use std::net::{UdpSocket, SocketAddr};
use futures::channel::mpsc::channel;
use tide::{error::ResultExt, response, App, Context, EndpointResult};

use crate::bucket::Buckets;
use crate::proto::Proto;

use std::os::unix::io::AsRawFd;
use std::sync::Arc;

pub async fn runtime_io() -> std::io::Result<()> {
    let mut socket: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let mut listener: TcpListener = TcpListener::bind(&socket)?;
    let mut buckets = Arc::new(Buckets::new(1024));
    let mut incoming = listener.incoming();
    let buckets_c = buckets.clone();

    while let Some(stream) = await!(incoming.next()) {
        let buckets_c = buckets.clone();
        let (tx_ch, rx_ch) = channel::<Proto>(10);
        runtime::spawn(async move {
            let stream: TcpStream = stream?;
            let peer = stream.peer_addr()?;
            let resu64 = farmhash::hash64(peer.to_string().as_bytes());
            buckets_c.insert_session(resu64, tx_ch);
            println!("accept from {}", peer.to_string());
            let (rd, wt) = &mut stream.split();
            await!(rd.copy_into(wt))?;
            Ok::<(), ::std::io::Error>(())
        });

        let buckets_c = buckets.clone();
        runtime::spawn(async move {
            match await!(rx_ch.into_future()) {
                (Some(proto), _receiver) => {
                    if proto.header.operation == 0 {
                        buckets_c.remove(proto.sender);
                    }
                }
                _ => unreachable!()
            }
        });
    }
    Ok(())
}


pub async fn timer(secs: u64) {
    let fut = Delay::platform_new(::std::time::Duration::from_secs(secs));
    await!(fut);
    let mut result = 0;
    for sink in (0..10) {
        result += sink;
    }
    println!("result {}", result);
}

pub async fn run_api(addr: SocketAddr, buckets: Arc<Buckets>) {
    let mut app = tide::App::new(buckets);
    app.at("/msg/push").post(api_push);
    app.at("/msg/pushs").post(api_pushs);
    app.at("/msg/uid").get(api_get_uid);
    app.serve(addr).unwrap();
}

async fn api_push(mut cx: Context<Arc<Buckets>>) -> String {
    let result: EndpointResult<Proto> = await!(cx.body_json()).client_err();
    let buckets = cx.app_data();
    match result {
        Ok(proto) => {
            let uid = proto.receiver;
            buckets.push(uid, proto);
            Ret::ok()
        }
        Err(e) => {
            let err = format!("{:?}", e.into_response());
            println!("args is invalid");
            Ret::new(1, err.as_bytes().to_vec()).to_json()
        }
    }
}

async fn api_pushs(mut cx: Context<Arc<Buckets>>) -> String {
    let result: EndpointResult<Proto> = await!(cx.body_json()).client_err();
    let buckets = cx.app_data();
    match result {
        Ok(proto) => {
            buckets.broadcast(proto);
            Ret::ok()
        }
        Err(e) => {
            let err = format!("{:?}", e.into_response());
            println!("args is invalid");
            Ret::new(1, err.as_bytes().to_vec()).to_json()
        }
    }
}

async fn api_get_uid(mut cx: Context<Arc<Buckets>>) -> String {
    serde_json::from_str("{}").unwrap()
}


use serde_derive::{Serialize, Deserialize};
use tide::response::IntoResponse;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Ret {
    #[serde(rename = "err_code")]
    err_code: usize,
    #[serde(rename = "response")]
    resp: Vec<u8>,
}

impl Ret {
    pub fn new(err_code: usize, resp: Vec<u8>) -> Self {
        Ret {
            err_code: err_code,
            resp: resp,
        }
    }

    pub fn ok() -> String {
        let ret = Ret::new(1, vec![]);
        ret.to_json()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}