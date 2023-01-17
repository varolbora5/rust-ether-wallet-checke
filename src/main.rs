mod wallet;

use http::Result;
use wallet_gen::{wallet::Wallet, coin::Coin};
use web3::types::{H160, U256};
use std::str::FromStr;
use std::fs::OpenOptions;
use std::io::Write;

#[tokio::main]
async fn main() -> web3::Result<()> {
    // let coin: Coin = Coin::from_symbol("ETH").unwrap();
    // let websocket = web3::transports::WebSocket::new("wss://mainnet.infura.io/ws/v3/aa7135cbc01745d28dab8c1bb52c9b0b").await?;
    // let web3s: web3::Web3<web3::transports::WebSocket> = web3::Web3::new(websocket);

    // loop {
    //     let wallet: Wallet = wallet_gen::ethereum::new_wallet(coin).expect("Wallet couldn't be generated");
    //     let address = H160::from_str(&wallet.address).expect("To H160 failed");
    //     let amount = web3s.eth().balance(address, None).await?;
    //     println!("ETH Balance of {}: {}", &wallet.address, &amount);
    //     if amount > U256::from(0) {
    //         println!("ETH Balance of {}: {}", &wallet.private_key, &amount);
    //         write(wallet.private_key, amount).expect("FUCK");
    //     }
    // }
    Ok(())
}
#[allow(dead_code)]
fn write(key: String, amount: U256) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("wallets.txt")
        .unwrap();

    writeln!(&mut file, "{}", format!("ETH balance of {}: {}", key, amount)).expect("Couldn't write to file");
    Ok(())
}