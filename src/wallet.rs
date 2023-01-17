use wagyu_model;
use wallet_gen::prelude::*;

fn new_wallet(coin: Coin) -> Result<Wallet> {
    let group = EcGroup::from_curve_name(Nid::SECP256K1)?;
    let key = EcKey::generate(&group)?;
    let pub_key = {
        let mut bn_ctx = BigNumContext::new()?;
        let mut vec =
            key.public_key()
                .to_bytes(&group, PointConversionForm::UNCOMPRESSED, &mut bn_ctx)?;
        vec.remove(0);
        vec
    };
    let priv_key = key.private_key().to_vec();
    let hash_bytes = keccak256(&pub_key);

    Ok(Wallet {
        coin:        coin,
        address:     format!("0x{:x}", &HexSlice::new(&hash_bytes[12..])),
        public_key:  format!("{:x}", &HexSlice::new(&pub_key)),
        private_key: format!("{:x}", &HexSlice::new(&priv_key[..])),
        other:       None,
    })
}

fn get_private_key(value: &[u8]) -> wagyu_model:: {

}