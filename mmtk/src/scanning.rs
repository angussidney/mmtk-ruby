use mmtk::vm::Scanning;
use mmtk::{TransitiveClosure, SelectedPlan, Mutator};
use mmtk::util::{ObjectReference, OpaquePointer, Address};
use mmtk::scheduler::gc_works::*;
use mmtk::scheduler::GCWorker;
use std::os::raw::{c_void, c_ulong};
use crate::Ruby;

/* automatically generated by rust-bindgen 0.57.0 */
#[allow(non_camel_case_types)]
pub type size_t = c_ulong;

extern "C" {
    // Functions exposed by Ruby for introspection into the VM
    pub fn rb_mmtk_referent_objects(
        object: *mut c_void,
        closure: *mut c_void,
		callback: *mut c_void // This is a hack to pass the function pointer. Ideally, we would do something
							  // like this, but it is not valid syntax. Additionally, we can't move the type
							  // parameter to above, because C functions cannot have type parameters
							  // unsafe extern "C" fn<T: TransitiveClosure>(closure: &mut T, adjacent: *mut *mut c_void)
    );

    pub fn rb_mmtk_roots(
        callback: unsafe extern "C" fn(root: *mut *mut c_void),
    );

    pub fn rb_mmtk_stacks(
        callback: unsafe extern "C" fn(stack: *mut c_void, size: size_t),
    );
}

// Passed to C to perform the transitive closure
pub unsafe extern "C" fn call_process_edge<T: TransitiveClosure>(closure: &mut T, adjacent: *mut *mut c_void) {
    closure.process_edge(Address::from_ptr(adjacent));
}

pub struct VMScanning {}

impl Scanning<Ruby> for VMScanning {
    fn scan_objects<W: ProcessEdgesWork<VM=Ruby>>(_objects: &[ObjectReference], _worker: &mut GCWorker<Ruby>) {
        unimplemented!()
    }

    fn scan_thread_roots<W: ProcessEdgesWork<VM=Ruby>>() {
        unimplemented!()
    }

    fn scan_thread_root<W: ProcessEdgesWork<VM=Ruby>>(_mutator: &'static mut Mutator<SelectedPlan<Ruby>>, _tls: OpaquePointer) {
        unimplemented!()
    }

    fn scan_vm_specific_roots<W: ProcessEdgesWork<VM=Ruby>>() {
        unimplemented!()
    }

    fn scan_object<T: TransitiveClosure>(_trace: &mut T, _object: ObjectReference, _tls: OpaquePointer) {
        unsafe {
            rb_mmtk_referent_objects(
				_object.to_address().to_mut_ptr(),
				_trace as *mut T as *mut c_void,
				call_process_edge::<T> as *mut c_void
			);
        }
    }

    fn notify_initial_thread_scan_complete(_partial_scan: bool, _tls: OpaquePointer) {
        unimplemented!()
    }

    fn supports_return_barrier() -> bool {
        false
    }
}