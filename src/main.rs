use leptos::prelude::*;
use phys2_lab_analysis::routes::app;
use tokio::signal;

#[tokio::main]
async fn main() {
    #[cfg(test)]
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    #[cfg(not(test))]
    let conf = get_configuration(None).unwrap();

    let addr = conf.leptos_options.site_addr;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        //.with_max_level(tracing::Level::INFO)
        .init();

    let app = app();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .inspect_err(|_| tracing::error!("failed to install ctrl-c handler"))
            .unwrap();
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .inspect_err(|_| tracing::error!("failed to install signal handler"))
            .unwrap()
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => { tracing::info!("recieved ctrl-c") },
        () = terminate => {},
    }
}
