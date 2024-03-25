use rawtx_rs::bitcoin;
use rawtx_rs::bitcoin::hex::FromHex;
use rawtx_rs::input::InputSigops;
use rawtx_rs::output::OutputSigops;
use rawtx_rs::tx::TransactionSigops;
use std::io;

fn deserialize_block(hex: &str) -> bitcoin::Block {
    let block_bytes = Vec::from_hex(hex).unwrap();
    bitcoin::consensus::deserialize(&block_bytes).unwrap()
}

const VERBOSE: bool = false;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.trim().to_string();

    let block = deserialize_block(&buffer);
    let mut block_sigops = 0;
    for tx in block.txdata.iter() {
        if VERBOSE {
            for input in tx.input.iter() {
                println!("input sigops: {}", input.sigops().unwrap());
            }
            for output in tx.output.iter() {
                println!("output sigops: {}", output.sigops().unwrap());
            }
            println!("txid: {}, sigops: {}", tx.txid(), tx.sigops().unwrap());
        }
        block_sigops += tx.sigops().unwrap();
    }
    println!("{}", block_sigops);
}
