use axum::extract::{Json, Path, Query};
use axum::handler;
#[allow(dead_code, unused, unused_imports)]
use axum::{routing::get, Router};
use serde::{de, Deserialize, Deserializer};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::{fmt, str::FromStr};

// `Path ` gives you the path parameters and deserializes them
async fn path(Path(user_id): Path<u32>) {}

// `Query` gives you the query parameters and deserializes them.
async fn query(Query(Params): Query<HashMap<String, String>>) {}

async fn json() -> Json<Value> {
    Json(json!({"data": 42}))
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
async fn name() -> &'static str {
    "Hello, Rana!"
}

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/hello", get(handler));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn handler(Query(params): Query<Params>) ->String{
    format!("{:?}", params)
}

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    foo: Option<i32>,
    bar: Option<String>

}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T> ,D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
    {
        let op = Option::<String>::deserialize(de)?;
        match op.as_deref() {
            None | Some("") => Ok(None),
            Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some)
        }
    }
