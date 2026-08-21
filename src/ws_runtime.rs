//! Async RFC 6455 WebSocket hub with bounded fan-out and lag handling.

use std::sync::{OnceLock, RwLock};

use axum::extract::ws::{CloseFrame, Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;

static ENDPOINT: OnceLock<RwLock<Option<String>>> = OnceLock::new();
static HUB: OnceLock<broadcast::Sender<String>> = OnceLock::new();

pub fn enable(endpoint: &str) -> Result<(), String> {
    if !endpoint.starts_with('/') || endpoint.contains('?') || endpoint.contains('#') {
        return Err("WebSocket endpoint must be an absolute path without query or fragment".into());
    }
    let mut guard = ENDPOINT
        .get_or_init(|| RwLock::new(None))
        .write()
        .map_err(|_| "WebSocket endpoint lock is poisoned")?;
    *guard = Some(endpoint.to_string());
    let _ = sender();
    Ok(())
}

pub fn endpoint() -> Option<String> {
    ENDPOINT.get().and_then(|value| value.read().ok()?.clone())
}

pub fn broadcast(message: String) -> usize {
    sender().send(message).unwrap_or(0)
}

fn sender() -> &'static broadcast::Sender<String> {
    HUB.get_or_init(|| broadcast::channel(1024).0)
}

pub async fn serve(socket: WebSocket) {
    let (mut outgoing, mut incoming) = socket.split();
    let mut subscription = sender().subscribe();
    loop {
        tokio::select! {
            message = incoming.next() => match message {
                Some(Ok(Message::Text(text))) => { let _ = sender().send(text.to_string()); }
                Some(Ok(Message::Binary(_))) => {}
                Some(Ok(Message::Ping(payload))) => {
                    if outgoing.send(Message::Pong(payload)).await.is_err() { break; }
                }
                Some(Ok(Message::Close(_))) | None | Some(Err(_)) => break,
                Some(Ok(Message::Pong(_))) => {}
            },
            message = subscription.recv() => match message {
                Ok(text) => if outgoing.send(Message::Text(text.into())).await.is_err() { break; },
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    let _ = outgoing.send(Message::Close(Some(CloseFrame {
                        code: 1013, reason: "client exceeded WebSocket backpressure budget".into(),
                    }))).await;
                    break;
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_endpoint_and_uses_bounded_hub() {
        assert!(enable("ws").is_err());
        enable("/events").unwrap();
        let mut receiver = sender().subscribe();
        assert_eq!(broadcast("hello".into()), 1);
        assert_eq!(receiver.try_recv().unwrap(), "hello");
    }
}
