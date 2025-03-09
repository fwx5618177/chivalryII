use crate::logging::{GameLogger, LogLevel};
use bevy::prelude::*;

#[derive(Debug, Event)]
pub enum NetworkEvent {
    ConnectionSuccess,
    ConnectionFailed,
    Disconnection,
    MessageReceived(String),
    MessageSent(String),
}

#[derive(Resource, Default)]
pub struct NetworkState {
    pub is_connected: bool,
    pub server_address: String,
    pub client_id: String,
    pub latency: f32,
    pub packet_loss: f32,
    pub last_ping: f32,
}

pub fn handle_network_events(
    mut events: EventReader<NetworkEvent>,
    mut state: ResMut<NetworkState>,
    mut logger: ResMut<GameLogger>,
) {
    for event in events.read() {
        match event {
            NetworkEvent::ConnectionSuccess => {
                state.is_connected = true;
                logger.log(LogLevel::Info, "网络连接成功");
            }
            NetworkEvent::ConnectionFailed => {
                state.is_connected = false;
                logger.log(LogLevel::Error, "网络连接失败");
            }
            NetworkEvent::Disconnection => {
                state.is_connected = false;
                logger.log(LogLevel::Error, "网络连接断开");
            }
            NetworkEvent::MessageReceived(message) => {
                logger.log(LogLevel::Debug, &format!("收到消息: {}", message));
            }
            NetworkEvent::MessageSent(message) => {
                logger.log(LogLevel::Debug, &format!("发送消息: {}", message));
            }
        }
    }
}
