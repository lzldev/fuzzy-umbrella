pub mod auth;
pub mod data;
pub mod env;
pub mod jwt;

use std::{
    collections::VecDeque,
    ops::Deref,
    sync::{
        atomic::{AtomicIsize, AtomicUsize},
        Arc,
    },
    time::Duration,
};

use anyhow::anyhow;
use data::ChatMessage;
use fred::{
    clients::RedisClient,
    interfaces::{ClientLike, EventInterface, KeysInterface, PubsubInterface},
    types::{Message, RedisConfig},
};
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
    pub atomic_counter: AtomicIsize,
    pub sender: Arc<broadcast::Sender<ChatMessage>>,
    pub receiver: Arc<broadcast::Receiver<ChatMessage>>,
}


impl WSBackendState {
    pub async fn create() -> Self {
        let (sender, receiver) = tokio::sync::broadcast::channel(10);
        let sender = Arc::new(sender);
        let msg_buf = Arc::new(RwLock::new(VecDeque::with_capacity(200)));

        let manager_subscribe = sender.subscribe();
        let msg_buf_ref = msg_buf.clone();

        let client = RedisClient::new(
            RedisConfig {
                version: fred::types::RespVersion::RESP3,
                ..Default::default()
            },
            None,
            None,
            None,
        );

        client.init().await.expect("Redis to connect");

        let _: () = client.ping().await.expect("AAAAAAAAA");

        let sender2 = sender.clone();

        let manager_handle = tokio::spawn(async move {
            let sender3 = sender2.clone();
            let _handle3 = tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(5));
                loop {
                    interval.tick().await;
                    sender3
                        .send(ChatMessage {
                            user_id: 0,
                            content: format!("[SERVER] Hello from server :3"),
                        })
                        .unwrap();
                }
            });
            dbg!("Manager");
            let client = client;

            let _: () = client
                .set("TEST_KEY", "0", None, None, false)
                .await
                .expect("To set key ");

            let mut sub = client.message_rx();
            let _ = client.subscribe("CHAT:GLOBAL").await;

            client.on_message(|m| {
                dbg!(&m);
                Ok(())
            });

            let mut manager_subscribe = manager_subscribe;
            let msg_buf = msg_buf_ref;

            let msg_buf2 = msg_buf.clone();

            let redis_handler = |redis_msg: Result<Message, broadcast::error::RecvError>| async {
                let redis_msg = redis_msg.unwrap();

                let msg = redis_msg.value.as_string().unwrap();

                sender2
                    .send(ChatMessage {
                        user_id: 0,
                        content: format!("[GLOBAL]  {}", msg),
                    })
                    .unwrap();
            };

            loop {
                tokio::select! {
                        redis_msg = sub.recv() => redis_handler(redis_msg).await,
                        manager_message = manager_subscribe.recv() => {

                        let msg = manager_message.unwrap();
                        let mut msg_buf = msg_buf.deref().write().await;
                        msg_buf.push_front(msg);
                        let cap = msg_buf.capacity() / 4;
                        let len = msg_buf.len();
                        if len < cap {
                            continue;
                        }

                        msg_buf.truncate(cap);
                    }
                };
            }

            // while let Ok(msg) = manager_subscribe.recv().await {
            //     let mut msg_buf = msg_buf.deref().write().await;
            //     msg_buf.push_front(msg);
            //     let cap = msg_buf.capacity() / 4;
            //     let len = msg_buf.len();
            //     if len < cap {
            //         continue;
            //     }

            //     msg_buf.truncate(cap);
            // }

            ()
        });

        Self {
            _manager_handle: manager_handle,
            msg_buf,
            sender: sender,
            receiver: Arc::new(receiver),
            atomic_counter: AtomicIsize::new(1),
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
