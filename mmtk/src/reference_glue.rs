use mmtk::vm::ReferenceGlue;
use mmtk::util::ObjectReference;
use mmtk::TraceLocal;
use mmtk::util::OpaquePointer;
use crate::Ruby;

pub struct VMReferenceGlue {}

impl ReferenceGlue<Ruby> for VMReferenceGlue {
    fn set_referent(_reference: ObjectReference, _referent: ObjectReference) {
        unimplemented!()
    }
    fn get_referent(_object: ObjectReference) -> ObjectReference {
        unimplemented!()
    }
    fn process_reference<T: TraceLocal>(_trace: &mut T, _reference: ObjectReference, _tls: OpaquePointer) -> ObjectReference {
        unimplemented!()
    }
}