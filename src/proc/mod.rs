pub mod linkedlist;
pub mod manager;
pub mod process;

use self::manager::TaskManager;
use manager::LockedManager;

pub static TASKMANAGER: LockedManager = LockedManager::new(TaskManager::init());
