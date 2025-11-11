use anyhow::anyhow;
use elements::bitcoin::secp256k1;
use elements::secp256k1_zkp::SecretKey;
use simplicity::elements::TxOut;

#[inline]
pub fn obtain_utxo_value(tx_out: &TxOut) -> anyhow::Result<u64> {
    tx_out
        .value
        .explicit()
        .ok_or_else(|| anyhow!("No value in utxo, check it, tx_out: {tx_out:?}"))
}

pub fn derive_public_blinder_key() -> anyhow::Result<secp256k1::Keypair> {
    let blinder_key = secp256k1::Keypair::from_secret_key(secp256k1::SECP256K1, &SecretKey::from_slice(&[1; 32])?);
    Ok(blinder_key)
}
