pub mod controller;
mod service;
mod entity;

use std::fs::OpenOptions;
use database::inmemory::InMemory;
use self::{entity::Workflow, service::WorkflowService};

pub fn service() -> WorkflowService {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("workflows.json")
        .unwrap();

    WorkflowService::new(InMemory::<Workflow>::new_with_file(file))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(controller::get_workflows)
       .service(controller::post_workflow)
       .service(controller::update_workflow)
       .service(controller::delete_workflow)
       .service(controller::get_workflow)
       .service(controller::get_workflow_name);
}
