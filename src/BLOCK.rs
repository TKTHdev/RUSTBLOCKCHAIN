pub mod b
{
    use crypto::sha2::Sha256;
    use crypto::digest::Digest;
    use crate::TRANSACTION::t::Transaction;

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
        /*NEEDS TO BE MODIFIED!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!*/
        pub fn new(idx:i32,d:Transaction,prev_hash:Sha256)->Self
        {
            return Block;
        }

        /*GET ORIGINAL HASH*/
        /*NEEDS TO BE MODIFIED!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!*/
        pub fn get_hash()->Sha256
        {
            return Sha256::new();
        }

        /*GET PREVIOUS HASH*/
        /*NEEDS TO BE MODIFIED!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!*/
        pub fn get_previous_hash()->Sha256
        {
            return Sha256::new();
        }

        /*CHECK IS THE HASH IS VALID */
        /*NEEDS TO BE MODIFIED!!!!!!!!!!!!!!!!!!*/
        pub fn is_hash_valid()->bool
        {
            return false;
        }





    }
}