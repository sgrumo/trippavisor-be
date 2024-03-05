pub mod utils;

use worker::*;

use crate::utils::models::festival::*;

const DATO_CMS_URL: &str = "https://graphql.datocms.com/";

#[event(fetch)]
async fn main(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let client = reqwest::Client::new();
    let body = req.bytes().await?;

    let body = reqwest::Body::from(body);

    let bearer = match env.secret("BEARER_TOKEN") {
        Ok(value) => value.to_string(),
        Err(err) => return Response::error(format!("Internal Server Error: {err}"), 500),
    };

    let res = client
        .post(DATO_CMS_URL)
        .body(body)
        .header("Authorization", format!("Bearer {bearer}"))
        .send()
        .await;

    let response_body = match res {
        Ok(response) => match response.bytes().await {
            Ok(bytes) => bytes.to_vec(),
            Err(err) => return Response::error(format!("Internal Server Error: {err}"), 500),
        },
        Err(err) => return Response::error(format!("Internal Server Error: {err}"), 500),
    };

    let serialized: FestivalsResponse = match serde_json::from_slice(&response_body) {
        Ok(value) => value,
        Err(err) => return Response::error(format!("Internal Server Error: {err}"), 500),
    };

    let origins = ["https://trippavisor.it", "https://dev.trippavisor.it"];
    let cors_headers: Vec<&str> = vec!["Content-Type"];

    let cors = Cors::default()
        .with_allowed_headers(cors_headers)
        .with_origins(origins)
        .with_methods(Method::all());

    match Response::from_json(&serialized) {
        Ok(response) => response.with_cors(&cors),
        Err(err) => return Response::error(format!("Internal Server Error: {err}"), 500),
    }
}
