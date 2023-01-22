use std::time::{SystemTime, UNIX_EPOCH};
use RustBlockchain::blockchain::bc::Blockchain;
use RustBlockchain::transaction::t::Transaction;

fn main()
{
    let mut awesomecoin:Blockchain=Blockchain::new();
    loop
    {
        println!("(1)Add block to the blockchain");
        println!("(2)Display the whole blockchain");
        println!("(3)Quit");
        println!("Enter:");
        let mut a=String::new();
        std::io::stdin().read_line(&mut a).ok();
        let a:i32=a.trim().parse().ok().unwrap();

        if a==1
        {
            add_block(&mut awesomecoin);
        }

        if a==2
        {
            awesomecoin.display_blockchain();
        }

        if a==3
        {
            break;
        }
        if a!=1 && a!=2 && a!=3
        {
            continue;
        }

    }


}

pub fn add_block(bc:&mut Blockchain)
{


    println!("Enter the name of the sender");
    let mut s=String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.to_owned();


    println!("Enter the name of the receiver");
    let mut r=String::new();
    std::io::stdin().read_line(&mut r).unwrap();
    r.to_owned();

    println!("Enter the amount of the transaction");
    let mut a=String::new();
    std::io::stdin().read_line(&mut a).ok();
    let a=a.trim().parse().ok().unwrap();


    println!("{},{},{}",s,r,a);


    let data:Transaction=Transaction
    {
        amount:a,
        receiver_key:r,
        sender_key:s,
        timestamp:SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
    };

    bc.add_block(data);


}