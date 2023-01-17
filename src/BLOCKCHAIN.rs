pub mod bc
{
    use crate::BLOCK::b::Block;
    use crate::TRANSACTION::t::Transaction;
    use std::time::{SystemTime, UNIX_EPOCH};
    use sha2::{Sha256,Digest};
    /*BLOCKCHAIN CLASS*/
    pub struct Blockchain
    {
        pub  chain:Vec<Block>,
    }
    impl Blockchain
    {
        /*NEEDS TO BE MODIFIED!!!!!!!!!!!!!!!!!!*/
        fn create_genesis_block()->Block
        {   let time=SystemTime::now();
            let mut d=Transaction
            {
                timestamp:time.duration_since(UNIX_EPOCH).unwrap(),
                receiver_key:String::from("None"),
                sender_key:String::from("None"),
                amount:0,
            };
            let mut hasher=Sha256::new();
            hasher.update("0");

            let genesis:Block=Block::new(0,d,hasher);
            return genesis;

        }

        /*CONSTRUCTOR*/
        /*NEEDS TO BE MODIFIED!!!!!!!!!!!*/
        pub fn new(&mut self)
        {
            let genesis:Block=Self::create_genesis_block();
            &self.chain.push(genesis);
        }

        /*NEEDS TO BE MODIFIED!!!!!!!!!!!!!!!!*/
        pub fn add_block(data:Transaction)
        {

        }

        /*NEEDS TO BE MODIFIED!!!!!!!!*/
        pub fn is_chain_valid()->bool
        {
            return false;
        }

        /*NEEDS TO BE MODIFIED!!!!!!!!!!!!!!!!!!!!*/
        pub fn get_latest_block(&mut self)->Block
        {
            let block=self.chain.last();
            return b ;
        }


    }
}