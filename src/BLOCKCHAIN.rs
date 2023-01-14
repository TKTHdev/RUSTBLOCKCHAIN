pub mod bc
{
    use crate::BLOCK::b::Block;
    use crate::TRANSACTION::t::Transaction;
    /*BLOCKCHAIN CLASS*/
    pub struct Blockchain
    {
        pub chain:Vec<Block>,
    }
    impl Blockchain
    {
        /*NEEDS TO BE MODIFIED!!!!!!!!!!!!!!!!!!*/
        fn crate_genesis_block()->Block
        {
            return Block;
        }

        /*CONSTRUCTOR*/
        /*NEEDS TO BE MODIFIED!!!!!!!!!!!*/
        pub fn new()
        {

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
        pub fn get_lastest_block()->Block
        {
            return Block;
        }


    }
}