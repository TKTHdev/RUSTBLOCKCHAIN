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
        pub fn new()->Self
        {
            let genesis:Block=Self::create_genesis_block();
            Self
            {
                chain:vec![genesis]
            }
        }

        /*NEEDS TO BE MODIFIED!!!!!!!!!!!!!!!!*/
        pub fn add_block(&mut self,data:Transaction)
        {
            let chain_length=(self.chain.len()-1) as i32;
            let prev_block=&self.chain[chain_length as usize];
            let hash=prev_block.get_hash();
            let new_block=Block::new(chain_length,data,hash.clone());
            &self.chain.push(new_block);

        }

        /*NEEDS TO BE MODIFIED!!!!!!!!*/
        pub fn is_chain_valid(&self)->bool
        {
            let chain_length = self.chain.len();

            for i in 1..chain_length
            {
                if chain_length<2
                {
                    return true;
                }

                if !self.chain[i].is_hash_valid()
                {
                    return false;
                }


                if chain_length>=2
                {
                    let previous_hash= format!("{:X}", self.chain[i].get_previous_hash().clone().finalize());
                    let now_hash= format!("{:X}", self.chain[i-1].get_hash().clone().finalize());
                    if previous_hash!=now_hash
                    {
                        return false;
                    }
                }
            }

            return true;
        }

        /*NEEDS TO BE MODIFIED!!!!!!!!!!!!!!!!!!!!*/
        pub fn get_latest_block(&mut self)->&Block
        {
            let block=self.chain.last();
            return block.unwrap();
        }


    }
}