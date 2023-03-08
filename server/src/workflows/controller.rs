use actix_web::{HttpResponse, error};
use actix_web::{get, post, delete, web::Data, Result, web::Path, Responder, web::Json};
use super::service::WorkflowService;
use super::entity::Workflow;

#[get("")]
pub async fn get_workflows(service: Data<WorkflowService>) -> Result<impl Responder> {
   let wf = service.get_workflows();
   println!("get_workflows: {:?}", wf);
   Ok(Json(wf))
}

#[get("/{id}")]
pub async fn get_workflow(service: Data<WorkflowService>, path: Path<u64>) -> Result<impl Responder> {
   let id = path.into_inner();

   match service.get_workflow(id) {
       Some(wf) => {
           println!("get_workflows: {:?}", wf);
           Ok(HttpResponse::Ok().json(wf))
       },
       None => Err(error::ErrorNotFound("Workflow not found"))
   }
}


#[post("")]
pub async fn post_workflow(service: Data<WorkflowService>, workflow: Json<Workflow>) -> Result<impl Responder>{
    let wf = service.create_workflow(workflow.into_inner());
    Ok(Json(wf))
}

#[post("/{id}")]
pub async fn update_workflow(service: Data<WorkflowService>, path: Path<u64>,workflow: Json<Workflow>) -> Result<impl Responder> {
    let id = path.into_inner();

    let wf = service.update_workflow(id, workflow.into_inner());
    Ok(Json(wf))
}

#[delete("/{id}")]
pub async fn delete_workflow(service: Data<WorkflowService>, path: Path<u64>) ->  Result<impl Responder> {
    let id = path.into_inner();

    let wf = service.delete_workflow(id);
    Ok(Json(wf))
}
