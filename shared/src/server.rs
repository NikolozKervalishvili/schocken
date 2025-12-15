use crate::schockerr::Result;
use std::collections::HashSet;
use std::sync::atomic::AtomicU32;
use tokio::net::TcpListener;

pub struct Server {
    listener: TcpListener,
    ids: HashSet<AtomicU32>,
}

impl Server {
    const LOCALHOST: &'static str = "127.0.0.1:8080";

    // inits it with localhost
    async fn localhost() -> Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(Self::LOCALHOST).await?,
            ids: HashSet::new(),
        })
    }
}
