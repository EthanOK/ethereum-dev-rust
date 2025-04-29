mod common;
use eyre::Result;

pub struct BatchBlock {
    pub from: u64,
    pub to: u64,
}

#[tokio::test]
async fn test_batch_block() -> Result<()> {
    let from_block = 1000_u64;
    let to_block = 1571_u64;

    let batch_size = 100_u64;
    let batch_blocks = get_batch_blocks(from_block, to_block, batch_size)?;

    for batch_block in batch_blocks {
        println!("from:{} to:{}", batch_block.from, batch_block.to);
    }

    Ok(())
}

pub fn get_batch_blocks(
    from_block: u64,
    to_block: u64,
    batch_size: u64,
) -> Result<Vec<BatchBlock>> {
    let mut batch_blocks: Vec<BatchBlock> = vec![];
    let mut current_block = from_block;
    while current_block <= to_block {
        let next_block = current_block + batch_size;
        let to = if next_block > to_block { to_block } else { next_block - 1 };
        batch_blocks.push(BatchBlock { from: current_block, to });
        current_block = to + 1;
    }
    Ok(batch_blocks)
}
