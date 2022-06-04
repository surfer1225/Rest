use ethers_core::types::Bytes;
use ethers_core::{
    utils::Ganache,
};
use ethers_providers::{Middleware, Provider, Ws};

async fn send_tx(tx: &str) {
    println!("{}", tx);

    let ganache = Ganache::new().block_time(2u64).spawn();
    let ws = Ws::connect(ganache.ws_endpoint()).await.unwrap();
    let provider = Provider::new(ws);

    let tx_byte = Bytes::from(hex::decode(tx).unwrap());
    let pending_tx = provider.send_raw_transaction(tx_byte).await.unwrap();

    println!("{}", *pending_tx);
}

#[async_std::main]
async fn main() {
    println!("Hello, world!");

    send_tx("0xf8690485012a05f200830c35009424c1a39fbb4c43589545502168c958953d0280b280843c7a3aff1ba0ceec1864a039cf7a47b17330c6658cc520a529b9bf3a41e22fd8a1b6becc47d4a0746d99de88a02bc96a492addf0dd7a4ddf97a90ba0be1c823e6c7690a20a4ea7").await;
}
