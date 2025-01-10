use tokio::sync::broadcast;

pub fn setup_broadcast_channel() -> (broadcast::Sender<String>, broadcast::Receiver<String>) {
    broadcast::channel::<String>(100)
}
