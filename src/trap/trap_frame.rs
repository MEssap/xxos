use crate::riscv::registers::{scause::Scause, sepc::Sepc};

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct TrapFrame {
    ra: usize,
    sp: usize,
    gp: usize,
    tp: usize,
    t0: usize,
    t1: usize,
    t2: usize,
    s0: usize,
    s1: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
    s2: usize,
    s3: usize,
    s4: usize,
    s5: usize,
    s6: usize,
    s7: usize,
    s8: usize,
    s9: usize,
    s10: usize,
    s11: usize,
    t3: usize,
    t4: usize,
    t5: usize,
    t6: usize,

    // CSR
    sepc: Sepc,
    scause: Scause,
    //sbadaddr: usize,
    //scratch: usize,
}

impl TrapFrame {
    pub fn store_in_stack() {}

    pub fn set_sp(&mut self, sp: usize) {
        self.sp = sp;
    }

    pub fn sepc(&self) -> usize {
        self.sepc.bits()
    }

    pub fn set_sepc(&mut self, pc: usize) {
        self.sepc.set_bits(pc);
    }

    pub fn scause(&self) -> &Scause {
        &self.scause
    }

    pub fn a0(&self) -> usize {
        self.a0
    }

    pub fn a1(&self) -> usize {
        self.a1
    }

    pub fn a2(&self) -> usize {
        self.a2
    }

    pub fn a7(&self) -> usize {
        self.a7
    }

    pub fn set_a0(&mut self, a0: usize) {
        self.a0 = a0
    }

    //pub fn app_init_context(entry: usize, sp: usize) -> Self {
    //    let mut sstatus = Sstatus::read();
    //    let scause = Scause::read();
    //    sstatus.set_spp(SPP::Supervisor);

    //    let mut cx = Self {
    //        x: [0; 32],
    //        sstatus,
    //        sepc: entry,
    //        sbadaddr: 0,
    //        scause,
    //    };
    //    cx.set_sp(sp);
    //    cx
    //}
}
