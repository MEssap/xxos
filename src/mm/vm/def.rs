use super::kvm::LockedKvm;

pub static KVM: LockedKvm = LockedKvm::new();
