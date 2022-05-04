mod routers;

#[tokio::main]
async fn main() {
    let app = routers::router();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
