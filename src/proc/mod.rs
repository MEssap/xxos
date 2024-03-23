pub mod linkedlist;
pub mod manager;
pub mod process;
use manager::LockedManager;

use self::manager::TaskManager;
pub static TASKMANAGER: LockedManager = LockedManager::new(TaskManager::init());
