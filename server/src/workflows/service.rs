use database::traits::IndexableDatabase;
use super::entity::Workflow;

pub type WorkflowServiceType = Box<dyn IndexableDatabase<Workflow> + Send + Sync>;

pub struct WorkflowService(WorkflowServiceType);

pub const fn init(service: WorkflowServiceType) -> WorkflowService {
    WorkflowService(service)
}

impl WorkflowService {
    pub fn get_workflow(&self, id: u64) -> Option<Workflow> {
        self.0.read(id)
    }

    pub fn get_workflows(&self) -> Vec<Workflow> {
        self.0.read_all()
    }

    pub fn create_workflow(&self, mut workflow: Workflow) -> Workflow {
        workflow.created_at = chrono::offset::Utc::now();
        workflow.updated_at = chrono::offset::Utc::now();

        match self.0.insert(workflow){
            Some(workflow) => workflow,
            None => panic!("Workflow insertion failed")
        }
    }

    pub fn update_workflow(&self, id: u64, mut workflow: Workflow) -> Workflow {
        workflow.updated_at = chrono::offset::Utc::now();

        match self.0.update(id, workflow){
            Some(workflow) => workflow,
            None => panic!("Workflow insertion failed")
        }
    }

    pub fn delete_workflow(&self, id: u64) -> Workflow {
        match self.0.delete(id) {
            Some(workflow) => workflow,
            None => panic!("Workflow not found")
        }
    }

}
