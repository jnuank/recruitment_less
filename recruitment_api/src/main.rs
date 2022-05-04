mod routers;

mod domain;

#[tokio::main]
async fn main() {
    let app = routers::router();

    let candidate  = domain::candidate::Candidate { name: "bob".to_string(), age: 20 };

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
