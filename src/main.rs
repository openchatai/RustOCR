pub mod controllers;


use actix_web::middleware::Logger;
use actix_web::{ App, HttpServer};

use utoipa_swagger_ui::SwaggerUi;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(),
    components(schemas()),
    tags((name = "Opencopilot Entities", description = "CRUD operation on all endpoints")),
    modifiers()
)]
struct ApiDoc;

#[actix_web::main]
async fn main()  -> std::io::Result<()>  {
    std::env::set_var("RUST_LOG", "actix_web=info");
    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .configure(controllers::scan_controller::config)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
