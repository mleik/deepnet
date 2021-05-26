use deepnet_site::StateManger;
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting DeepNet Site");

    let _state = StateManger::new();
}
