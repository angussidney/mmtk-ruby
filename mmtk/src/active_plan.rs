use mmtk::Plan;
use mmtk::vm::ActivePlan;
use mmtk::util::OpaquePointer;
use mmtk::scheduler::*;
use mmtk::Mutator;
use crate::Ruby;
use crate::SINGLETON;

pub struct VMActivePlan<> {}

impl ActivePlan<Ruby> for VMActivePlan {
    fn global() -> &'static dyn Plan<VM=Ruby> {
        &*SINGLETON.plan
    }

    unsafe fn worker(_tls: OpaquePointer) -> &'static mut GCWorker<Ruby> {
        unimplemented!()
    }

    fn number_of_mutators() -> usize {
        unimplemented!()
    }

    unsafe fn is_mutator(_tls: OpaquePointer) -> bool {
        // FIXME
        true
    }

    unsafe fn mutator(_tls: OpaquePointer) -> &'static mut Mutator<Ruby> {
        unimplemented!()
    }

    fn reset_mutator_iterator() {
        unimplemented!()
    }

    fn get_next_mutator() -> Option<&'static mut Mutator<Ruby>> {
        unimplemented!()
    }
}