//#![feature(trace_macros)]
//trace_macros!(true);

#[allow(unused)]
#[macro_use] extern crate rocket;

mod workflows;
use workflows::WorkflowRoutes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .workflow_mount()
}
