use xx_mutex_lock::Mutex;

use super::process::Tcb;
use alloc::{collections::VecDeque, sync::Arc};

pub type LockedManager = Mutex<TaskManager>;

pub struct TaskManager {
    pub tasks: VecDeque<Arc<Tcb>>,
}

impl TaskManager {
    pub const fn init() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }
    pub fn push(&mut self, task: Arc<Tcb>) {
        self.tasks.push_back(task);
    }

    pub fn pop(&mut self) -> Option<Arc<Tcb>> {
        let task = self.tasks.pop_front();
        self.tasks.push_back(task.clone().unwrap());
        task
    }
}
