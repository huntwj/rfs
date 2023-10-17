use poem::{
    listener::TcpListener,
    middleware::{AddData, Cors},
    web::{Data, Path},
    EndpointExt, Route, Server,
};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use sqlx::{PgPool, Pool, Postgres};

struct MyApi;

#[OpenApi]
impl MyApi {
    #[oai(path = "/hello/:name", method = "get")]
    async fn index(&self, _pool: Data<&Pool<Postgres>>, name: Path<String>) -> PlainText<String> {
        PlainText(format!("Hello, {}", name.0))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    let db = PgPool::connect("postgres://postgres@localhost").await?;
    let api_service = OpenApiService::new(MyApi, "MyApi", "0.1.0").server("http://localhost:3456");
    let ui = api_service.openapi_explorer();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/openapi/", ui)
        .with(Cors::new())
        .with(AddData::new(db));
    Server::new(TcpListener::bind("127.0.0.1:3456"))
        .run(app)
        .await?;

    Ok(())
}
