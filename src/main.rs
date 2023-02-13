use poem::{get, handler, listener::TcpListener, Route, Server};

#[handler]
fn home() -> String {
    format!(
        r#"{{
    "code": "400",
    "msg": "access denied"
}}"#
    )
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/", get(home));
    Server::new(TcpListener::bind("127.0.0.1:80"))
        .run(app)
        .await
}
