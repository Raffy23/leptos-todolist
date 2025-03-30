#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use tracing_subscriber::{filter, fmt::format::FmtSpan};

    tracing_subscriber::fmt()
        .with_env_filter(
            filter::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "leptos_todolist=debug".into()),
        )
        .with_level(true)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    leptos_todolist::server::main().await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
