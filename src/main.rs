#[allow(unused)]
#[macro_use] extern crate rocket;

mod workflows;
mod mongo;
mod inmemory;
mod database_traits;
use workflows::WorkflowRoutes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![get_current_time])
        .workflow_mount()
}
