use binaryninjacore_sys::BNHighLevelILFunction;

pub struct Function {
    pub(crate) handle: *mut BNHighLevelILFunction,
}
