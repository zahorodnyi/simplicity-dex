use simplicityhl::elements;
use simplicityhl::elements::bitcoin::secp256k1;
use simplicityhl::elements::{AddressParams, AssetId, OutPoint, Transaction};

pub struct DcdManager;

impl DcdManager {
    pub fn faucet() -> crate::error::Result<()> {
        crate::manager::handlers::faucet::handle()?;
        Ok(())
    }
    pub fn maker_init() -> crate::error::Result<()> {
        crate::manager::handlers::maker_init::handle()?;
        Ok(())
    }
    pub fn maker_funding() -> crate::error::Result<()> {
        crate::manager::handlers::maker_funding::handle()?;
        Ok(())
    }
    pub fn taker_funding() -> crate::error::Result<()> {
        crate::manager::handlers::taker_funding::handle()?;
        Ok(())
    }
    pub fn taker_early_termination() -> crate::error::Result<()> {
        crate::manager::handlers::taker_termination_early::handle()?;
        Ok(())
    }
    pub fn maker_collateral_termination() -> crate::error::Result<()> {
        crate::manager::handlers::maker_termination_collateral::handle()?;
        Ok(())
    }
    pub fn maker_settlement_termination() -> crate::error::Result<()> {
        crate::manager::handlers::maker_termination_settlement::handle()?;
        Ok(())
    }
    pub fn maker_settlement() -> crate::error::Result<()> {
        crate::manager::handlers::maker_settlement::handle()?;
        Ok(())
    }
    pub fn taker_settlement() -> crate::error::Result<()> {
        crate::manager::handlers::taker_settlement::handle()?;
        Ok(())
    }
    pub fn split_utxo_native(
        keypair: secp256k1::Keypair,
        fee_utxo: OutPoint,
        parts_to_split: u64,
        fee_amount: u64,
        address_params: &'static AddressParams,
        change_asset: AssetId,
        genesis_block_hash: elements::BlockHash,
    ) -> crate::error::Result<Transaction> {
        crate::manager::handlers::split_utxo::handle(
            keypair,
            fee_utxo,
            parts_to_split,
            fee_amount,
            address_params,
            change_asset,
            genesis_block_hash,
        )
    }
}
