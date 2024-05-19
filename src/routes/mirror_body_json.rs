use axum::Json;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct JsonBody {
    message: String
}

#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_server: String
}

pub async fn mirror_body_json(Json(body): Json<JsonBody>) -> Json<MirrorJsonResponse> {
    Json(MirrorJsonResponse {
        message: body.message,
        message_from_server: "Hello from server!".to_owned()
    })
}