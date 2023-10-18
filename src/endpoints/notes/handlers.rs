use rocket::{response::status::NotFound, serde::json::Json, Route};

use super::dto::GetNotesResponseDto;

pub fn provide_routes_notes() -> Vec<Route> {
    routes![get_index, post_index]
}

//TODO: Change this to actual server response in working build
#[get("/notes")]
pub fn get_index() -> Result<Json<GetNotesResponseDto>, NotFound<String>> {
    Ok(Json(GetNotesResponseDto::dummy()))
}

#[post("/notes", format = "application/json", data = "<note>")]
pub fn post_index(note: rocket::serde::json::Json<crate::models::note::Note>) -> String {
    String::from("Sucessful post")
}
