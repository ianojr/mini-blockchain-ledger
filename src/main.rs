#![deny(clippy::all)]


#[derive(Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

#[derive(Debug)]
struct Block {
    transaction: Vec<Transaction>,
    previous_hash: String,
}

#[derive(Debug)]
struct Blockchain{
    blocks: Vec<Block>,
}

impl Blockchain{
    fn new() -> Self {
        Blockchain { blocks: Vec::new() }
    }

    fn add_block(&mut self, transaction: Vec<Transaction>, previous_hash: String) {
        let block = Block {
            transaction,
            previous_hash,
        };
        self.blocks.push(block);
    }
}

fn main() {
    let mut ledger = Blockchain::new();

    let transaction1 = Transaction {
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
        amount: 50.0,
    };

    let transaction2 = Transaction {
        sender: String::from("Bob"),
        receiver: String::from("Charlie"),
        amount: 25.0,
    };

    ledger.add_block(vec![transaction1], String::from("0"));
    ledger.add_block(vec![transaction2], String::from("hash_of_previous_block"));

    println!("{:?}", ledger);
}