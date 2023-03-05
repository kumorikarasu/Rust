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
        .workflow_mount()
}
