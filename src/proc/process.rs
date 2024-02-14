#![allow(unused)]
enum ProcState {
    Unused,
    Used,
    Sleeping,
    Runnable,
    Running,
    Zombie,
}

pub struct Proc {
    state: ProcState,
    parent: *mut Proc,
}
