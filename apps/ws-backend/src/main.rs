#[macro_use]
extern crate rocket;

use artspace_core::env::EnvContainer;
use rocket::{http::Method, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use tokio::time::Duration;
use ws_backend::{
    auth::{ClerkFairing, WSBackendJWKS},
    data::ChatMessage,
    env::WSBackendEnvVars,
    WSBackendState,
};

#[launch]
async fn launch() -> _ {
    let env = ws_backend::env::WSBackendEnv::load_env();

    let origins = env
        .get_env_var(WSBackendEnvVars::CORSOrigins)
        .split(",")
        .map(str::to_owned)
        .collect::<Vec<String>>();

    let jwks_state =
        WSBackendJWKS::from_uri(env.get_env_var(WSBackendEnvVars::ClerkJWKSUrl).as_str())
            .await
            .expect("To create a jwks state.");

    dbg!(&origins);

    let allowed_origins = AllowedOrigins::some_exact(origins.as_slice());

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("to build cors options.");

    let state = WSBackendState::create().await;

    rocket::build()
        .manage::<WSBackendJWKS>(jwks_state)
        .manage::<WSBackendState>(state)
        .attach(ClerkFairing {})
        .attach(cors)
        .mount(
            "/ws",
            routes![ws_backend::auth::unauthorized_get, ping_get, echo_channel],
        )
}

#[get("/ping")]
fn ping_get() -> &'static str {
    "Pong"
}

#[get("/echo")]
fn echo_channel(ws: rocket_ws::WebSocket, state: &State<WSBackendState>) -> rocket_ws::Channel<'_> {
    use rocket::futures::{SinkExt, StreamExt};

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let n = state
                .atomic_counter
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            let mut interval = tokio::time::interval(Duration::from_secs(60));
            let _ = interval.tick().await;

            let _ = {
                let msg_buf = state.msg_buf.read().await;
                let mut old = msg_buf
                    .range(
                        ..if msg_buf.len() < 50 {
                            msg_buf.len()
                        } else {
                            50
                        },
                    )
                    .rev();

                while let Some(msg) = old.next() {
                    let _ = stream.send(msg.content.as_str().into()).await?;
                }
            };

            let sender = state.sender.clone();
            let mut receiver = state.sender.subscribe();

            loop {
                tokio::select! {
                   msg = receiver.recv() => {
                        let msg = msg.expect("To unwrap channel message");
                        let _ = stream.send(msg.content.as_str().into()).await;
                    },
                    _ = interval.tick() => {
                            // sender.send(
                            //    ChatMessage {
                            //         user_id:0,
                            //         content:String::from("[SERVER] Hii from Server :3 ")
                            //    }
                            // ).unwrap();
                    },
                    msg = stream.next() => {

                        let message = match msg {
                            Some(Ok(msg)) => msg,
                            Some(Err(_)) => break,
                            None => break
                        };

                        if !message.is_text(){
                            continue;
                        }

                        let txt = message.to_text().unwrap();

                        if txt == "what_count" {
                            let v = &state
                                .atomic_counter
                                .load(std::sync::atomic::Ordering::Relaxed);
                            let v = v.clone() -1 ;
                            let _ = stream.send(format!("[DEBUG] Connections: {v}").into()).await;
                            continue;
                        }

                        let txt : &str = message.to_text().unwrap();
                        sender.send(ChatMessage {
                            content:format!("[{}] {}",n,txt).to_owned(),
                            user_id:n.clone() as usize,
                        }).unwrap();
                    }
                }
            }

            let _ = &state
                .atomic_counter
                .fetch_sub(1, std::sync::atomic::Ordering::Relaxed);

            dbg!("Closing connection");

            Ok(())
        })
    })
}
