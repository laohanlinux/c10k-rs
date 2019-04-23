use std::collections::HashMap;
use futures::channel::mpsc::Sender;
use futures::prelude::*;
use runtime::task::spawn;
use parking_lot::RwLock;

use crate::proto::Proto;

pub struct Buckets {
    inner: Vec<HSession>,
}

type HSession = RwLock<HashMap<u64, Session>>;

impl Buckets {
    pub fn new(sz: usize) -> Self {
        let mut buckets = Buckets {
            inner: vec![],
        };

        for idx in (0..sz) {
            buckets.inner.push(HSession::new(HashMap::new()));
        }

        buckets
    }

    pub fn insert_session(&self, uid: u64, ch: Sender<Proto>) {
        let sessions = self.sessions(uid);
        {
            if sessions.read().get(&uid).is_some() {
                return;
            }
        }

        let session = Session::new(ch);
        {
            let sessions = self.sessions(uid);
            sessions.write().insert(uid, session);
        }
    }

    pub fn get_session(&self, uid: u64) -> Option<&Session> {
        let sessions = self.sessions(uid);
        if let session = sessions.read().get(&uid) {
            None
        } else {
            None
        }
    }

    pub fn remove(&self, uid: u64) {
        let sessions = self.sessions(uid);
        sessions.write().remove(&uid);
    }

    pub fn push(&self, uid: u64, proto: Proto) {
        let sessions = self.sessions(uid);
        if let Some(session) = sessions.write().get_mut(&uid) {
            session.push(proto);
        }
    }

    pub fn broadcast(&self, proto: Proto) {
        for inner in &self.inner {
            let mut sessions = inner.write();
            sessions.iter_mut().for_each(|(_, session)| {
                session.push(proto.clone());
            });
        }
    }

    fn sessions(&self, uid: u64) -> &HSession {
        let idx = uid % self.inner.len() as u64;
        self.inner.get(idx as usize).unwrap()
    }
}

pub struct Session {
    ch: Sender<Proto>,
}

impl Session {
    pub fn new(ch: Sender<Proto>) -> Self {
        Session { ch: ch }
    }

    pub fn push(&mut self, proto: Proto) {
        let mut sender = self.ch.clone();
        runtime::spawn(async move {
            await!(sender.send(proto));
        });
    }
}


#[cfg(test)]
mod test {
    use parking_lot::RwLock;
    use std::collections::HashMap;
    use crate::bucket::Buckets;
    use crate::proto::Proto;
    use futures::channel::mpsc::channel;
    use futures::prelude::*;

    #[test]
    fn conrrency() {
        tt()
    }

    #[runtime::test]
    async fn tt() {
        let mut buckets = Buckets::new(1024);
        let mut rx_vec = vec![];
        for i in (0..100) {
            let (tx, rx) = channel(10);
            rx_vec.push(rx);
            buckets.insert_session(i, tx);
        }
        for i in (0..100) {
            buckets.push(i, Proto::new(i))
        }
        for i in (0..10) {
            buckets.broadcast(Proto::new(i));
        }

        rx_vec.into_iter().for_each(|rx| {
            runtime::spawn(async move {
                match await!(rx.into_future()) {
                    (Some(proto), _receiver) => {
                        println!("receive: {:?}", proto);
                    }
                    _ => {}
                }
            });
        });
    }
}
