pub fn make_wallet(value: u128) -> Result<wallet_gen::wallet::Wallet, dyn std::error::Error> {
    let mut array:[u8; 32] = [0; 32];
    let bytes = value.to_le_bytes();

    for i in 0..bytes.len() {
        array[i] = bytes[i];
    }
    todo!()
}
