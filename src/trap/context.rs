#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct TrapContext {
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
    //sepc: usize,//可能会发生循环嵌套
    //sbadaddr: usize,
    //scause: usize,
    //scratch: usize,
}

impl TrapContext {
    pub fn new() -> Self {
        Self {
            ra: 0,
            sp: 0,
            gp: 0,
            tp: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            s0: 0,
            s1: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            //sepc: 0,
            //sbadaddr: 0,
            //scause: 0,
            //scratch: 0,
        }
    }

    pub fn store_in_stack() {}

    pub fn set_sp(&mut self, sp: usize) {
        self.sp = sp;
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
