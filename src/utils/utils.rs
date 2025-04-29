use eyre::Result;

pub struct BatchBlock {
    pub from: u64,
    pub to: u64,
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
