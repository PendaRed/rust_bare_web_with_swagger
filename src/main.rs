use std::{
    error::Error,
    future::{self, Ready},
    net::Ipv4Addr,
};

use crate::web_api::api_map_info::{get_mapinfo_by_name,MapInfo};
use crate::web_api::api_map_info::__path_get_mapinfo_by_name;
use utoipa::OpenApi;
use actix_web::{get, web, App, HttpServer, Responder};
use utoipa_swagger_ui::SwaggerUi;
use log::{debug, error, log_enabled, info, Level};

mod web_api;
mod web_server;

#[actix_web::main]
pub async fn main() -> Result<(), impl Error> {

    #[derive(OpenApi)]
    #[openapi(paths(
        web_api::api_map_info::get_mapinfo_by_name
    ),
    components(
        schemas(MapInfo)
    ))]
    struct ApiDoc;


    env_logger::init();
    let logo = r##"
       ______  ______
      / / __ \/ ____/
 __  / / /_/ / / __
/ /_/ / ____/ /_/ /
\____/_/    \____/
"##;

    // println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
    let host = "127.0.0.1";
    let port = 8989;

    println!("{}
Starting webserver:
   {}:{}/swagger-ui/
   {}:{}/map/info/<game_name>", logo, host, port, host, port);
    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", openapi.clone()),
            )
            .service(get_mapinfo_by_name)
    })
//        .bind((Ipv4Addr::UNSPECIFIED, 8989)).unwrap()
        .bind((host, port)).unwrap()
        .run()
        .await
}
