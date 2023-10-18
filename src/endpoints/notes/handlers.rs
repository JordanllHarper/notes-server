use rocket::{response::status::NotFound, serde::json::Json, Route};

use super::dto::GetNotesResponseDto;

pub fn provide_routes_notes() -> Vec<Route> {
    routes![get_notes]
}

//TODO: Change this to actual notes response in working build
#[get("/notes")]
pub fn get_notes() -> Result<Json<GetNotesResponseDto>, NotFound<String>> {
    Ok(Json(GetNotesResponseDto::dummy()))
}
