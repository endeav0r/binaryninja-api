use binaryninjacore_sys::BNBasicBlock;

pub struct BasicBlockList {
    count: usize,
    blocks: *mut *mut BNBasicBlock,
}

impl Drop for BasicBlockList {
    fn drop(&mut self) {
        unsafe { binaryninjacore_sys::BNFreeBasicBlockList(self.blocks, self.count) }
    }
}

impl BasicBlockList {
    pub(crate) fn new(count: usize, blocks: *mut *mut BNBasicBlock) -> BasicBlockList {
        BasicBlockList { count, blocks }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    unsafe fn blocks_mut(&self) -> *mut *mut BNBasicBlock {
        std::mem::transmute(self.blocks)
    }

    pub fn get(&self, offset: usize) -> Option<BasicBlock> {
        use binaryninjacore_sys::BNNewBasicBlockReference;
        unsafe {
            std::slice::from_raw_parts_mut(*self.blocks_mut(), self.count)
                .get_mut(offset)
                .map(|handle| BasicBlock::from_raw(BNNewBasicBlockReference(handle)))
        }
    }

    pub fn iter(&self) -> BasicBlockListIterator {
        BasicBlockListIterator::new(self)
    }
}

impl<'b> Iterator for BasicBlockListIterator<'b> {
    type Item = BasicBlock;

    fn next(&mut self) -> Option<BasicBlock> {
        self.next()
    }
}

pub struct BasicBlockListIterator<'b> {
    list: &'b BasicBlockList,
    offset: usize,
}

impl<'b> BasicBlockListIterator<'b> {
    pub(crate) fn new(list: &BasicBlockList) -> BasicBlockListIterator<'_> {
        BasicBlockListIterator { list, offset: 0 }
    }

    pub fn next(&mut self) -> Option<BasicBlock> {
        self.offset += 1;
        self.list.get(self.offset - 1)
    }
}

pub struct BasicBlock {
    pub(crate) handle: *mut BNBasicBlock,
}

impl BasicBlock {
    pub(crate) fn from_raw(handle: *mut BNBasicBlock) -> BasicBlock {
        BasicBlock { handle }
    }

    pub fn start(&self) -> u64 {
        unsafe { binaryninjacore_sys::BNGetBasicBlockStart(self.handle) }
    }

    pub fn end(&self) -> u64 {
        unsafe { binaryninjacore_sys::BNGetBasicBlockEnd(self.handle) }
    }

    pub fn len(&self) -> u64 {
        unsafe { binaryninjacore_sys::BNGetBasicBlockLength(self.handle) }
    }
}

impl Drop for BasicBlock {
    fn drop(&mut self) {
        unsafe { binaryninjacore_sys::BNFreeBasicBlock(self.handle) }
    }
}
