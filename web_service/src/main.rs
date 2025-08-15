use poem::listener::TcpListener;
use poem::{Route, Server};
use poem_openapi::payload::PlainText;
use poem_openapi::{OpenApi, OpenApiService};
use std::io::Result;

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/open-door", method = "post")]
    async fn open_door(&self) -> PlainText<&'static str> {
        PlainText("Türe öffne dich...")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_service = OpenApiService::new(Api, "Richi Remote Door Web Service", "1.0")
        .server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", ui).nest("/api", api_service);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
