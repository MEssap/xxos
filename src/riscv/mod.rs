use crate::riscv::registers::RegisterOperator;
use xxos_log::{error, info};

pub mod cpu;
pub(crate) mod def;
pub mod registers;
pub mod sv39;

pub fn riscv_test() {
    use registers::{satp::Satp, sstatus::Sstatus};

    let mut satp = Satp::read();
    let mut sstatus = Sstatus::read();

    satp.set(0xdeadbeef);
    sstatus.set(0xdeadbeef);
    satp.write();
    sstatus.write();

    satp.set(0);
    sstatus.set(0);
    satp = Satp::read();
    sstatus = Sstatus::read();

    if satp.bits() != 0xdeadbeef {
        error!("satp wrong");
        panic!();
    } else {
        info!("satp: {:#x}", satp.bits());
    }

    if sstatus.bits() != 0xdeadbeef {
        error!("sstatus wrong");
        panic!();
    } else {
        info!("sstatus: {:#x}", sstatus.bits());
    }
}
