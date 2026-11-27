mod controllers;
mod middleware;
mod models;
mod routes;
mod services;
mod state;
mod views;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = state::AppState::from_env().await;
    let app = routes::router(state.pool, state.jwt_secret);

    let listener = TcpListener::bind((state.app_host.as_str(), state.app_port))
        .await
        .unwrap_or_else(|error| {
            panic!(
                "Gagal membuka listener pada {}:{}: {error}",
                state.app_host, state.app_port
            )
        });

    let address = listener
        .local_addr()
        .expect("Gagal membaca alamat listener");
    println!("Server berjalan di http://{address}");

    axum::serve(listener, app)
        .await
        .expect("Server gagal berjalan");
}
