extern crate chain;
extern crate keys;
extern crate serialization;

use chain::bytes::Bytes;
use chain::constants::SATOSHIS_IN_COIN;
use chain::compact::Compact;
use chain::{
    Block, BlockHeader,
    Transaction, TransactionInput,
    TransactionOutput };
use keys::generator::{ Random, Generator };
use keys::{ KeyPair, Network };
use serialization::serialize;

fn main() {
    println!("Initializing the master KeyPair...");
    let keys = init_keypair();

    println!("{}", keys);
    println!("{}", keys.address());

    create_genesis_block();
}

fn create_genesis_block() {
    println!("Generating genesis block...");
    let gen_tx: Transaction = init_tranasaction();

    let gen_header: BlockHeader = BlockHeader {
        version: 1,
        previous_header_hash: [0;32].into(),
        merkle_root_hash: [0;32].into(),
        time: 3131313131,
        bits: Compact::from(486604799),
        nonce: 2083236893,
    };

    let gen_block: Block = Block::new(gen_header, vec![gen_tx]);
    println!("{:?}", serialize(&gen_block));
}

/// The inception transaction of the genesis block.
fn init_tranasaction() -> Transaction {

    let psz_timestamp = b"Bloomberg 25/Sep/2017 Trump Plan Cuts Taxes for Rich";
    let init_value: u64 = 50 * SATOSHIS_IN_COIN;

    if psz_timestamp.len() > 254 {
        panic!("Timestamp is too long!")
    }

    let b = Bytes::from(psz_timestamp.to_vec());

    Transaction {
        version: 1,
        inputs: vec![TransactionInput::coinbase(b)],
        outputs: vec![TransactionOutput {
            value: init_value,
            script_pubkey: Bytes::from("0482e1d44a91b9ee4bb7c8e0b154521a4748f0ff2be2a75919f7e72c2bb845cc0ee48830fee8c34795f5f2848922ec7ee29d5eaf0dceead6ace451138b90435a4b"),
        }],
        lock_time: 0,
    }
}

fn init_keypair() -> KeyPair {
    let rand = Random::new(Network::Elcoin);
    let rand_keys = rand.generate().unwrap();
    rand_keys
}