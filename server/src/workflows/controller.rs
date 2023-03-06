use rocket::State;
use rocket::serde::json::Json;
use super::service::WorkflowService;
use super::entity::Workflow;

#[get("/<id>")]
pub fn get_workflow(service: &State<WorkflowService>, id: u64) -> Option<Json<Workflow>> {
   match service.get_workflow(id) {
       Some(wf) => Some(Json(wf.clone())),
       None => None
   }
}

#[get("/")]
pub fn get_workflows(service: &State<WorkflowService>) -> Json<Vec<Workflow>> {
   let wf = service.get_workflows();
   Json(wf.clone())
}

#[post("/", format = "json", data = "<workflow>")]
pub fn post_workflow(service: &State<WorkflowService>, workflow: Json<Workflow>) -> Json<Workflow> {
    let wf = service.create_workflow(workflow.into_inner());
    Json(wf.clone())
}

#[post("/<id>", format = "json", data = "<workflow>")]
pub fn update_workflow(service: &State<WorkflowService>, id: u64, workflow: Json<Workflow>) -> Json<Workflow> {
    let wf = service.update_workflow(id, workflow.into_inner());
    Json(wf.clone())
}

#[delete("/<id>")]
pub fn delete_workflow(service: &State<WorkflowService>, id: u64) -> Json<Workflow> {
    let wf = service.delete_workflow(id);
    Json(wf.clone())
}
