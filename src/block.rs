pub mod b
{
    use std::ops::ShrAssign;
    use crate::transaction::t::Transaction;
    use sha2::{Sha256, Digest};
    use std::time::{Duration};
    use rand::Rng;


    /*BLOCK CLASS*/
    pub struct Block
    {
        /*PRIVATE VARIABLES*/
        index:i32,
        block_hash:Sha256,
        previous_hash:Sha256,


        /*PUBLIC VARIABLES*/
        data:Transaction,
        nonce:u32

    }

    impl Block
    {
        /*CONSTRUCTOR*/
        pub fn new(idx:i32,d:Transaction,prev_hash:Sha256)->Self
        {
            let (nonce,hash)=Self::generate_hash(&d,&prev_hash);
            Self
            {
                index:idx,
                data:d,
                previous_hash:prev_hash,
                block_hash:hash,
                nonce:nonce,

            }
        }

        fn generate_hash(trans:&Transaction,prev_hash:&Sha256) -> (u32,Sha256)
        {
            let timestamp = trans.timestamp.as_millis() as i32;
            let  to_hash = trans.amount.to_string() + &trans.receiver_key + &trans.sender_key +&timestamp.to_string() as &str;



            let f2:String=format!("{:X}",prev_hash.clone().finalize());

            return Self::proof_of_work(to_hash, f2);

        }

        fn proof_of_work(data1: String, data2: String) ->(u32,Sha256)
        {

            let mut hasher=Sha256::new();
            let mut combined_data=data1+&data2;
            hasher.update(&mut combined_data);
            let mut rng=rand::thread_rng();
            let mut  count=0;
            loop
            {
                let mut hasher2=Sha256::new();
                let mut nonce=rng.gen::<u32>();
                let mut combined_data2=combined_data.clone()+&nonce.to_string() as &str;
                hasher2.update(&mut combined_data2);
                let f2: String = format!("{:X}", hasher2.clone().finalize());
                let c = f2.as_bytes()[0] as char;
                let c2 = f2.as_bytes()[1] as char;
                let c3=f2.as_bytes()[2] as char;
                let c4=f2.as_bytes()[3] as char;
                //let c5=f2.as_bytes()[4] as char;
                if c == '0' && c2 == '0'&&c3 == '0' && c4 == '0' //&&c5 == '0'
                {
                    println!("");
                    println!("");
                    println!("");
                    println!("");
                    println!("Correct!!  nonce:{}  hash{}  {}th challange", nonce, f2,count+1);
                    println!("");
                    println!("");
                    return (nonce,hasher2);
                }
                else
                {
                    count+=1;
                    println!("nonce:{}   hash:{}   {}th challange", nonce,f2,count);

                }
            }


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

        pub fn get_nonce(&self) -> u32
        {
            return self.nonce;
        }

        /*CHECK IS THE HASH IS VALID */
        pub fn is_hash_valid(&self)->bool
        {

            let f1:String=format!("{:X}",Self::generate_hash(&self.data,&self.previous_hash).1.finalize());
            let f2:String=format!("{:X}",&self.block_hash.clone().finalize());

            return f1==f2;
        }

        pub fn display_hash(&self) -> String
        {
            return format!("{:X}",&self.block_hash.clone().finalize());
        }

        pub fn display_receiver_key(&self) -> String
        {
            return self.data.receiver_key.clone();
        }
        pub fn display_sender_key(&self) -> String
        {
            return self.data.sender_key.clone();
        }

        pub fn display_value(&self)->u32
        {
            return self.data.amount.clone();
        }

        pub fn display_timestamp(&self)->Duration
        {
            return self.data.timestamp.clone();
        }

    }
}