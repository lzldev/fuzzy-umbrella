pub mod auth;
pub mod data;
pub mod env;
pub mod jwt;

use std::{
    collections::VecDeque,
    ops::Deref,
    sync::{atomic::AtomicUsize, Arc},
};

use anyhow::anyhow;
use data::ChatMessage;
use jsonwebtoken::TokenData;
use jwt::Claims;
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Request,
};
use tokio::{
    sync::{broadcast, RwLock},
    task::JoinHandle,
};

#[derive(Debug)]
pub struct WSBackendState {
    _manager_handle: JoinHandle<()>,
    pub msg_buf: Arc<RwLock<VecDeque<ChatMessage>>>,
    pub atomic_counter: AtomicUsize,
    pub sender: Arc<broadcast::Sender<ChatMessage>>,
    pub receiver: Arc<broadcast::Receiver<ChatMessage>>,
}

impl WSBackendState {
    pub async fn create() -> Self {
        let (sender, receiver) = tokio::sync::broadcast::channel(10);
        let msg_buf = Arc::new(RwLock::new(VecDeque::with_capacity(200)));

        let manager_subscribe = sender.subscribe();
        let msg_buf_ref = msg_buf.clone();

        let manager_handle = tokio::spawn(async move {
            let mut manager_subscribe = manager_subscribe;
            let msg_buf = msg_buf_ref;

            while let Ok(msg) = manager_subscribe.recv().await {
                let mut msg_buf = msg_buf.deref().write().await;
                msg_buf.push_front(msg);
                let cap = msg_buf.capacity() / 4;
                let len = msg_buf.len();
                if len < cap {
                    continue;
                }

                msg_buf.truncate(cap);
            }

            ()
        });

        Self {
            _manager_handle: manager_handle,
            msg_buf,
            sender: Arc::new(sender),
            receiver: Arc::new(receiver),
            atomic_counter: AtomicUsize::new(1),
        }
    }
}

#[derive(Debug)]
pub struct ClerkUser<'r> {
    pub token: &'r TokenData<Claims>,
}

#[allow(unused_variables, dead_code)]
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClerkUser<'r> {
    type Error = anyhow::Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request
            .local_cache_async(async {
                dbg!("Cache Miss");

                let session_cookie = match request.cookies().get("__session") {
                    Some(r) => r,
                    None => {
                        return Err::<TokenData<Claims>, anyhow::Error>(anyhow!(
                            "Session not found"
                        ));
                    }
                };

                panic!("Tried restoring Token cache without running ClerkFairing")
            })
            .await;

        if let Err(err) = token {
            return request::Outcome::Error((Status::Unauthorized, anyhow!(Status::Unauthorized)));
        }

        let token = token.as_ref().unwrap();

        request::Outcome::Success(Self { token })
    }
}
