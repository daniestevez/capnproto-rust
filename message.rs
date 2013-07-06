use std;
use common::*;
use arena::*;
use layout;

pub struct ReaderOptions {
    traversalLimitInWords : u64,
    nestingLimit : uint
}

pub static DEFAULT_READER_OPTIONS : ReaderOptions =
    ReaderOptions { traversalLimitInWords : 8 * 1024 * 1024, nestingLimit : 64 };

pub struct MessageReader<'self> {
    segments : &'self [ &'self [u8]],
    options : ReaderOptions,
//    arena : ReaderArena<'self>
}

type SegmentId = u32;

impl <'self> MessageReader<'self> {

    #[inline(always)]
    pub fn getSegment(&self, id : uint) -> &'self [u8] {
        self.segments[id]
    }

    #[inline(always)]
    pub fn getSegmentReader<'a>(&'a self, id : SegmentId) -> SegmentReader<'a> {
        SegmentReader { messageReader : self, segment : self.getSegment(id as uint) }
    }

    #[inline(always)]
    pub fn getOptions<'a>(&'a self) -> &'a ReaderOptions {
        return &self.options;
    }

    pub fn getRoot<'a>(&'a mut self) -> layout::StructReader<'a> {
        let segment = self.getSegmentReader(0);

        return layout::StructReader::readRoot(0, segment,
                                              self.options.nestingLimit as int);
    }

}

pub enum AllocationStrategy {
    FIXED_SIZE,
    GROW_HEURISTICALLY
}

pub static SUGGESTED_FIRST_SEGMENT_WORDS : uint = 1024;
pub static SUGGESTED_ALLOCATION_STRATEGY : AllocationStrategy = GROW_HEURISTICALLY;

pub struct MessageBuilder {
    nextSize : uint,
    allocationStrategy : AllocationStrategy,
    segments : ~[~SegmentBuilder]
}


impl MessageBuilder {

/*
    pub fn new(firstSegment : ~[u8], allocationStrategy : AllocationStrategy)
        -> @MessageBuilder {
        @MessageBuilder {
            nextSize : firstSegment.len(),
            allocationStrategy : allocationStrategy,
            firstSegment : firstSegment,
            moreSegments : ~[]
        }
    }
*/

    pub fn new_default() -> @mut MessageBuilder {
        let result = @mut MessageBuilder {
            nextSize : SUGGESTED_FIRST_SEGMENT_WORDS,
            allocationStrategy : SUGGESTED_ALLOCATION_STRATEGY,
            segments : ~[]
        };
        result.segments.push(
            ~SegmentBuilder::new(result, SUGGESTED_FIRST_SEGMENT_WORDS * BYTES_PER_WORD));

        result
    }

    pub fn allocateSegment<'a>(&'a mut self, minimumSize : uint) -> &'a [u8] {
        fail!()
    }

}