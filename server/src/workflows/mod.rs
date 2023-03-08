pub mod controller;
mod service;
mod entity;

use std::fs::OpenOptions;
use database::inmemory::InMemory;


use self::entity::Workflow;

pub trait WorkflowRoutes {
    fn workflow_mount(self) -> Self;
}


pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("workflows.json")
        .unwrap();

    let s = service::init(InMemory::<Workflow>::new_with_file(file));

    cfg.app_data(actix_web::web::Data::new(s))
       .service(controller::get_workflows)
       .service(controller::post_workflow)
       .service(controller::update_workflow)
       .service(controller::delete_workflow)
       .service(controller::get_workflow);

    //cfg.service(controller::get_workflows)
}
