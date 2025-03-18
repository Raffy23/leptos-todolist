use crate::app::*;
use crate::auth::new_session_layer;
use crate::database;
use crate::repository::UserRepository;
use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

pub async fn main() {
    let db = database::create_pool().await;
    let user_repository = UserRepository::new(db.clone());

    // Routes from Leptos configuration
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    let auth_layer = new_session_layer(db.clone(), user_repository.clone()).await;

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options.clone(),
            routes,
            {
                let user_repository = user_repository.clone();

                move || {
                    provide_context(user_repository.clone());
                }
            },
            {
                let leptos_options = leptos_options.clone();

                move || shell(leptos_options.clone())
            },
        )
        //.fallback(leptos_axum::file_and_error_handler(shell))
        .fallback(leptos_axum::file_and_error_handler_with_context(
            {
                let user_repository = user_repository.clone();

                move || {
                    provide_context(user_repository.clone());
                }
            },
            shell,
        ))
        .with_state(leptos_options)
        .layer(auth_layer)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    info!("Listening on http://{}", &addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
