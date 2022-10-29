use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MapInfo {
    id: u64,
    name: String,
    width: u32,
    height: u32,
}

#[utoipa::path(
    get,
    path = "/map/info/{game_name}",
    responses(
        (status = 200, description = "Map found succesfully", body = MapInfo),
        (status = 404, description = "MapInfo was not found")
    ),
    params(
        ("game_name" = String, Path, description = "Name for the map"),
    )
)]
#[get("/map/info/{game_name}")]
pub async fn get_mapinfo_by_name(game_name: web::Path<String>) ->  impl Responder {
    HttpResponse::Ok().json(MapInfo {
        id: 1,
        name: game_name.into_inner(),
        width: 10,
        height: 10,
    })
}

