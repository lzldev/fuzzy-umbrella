use rocket::{Build, Rocket};

pub fn register(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/ws/", routes![sub_events])
}

#[get("/sub")]
fn sub_events<'a>(ws: rocket_ws::WebSocket) -> rocket_ws::Channel<'a> {
    use rocket::futures::{SinkExt, StreamExt};

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                let msg = match stream.next().await {
                    Some(m) => m,
                    None => break,
                };

                let msg = match msg {
                    Ok(m) => m,
                    Err(err) => {
                        eprintln!("Socket Stream Error:{:#?}", err);
                        break;
                    }
                };

                let _ = stream.send(msg).await;
            }

            Ok(())
        })
    })
}
