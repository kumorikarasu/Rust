use database::{inmemory::InMemory, traits::{Indexable, IndexType, Database, IndexableDatabase}};
use super::entity::Workflow;

#[derive(Clone)]
pub struct WorkflowService {
    db: Box<InMemory<Workflow>>
}

impl WorkflowService {
    pub fn new(db: Box<InMemory<Workflow>>) -> WorkflowService {
        WorkflowService{
            db
        }
    }

    pub fn get_workflow(&self, id: u64) -> Option<Workflow> {
        self.db.read(id)
    }

    pub fn get_workflows(&self) -> Vec<Workflow> {
        self.db.read_all()
    }

    pub fn get_workflow_name(&self, name: String) -> Vec<Workflow> {
        self.db.search("name", IndexType::String(name))
    }

    pub fn create_workflow(&self, mut workflow: Workflow) -> Workflow {
        match self.db.insert(workflow){
            Some(workflow) => workflow,
            None => panic!("Workflow insertion failed")
        }
    }

    pub fn update_workflow(&self, id: u64, mut workflow: Workflow) -> Workflow {
        workflow.updated_at = chrono::offset::Utc::now();

        match self.db.update(id, workflow){
            Some(workflow) => workflow,
            None => panic!("Workflow insertion failed")
        }
    }

    pub fn delete_workflow(&self, id: u64) -> Workflow {
        match self.db.delete(id) {
            Some(workflow) => workflow,
            None => panic!("Workflow not found")
        }
    }
}
