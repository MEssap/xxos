pub mod satp;
pub mod scause;
pub mod sstatus;
pub mod stval;

pub trait RegisterOperator {
    fn read() -> Self;
    fn write(&self);
    fn _clear(&self, bits: usize);
    fn _set(&self, bits: usize);
}
