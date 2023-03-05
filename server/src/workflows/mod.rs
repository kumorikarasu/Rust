pub mod controller;
mod service;
mod entity;

use crate::workflows::entity::Workflow;
use database::inmemory::InMemory;
use rocket::Build;
use rocket::Rocket;

pub trait WorkflowRoutes {
    fn workflow_mount(self) -> Self;
}

impl WorkflowRoutes for Rocket<Build> {
    fn workflow_mount(self) -> Self {
        let s = service::init(InMemory::<Workflow>::new());

        self.mount("/workflows", routes![controller::get_workflow, controller::post_workflow])
            .manage(s)
    }
}
