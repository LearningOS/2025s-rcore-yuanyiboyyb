//!Implementation of [`TaskManager`]
use super::task::BIG_STRIDE;
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::sync::Arc;
use alloc::vec::Vec;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: Vec<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: Vec::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        let mut min_index: Option<usize> = None;
        let mut min_boot = 0; 

        for (index, person) in self.ready_queue.iter().enumerate() {
            if person.boot < min_boot {
                min_index = Some(index);
                min_boot = person.boot;
            }
        }

        // 如果找到最小值，删除并返回对应的 TaskControlBlock
        if let Some(index) = min_index {
            if let Some(task) = Arc::get_mut(&mut self.ready_queue[index]){
                task.boot+=task.priority;
            }else{
                unreachable!();
            }
            Some(self.ready_queue.remove(index))
        } else {
            None
        }
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
