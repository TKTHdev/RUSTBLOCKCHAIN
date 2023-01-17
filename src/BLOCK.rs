pub mod b
{
    use crate::TRANSACTION::t::Transaction;
    use sha2::{Sha256, Digest};


    /*BLOCK CLASS*/
    pub struct Block
    {
        /*PRIVATE VARIABLES*/
        index:i32,
        block_hash:Sha256,
        previous_hash:Sha256,


        /*PUBLIC VARIABLES*/
        data:Transaction,

    }

    impl Block
    {
        /*CONSTRUCTOR*/
        pub fn new(idx:i32,d:Transaction,prev_hash:Sha256)->Self
        {
            Self
            {
                index:idx,
                data:d,
                previous_hash:prev_hash,
                block_hash:Self::generate_hash(&d,&prev_hash),

            }
        }

        fn generate_hash(&trans:&Transaction,&prev_hash:&Sha256) -> Sha256
        {
            let timestamp = trans.timestamp.as_millis() as i32;
            let mut to_hash = trans.amount.to_string() + &trans.receiver_key + &trans.sender_key +&timestamp.to_string() as &str;



            let f2:String=format!("{:X}",prev_hash.finalize());

            return Self::combine_sha256(to_hash, f2);

        }

        fn combine_sha256(data1: String, data2: String) -> Sha256
        {

            let mut hasher=Sha256::new();
            let mut combined_data=data1+&data2;
            hasher.update(&mut combined_data);
            return hasher;


        }

        /*GET ORIGINAL HASH*/
        pub fn get_hash(&self)->&Sha256
        {
            return &self.block_hash;
        }

        /*GET PREVIOUS HASH*/
        pub fn get_previous_hash(&self)->&Sha256
        {
            return &self.previous_hash;
        }

        /*CHECK IS THE HASH IS VALID */
        pub fn is_hash_valid(&self)->bool
        {
            let f1:String=format!("{:X}",Self::generate_hash(&self.data,&self.previous_hash).finalize());
            let f2:String=format!("{:X}",&self.block_hash.finalize());

            return f1==f2;
        }


    }
}