pub mod controller;
mod service;
mod entity;

use std::fs::OpenOptions;
use database::inmemory::InMemory;
use rocket::Build;
use rocket::Rocket;

use self::entity::Workflow;

pub trait WorkflowRoutes {
    fn workflow_mount(self) -> Self;
}

impl WorkflowRoutes for Rocket<Build> {
    fn workflow_mount(self) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("workflows.json")
            .unwrap();

        let s = service::init(InMemory::<Workflow>::new_with_file(file));

        self.mount("/workflows", routes![
            controller::get_workflow,
            controller::get_workflows,
            controller::post_workflow,
            controller::update_workflow,
            controller::delete_workflow,
        ])
            .manage(s)
    }
}
