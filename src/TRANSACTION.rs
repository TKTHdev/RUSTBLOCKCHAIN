pub mod t
{
    use std::time::{SystemTime,UNIX_EPOCH,Duration};
    //TRANSACTION DATA
    pub struct Transaction
    {
        pub amount:u32,
        pub sender_key:String,
        pub receiver_key:String,
        pub timestamp:Duration,
    }
}