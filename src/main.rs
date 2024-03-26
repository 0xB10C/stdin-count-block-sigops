use rawtx_rs::bitcoin;
use rawtx_rs::bitcoin::Network;
use rawtx_rs::bitcoin::hex::FromHex;
use rawtx_rs::input::InputSigops;
use rawtx_rs::output::OutputSigops;
use rawtx_rs::tx::TransactionSigops;
use std::io;
use bitcoin_pool_identification::PoolIdentification;
use bitcoin_pool_identification::default_data;

fn deserialize_block(hex: &str) -> bitcoin::Block {
    let block_bytes = Vec::from_hex(hex).unwrap();
    bitcoin::consensus::deserialize(&block_bytes).unwrap()
}

const VERBOSE: bool = true;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.trim().to_string();

    let block = deserialize_block(&buffer);
    let pool = match block.identify_pool(Network::Bitcoin, &default_data(Network::Bitcoin)) {
        Some(result) => result.pool.name,
        None => String::from("Unknown"),  
    };
    let mut block_sigops = 0;
    for tx in block.txdata.iter() {
        if VERBOSE {
            //println!("txid: {}, sigops: {}", tx.txid(), tx.sigops().unwrap());
            //println!("txid: {}, sigopsnew: {}", tx.txid(), tx.sigopsnew().unwrap());
            for input in tx.input.iter() {
                let old = input.sigops().unwrap();
                let new = input.sigopsnew().unwrap();
                //println!("input sigops: old={} new={}", old, new);
                assert_eq!(new, old);
            }
            for output in tx.output.iter() {
                let old = output.sigops().unwrap();
                let new = output.sigopsnew().unwrap();
                // println!("output sigops: old={} new={}", old, new);
                assert_eq!(new, old);
            }
        }
        block_sigops += tx.sigops().unwrap();
    }
    println!("{},{}", block_sigops, pool);
}
