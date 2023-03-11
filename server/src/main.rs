//#![feature(trace_macros)]
//trace_macros!(true);

#[allow(unused)]

mod workflows;
use actix_web::{App, HttpServer, web};

/*
#[launch]
fn rocket() -> _ {
    rocket::build()
        .workflow_mount()
}
*/


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // No idea how to get around this for now
    let s = workflows::service();

    HttpServer::new(move || {
        App::new()
            .service(web::scope("/workflows")
                .app_data(web::Data::new(s.clone()))
                .configure(workflows::configure))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
