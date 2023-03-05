use database::traits::Database;
use crate::workflows::entity::Workflow;

pub type WorkflowServiceType = Box<dyn Database<Workflow> + Send + Sync>;

pub struct WorkflowService(WorkflowServiceType);

pub const fn init(service: WorkflowServiceType) -> WorkflowService {
    WorkflowService(service)
}

impl WorkflowService {
    pub fn get_workflow(&self , id: i32) -> Workflow {
        match self.0.read(id) {
            Some(workflow) => workflow,
            None => panic!("Workflow not found")
        }
    }

    pub fn create_workflow(&self ,workflow: Workflow) -> Workflow {
        match self.0.insert(workflow){
            Some(workflow) => workflow,
            None => panic!("Workflow insertion failed")
        }
    }

}
