use self::kvm::LockedKvm;

pub mod def;
pub mod kvm;
pub mod uvm;

pub static KVM: LockedKvm = LockedKvm::new();

pub fn kvm_init() {
    KVM.get_or_init()
}
