mod controllers;
mod routes;
mod views;

#[tokio::main]
async fn main() {
    let app = routes::router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind to 127.0.0.1:3000");
    println!("Frontend INVENTRA berjalan di http://127.0.0.1:3000");
    
    axum::serve(listener, app)
        .await
        .expect("frontend server failed");
}
