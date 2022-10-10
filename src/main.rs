use futuresdr::{
    anyhow::Result,
    async_io::block_on,
    async_trait::async_trait,
    runtime::{
        Block, BlockMeta, BlockMetaBuilder, Flowgraph, Kernel, MessageIo, MessageIoBuilder,
        Runtime, StreamIo, StreamIoBuilder, WorkIo,
    },
};

fn main() {
    let mut fg = Flowgraph::new();
    fg.add_block(ExampleBlock.into());
    let (task, mut handle) = block_on(Runtime::new().start(fg));

    // Trying to get drop to be called on the block
    block_on(handle.terminate()).unwrap();

    // This hangs forever
    block_on(task).unwrap();
}

struct ExampleBlock;

impl Into<Block> for ExampleBlock {
    fn into(self) -> Block {
        Block::new(
            BlockMetaBuilder::new("ExampleBlock").build(),
            StreamIoBuilder::new().build(),
            MessageIoBuilder::new().build(),
            self,
        )
    }
}

#[async_trait]
impl Kernel for ExampleBlock {
    async fn work(
        &mut self,
        _io: &mut WorkIo,
        _sio: &mut StreamIo,
        _mio: &mut MessageIo<Self>,
        _meta: &mut BlockMeta,
    ) -> Result<()> {
        Ok(())
    }

    async fn deinit(
        &mut self,
        _s: &mut StreamIo,
        _m: &mut MessageIo<Self>,
        _b: &mut BlockMeta,
    ) -> Result<()> {
        println!("Drop from Kernel");
        Ok(())
    }
}

impl Drop for ExampleBlock {
    fn drop(&mut self) {
        println!("Drop from Drop");
    }
}
