#![allow(unused)]

use core::ptr::null_mut;

enum ProcState {
    Unused,
    Used,
    Sleeping,
    Ready,
    Running,
    Zombie,
}

pub struct Proc {
    pid: usize,
    state: ProcState,
    parent: *mut Proc,
    next: *mut Proc,
}

impl Proc {
    pub fn new() -> Self {
        Self {
            pid: 0,
            state: ProcState::Unused,
            parent: null_mut(),
            next: null_mut(),
        }
    }
}
