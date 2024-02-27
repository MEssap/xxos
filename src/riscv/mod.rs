use xxos_log::{error, info};

pub mod cpu;
pub(crate) mod def;
pub mod registers;
pub mod sv39;

pub fn riscv_test() {
    use registers::{satp, sstatus};

    let mut satp = satp::read();
    let mut sstatus = sstatus::read();

    satp.set(0xdeadbeef);
    sstatus.set(0xdeadbeef);
    satp::write(satp.bits());
    sstatus::write(satp.bits());

    satp.set(0);
    sstatus.set(0);
    satp = satp::read();
    sstatus = sstatus::read();

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
