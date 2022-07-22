//! Implementation of [`TaskManager`]
//!
//! It is only used to manage processes and schedule process based on ready queue.
//! Other CPU process monitoring functions are in Processor.


use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use alloc::vec::Vec;
use lazy_static::*;
use crate::config::BIG_STRIDE;
use crate::task::TaskStatus;

pub struct TaskManager {
    ready_queue: Vec<Arc<TaskControlBlock>>,
}

// YOUR JOB: FIFO->Stride
/// A simple FIFO scheduler.
impl TaskManager {
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
        // Some(self.ready_queue.remove(0))
        self.stride()
    }
    // stride 调度算法
    pub fn stride(&mut self) -> Option<Arc<TaskControlBlock>> {
       if let Some((index,task)) = self.ready_queue.iter().enumerate().min_by_key(|(_,task)|{
           task.inner_exclusive_access().pass
       }){
           let mut inner = task.inner_exclusive_access();
           inner.pass +=inner.stride;
           drop(inner);
           Some(self.ready_queue.remove(index))
       }else {
           None
       }
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}
