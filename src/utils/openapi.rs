use axum::routing::get;
use axum::Router;
use utoipa::openapi::{self, InfoBuilder};
use utoipa::OpenApi;

use crate::handlers;

use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    Json,
};
use utoipa_redoc::Redoc;

pub fn create_openapi_docs() -> openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        servers(
            (url = "https://openapi.ankarhem.dev"),
        ),
        paths(
            handlers::openapi::get,
            // handlers::worlds::get,
            // handlers::worlds_world_name::get,
            // handlers::worlds_world_name_guilds::get,
            // handlers::worlds_world_name_kill_statistics::get,
            // handlers::worlds_world_name_residences::get,
        ),
        // components(schemas(

        // )),
        tags()
    )]
    struct ApiDocV1;
    let mut openapi = ApiDocV1::openapi();
    openapi.info = InfoBuilder::new()
        .title("Remote Types")
        // .description(Some(API_DESCRIPTION))
        .version("1.0.0")
        .build();

    openapi
}

pub fn openapi_routes() -> Router {
    let docs = create_openapi_docs();

    Router::new()
        .route("/", get(redirect_redocly))
        .route("/openapi.json", get(serve_openapi))
        .route("/api-docs", get(serve_redocly))
        .with_state(docs)
}

async fn redirect_redocly() -> Redirect {
    Redirect::temporary("/api-docs")
}

async fn serve_openapi(State(openapi_docs): State<openapi::OpenApi>) -> impl IntoResponse {
    Json(openapi_docs)
}

async fn serve_redocly() -> impl IntoResponse {
    Html(
        Redoc::new("/openapi.json")
            .custom_html(CUSTOM_HTML)
            .to_html(),
    )
}

const CUSTOM_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
  <title>Remote Types</title>
  <!-- needed for adaptive design -->
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <link rel="icon" href="/favicon.png">
  <link
    href="https://fonts.googleapis.com/css?family=Montserrat:300,400,700|Roboto:300,400,700"
    rel="stylesheet"
  />

  <!--
  Redoc doesn't change outer page styles
  -->
  <style>
    body {
      margin: 0;
      padding: 0;
    }
  </style>
</head>
<body>
  <div id="redoc-container"></div>
  <script src="https://cdn.redoc.ly/redoc/latest/bundles/redoc.standalone.js"></script>
  <script>
    Redoc.init(
      $spec,
      $config,
      document.getElementById("redoc-container")
    );
  </script>
</body>
</html>
"#;
