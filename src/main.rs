use blockchainlib::*;

fn main() {
    let block_str = String::from("Genesis block");
    let mut block = Block::new(0, 0, vec![0; 32], 0, block_str);

    println!("{:?}", &block);

    let h = block.hash();

    println!("{:?}", &h);

    block.hash = h;

    println!("{:?}", &block);
}
