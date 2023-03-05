use rocket::State;
use crate::workflows::service::WorkflowService;
use rocket::serde::json::Json;
use crate::workflows::entity::Workflow;

#[get("/<id>")]
pub fn get_workflow(service: &State<WorkflowService>, id: i32) -> Json<Workflow> {
   let wf = service.get_workflow(id);
   Json(wf.clone())
}

#[post("/", format = "json", data = "<workflow>")]
pub fn post_workflow(service: &State<WorkflowService>, workflow: Json<Workflow>) -> Json<Workflow> {
    let wf = service.create_workflow(workflow.into_inner());
    Json(wf.clone())
}
