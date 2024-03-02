use self::def::KVM;

pub mod def;
pub mod kvm;
pub mod uvm;

pub fn kvm_init() {
    KVM.install_kvm()
}
