use crate::common::keys::derive_secret_key_from_index;
use crate::common::settings::Settings;
use elements::bitcoin::{XOnlyPublicKey, secp256k1};
use simplicityhl::elements::{Address, AddressParams};
use simplicityhl_core::get_p2pk_address;

pub fn handle(index: u32) -> crate::error::Result<(XOnlyPublicKey, Address)> {
    let settings = Settings::load()?;
    let keypair =
        secp256k1::Keypair::from_secret_key(secp256k1::SECP256K1, &derive_secret_key_from_index(index, settings));
    let public_key = keypair.x_only_public_key().0;
    let address = get_p2pk_address(&public_key, &AddressParams::LIQUID_TESTNET)
        .map_err(|err| crate::error::CliError::P2pkAddress(err.to_string()))?;
    Ok((public_key, address))
}
