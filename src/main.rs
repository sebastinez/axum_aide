use aide::{
    axum::{routing::get, ApiRouter},
    openapi::OpenApi,
};
use axum::{extract::State, Extension};
use axum_jsonschema::Json;
use schemars::JsonSchema;
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Clone, Default)]
pub struct Context {
    hello: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}

pub async fn run() -> anyhow::Result<()> {
    let ctx = Context {
        hello: "world".to_owned(),
    };

    let mut api = OpenApi::default();
    let app = router(ctx).finish_api(&mut api).layer(Extension(api));

    let listener = TcpListener::bind("0.0.0.0:8082").await.unwrap();
    axum::serve(listener, app)
        .await
        .map_err(|err| anyhow::Error::from(err))
}

pub fn router(ctx: Context) -> ApiRouter {
    ApiRouter::new().route("/", get(handler)).with_state(ctx)
}

async fn handler(State(ctx): State<Context>) -> Result<Json<Stats>, anyhow::Error> {
    Ok(Json(Stats {
        projects: StatCount { count: 1 },
        users: StatCount { count: 0 },
        message: ctx.hello,
    }))
}

#[derive(Clone, Serialize, JsonSchema)]
struct StatCount {
    count: usize,
}

#[derive(Clone, Serialize, JsonSchema)]
struct Stats {
    projects: StatCount,
    users: StatCount,
    message: String,
}
