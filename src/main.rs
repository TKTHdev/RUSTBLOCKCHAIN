use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
use std::time;
use std::time::SystemTime;

fn main()
{
    for i in 0..3
    {
        println!("hello world"+i);
    }
}

fn hash<T: Hash>(t: &T) -> u64
{
    let mut s = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

//Transaction Data

struct TransactionData
{
    amount:f64,
    sender_key:String,
    receiver_key:String,
    timestamp:SystemTime::now(),


}


//Block Class

struct Block
{
    index:i32,
    block_hash:usize,
    previous_hash:usize,


    pub data:TransactionData,
}

impl Block
{
        fn generate_hash()->usize
        {
            let mut hasher=DefaultHasher::new();
            let mut hasher2=DefaultHasher::new();
            let mut finalHash=DefaultHasher::new();
            let toHash:string=ToString(data.amount)+data.receiver_key+data.senderkey+ToString(data.timestamp);

            toHash.hash(&mut hasher);
            let firstPart=hasher.finish();
            previous_hash.hash(&mut hasher2);
            let secondPart=hasher2.finish();
            (firstPart+secondPart).hash(&mut finalHash);
            let end=finalHash.finish();
            return end as usize;


        }
        pub fn get_hash()->uszie
        {
            return block_hash;
        }
        pub fn get_previous_hash()->usize
        {
            return previous_hash;
        }

        pub fn is_hash_valid()->bool
        {
            return generate_hash()==block_hash;
        }
        pub fn new(idx:int,d:TransactionData,prevHash:usize)->Block
        {
            Block
            {
                index:idx,
                data:d,
                previous_hash:prevHash,
                block_hash:generate_hash()

            }
        }

}



//Blockchain Class
struct Blockchain
{
    pub chain:Vec<Block>,
}

impl Blockchain
{
    /*constructor (NEED TO BE EDITTED!!!!!)*/
    pub fn new()->Self
    {
        let genesis:Block=createGenesisBlock();
        chain.push(genesis);
    }


    pub fn addBlock(data:TransactionData)->void
    {
        index:i32=chain.size()-1;
        newBlock();

    }


    pub fn isChainValid()->bool{return false}
    pub fn getLatestBlock()->void{return 0}
    fn createGenesisBlock()->Block
    {
        let current:SystemTime::now();
        let mut d=TransactionData
        {
            amount:0.0,
            receiver_key:String::from("None"),
            sender_key:String::from("None"),
            timestamp:time::SystemTime::now(),
        };
        let mut hasher=DefaultHasher::new();
        let mut zero:i32=0;
        let h=hash(&zero);
        let genesis=Block::new(0,d,h as usize);
        return genesis;
        
        assert!();

    }




}