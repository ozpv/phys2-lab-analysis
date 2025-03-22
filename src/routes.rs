use crate::app::{shell, App};
use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use std::time::Duration;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};

/// # Panics
///
/// Shouldn't panic, but it will if the app isn't ran with `cargo-leptos` or doesn't have `Cargo.toml`
/// available
pub fn app() -> Router {
    #[cfg(test)]
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    #[cfg(not(test))]
    let conf = get_configuration(None).unwrap();

    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new().br(true))
        // timeout needs to exist for graceful shutdown
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
}
