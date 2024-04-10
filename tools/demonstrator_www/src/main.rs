use axum::{extract::Form, response::Html, routing::get, Router};
use axum::http::Method;
use axum::routing::get_service;
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tower_http::{cors, services::{ServeDir, ServeFile}, trace::TraceLayer};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_form=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with some routes
    let app = Router::new()
        // .route("/", get(show_form).post(accept_form))
        .route("/", get(show_form))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(
            CorsLayer::new()
                .allow_origin(cors::Any)
                .allow_headers(cors::Any)
                .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        )
        ;

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn show_form() -> Html<&'static str> {

    Html(
        r#"
        <!doctype html>
        <html>
            <head>
            <script type="text/javascript"
                src="https://cdn.jsdelivr.net/npm/@massalabs/massa-web3/bundle.js">
            </script>
            <script src="https://cdn.jsdelivr.net/npm/axios/dist/axios.min.js"></script>
            <script type="module" src="assets/sc_scan_01.js"></script>
            <script type="text/javascript">
                console.log("foo");
                // myjs_print();
            </script>
            <script type="text/javascript">
                console.log("foo");
                // myjs_print();
            </script>
            </head>
            <body>
                <form id="form_address" action="/" method="post">
                    <label id="label_address" for="address">
                        Enter a Smart Contract address:
                        <input id="input_address" type="text" name="address">
                    </label>

                    <!-- <input type="submit" value="Parse" onclick="parseFromAddress();"> -->
                    <button id="parse" type="button">Parse</button>
                </form>
            </body>
            <script type="module" src="assets/myjs.js"></script>
            <script>
                console.log("Try to print Massa Web3...");
                console.log("Massa Web3 ", window.massa);
                // const [client, account] = getClient();
                // console.log("Launching main function...");
                // import {main} from "./myjs";
                // main();
            </script>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    address: String,
}

async fn accept_form(Form(input): Form<Input>) {
    dbg!(&input);
}