use blockchainlib::*;

fn main() {
    let difficulty = 0x00ffffffffffffffffffffffffffffff;

    // 1. create first block
    let mut genesis_block =
        Block::new(0, 0, vec![0; 32], 0, "Genesis block".to_owned(), difficulty);

    // 2. validate the created block?
    genesis_block.mine();
    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    // 3. create blockchain
    let mut blockchain = Blockchain {
        blocks: vec![genesis_block],
    };

    println!("Verify: {}", &blockchain.verify());

    // 4. repeat the process
    for i in 1..=10 {
        let block_number = format!("Block {}", i);
        let mut block = Block::new(i, now(), last_hash, 0, block_number.to_owned(), difficulty);

        block.mine();
        println!("Mined new block {:?}", &block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);

        println!("Verify: {}", &blockchain.verify());
    }

    println!("{:#?}", &blockchain);
}
