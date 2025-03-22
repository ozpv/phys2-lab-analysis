use leptos::prelude::*;
use phys2_lab_analysis::routes::app;

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
        .await
        .unwrap();
}
