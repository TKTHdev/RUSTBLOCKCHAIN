use std::time::{SystemTime, UNIX_EPOCH};
use sha2::digest::block_buffer::Block;
use RustBlockchain::BLOCKCHAIN::bc::Blockchain;
use RustBlockchain::TRANSACTION::t::Transaction;

fn main()
{
    let mut awesomecoin:Blockchain=Blockchain::new();

    let data1:Transaction=Transaction
    {
        amount:15,
        receiver_key:String::from("Joe"),
        sender_key:String::from("Sally"),
        timestamp:SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
    };

    awesomecoin.add_block(data1);

    println!("IS THE CHAIN VALID?");
    println!("{}",awesomecoin.chain.len());
    println!("{}",awesomecoin.is_chain_valid());
    for i in 0..awesomecoin.chain.len()
    {
        println!("{}",awesomecoin.chain[i].display_information());
    }

}