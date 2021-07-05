use super::BasicBlockList;
use binaryninjacore_sys::BNHighLevelILFunction;

pub struct Function {
    pub(crate) handle: *mut BNHighLevelILFunction,
}

impl Drop for Function {
    fn drop(&mut self) {
        unsafe { binaryninjacore_sys::BNFreeHighLevelILFunction(self.handle) }
    }
}

impl Function {
    pub(crate) unsafe fn from_raw(handle: *mut BNHighLevelILFunction) -> Function {
        Function { handle }
    }

    pub fn basic_blocks(&mut self) -> BasicBlockList {
        use binaryninjacore_sys::{BNBasicBlock, BNGetHighLevelILBasicBlockList};
        let mut count: usize = 0;

        let blocks: *mut *mut BNBasicBlock =
            unsafe { BNGetHighLevelILBasicBlockList(self.handle, &mut count) };

        BasicBlockList::new(count, blocks)
    }

    pub fn get_current_address(&mut self) -> u64 {
        unsafe { binaryninjacore_sys::BNHighLevelILGetCurrentAddress(self.handle) }
    }
}

#[test]
fn hlil_function() {
    use crate::binaryview::BinaryViewExt;

    crate::headless::init();

    let bv = super::test::get_test_binary_0();
    
    let functions = bv.functions_at(0x1060);

    if functions.len() != 1 {
        panic!("Failed to get main function at 0x1060");
    }
    let function = functions.get(0);

    let _hlil_function = function
        .high_level_il()
        .expect("Failed to get high level il function");
}
