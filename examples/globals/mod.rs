// https://docs.rs/tracing-error/0.1.2/tracing_error/index.html
pub fn install() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;

    let subscriber = tracing_subscriber::Registry::default().with(ErrorLayer::default());

    tracing::subscriber::set_global_default(subscriber).unwrap();
}
