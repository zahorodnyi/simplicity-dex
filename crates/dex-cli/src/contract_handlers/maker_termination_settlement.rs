use crate::common::keys::derive_secret_key_from_index;
use crate::common::settings::Settings;
use crate::common::store::{SledError, Store};
use crate::common::{DCDCliArguments, broadcast_tx_inner, vec_to_arr};
use elements::bitcoin::hex::DisplayHex;
use elements::bitcoin::secp256k1;
use simplicity::elements::OutPoint;
use simplicity::elements::pset::serialize::Serialize;
use simplicity_contracts::DCDArguments;
use simplicity_contracts_adapter::dcd::{
    BaseContractContext, CommonContext, DcdContractContext, DcdManager, MakerTerminationSettlementContext,
};
use simplicityhl::elements::{AddressParams, Txid};
use simplicityhl_core::{LIQUID_TESTNET_BITCOIN_ASSET, LIQUID_TESTNET_GENESIS, TaprootPubkeyGen};
use tracing::instrument;

#[derive(Debug)]
pub struct ProcessedArgs {
    keypair: secp256k1::Keypair,
    dcd_arguments: DCDArguments,
    dcd_taproot_pubkey_gen: String,
    fee_utxo: OutPoint,
    settlement_asset_utxo: OutPoint,
    grantor_settlement_token_utxo: OutPoint,
    grantor_settlement_amount_to_burn: u64,
}

#[instrument(level = "debug", skip_all, err)]
pub fn process_args(
    account_index: u32,
    dcd_init_params: Option<DCDCliArguments>,
    dcd_taproot_pubkey_gen: impl AsRef<str>,
    fee_utxos: Vec<OutPoint>,
    grantor_settlement_amount_to_burn: u64,
) -> crate::error::Result<ProcessedArgs> {
    const FEE_UTXOS_NEEDED: usize = 3;

    let store = Store::load()?;

    let settings = Settings::load().map_err(|err| crate::error::CliError::EnvNotSet(err.to_string()))?;

    let keypair = secp256k1::Keypair::from_secret_key(
        secp256k1::SECP256K1,
        &derive_secret_key_from_index(account_index, settings.clone()),
    );

    let fee_utxos = vec_to_arr::<FEE_UTXOS_NEEDED, OutPoint>(fee_utxos)?;

    let dcd_arguments: DCDArguments = match dcd_init_params {
        None => {
            todo!()
        }
        Some(x) => x.convert_to_dcd_arguments()?,
    };

    Ok(ProcessedArgs {
        keypair,
        dcd_arguments,
        dcd_taproot_pubkey_gen: dcd_taproot_pubkey_gen.as_ref().to_string(),
        grantor_settlement_token_utxo: fee_utxos[0],
        settlement_asset_utxo: fee_utxos[1],
        fee_utxo: fee_utxos[2],
        grantor_settlement_amount_to_burn,
    })
}

#[instrument(level = "debug", skip_all, err)]
pub fn handle(
    ProcessedArgs {
        keypair,
        dcd_arguments,
        dcd_taproot_pubkey_gen,
        fee_utxo,
        settlement_asset_utxo,
        grantor_settlement_token_utxo,
        grantor_settlement_amount_to_burn,
    }: ProcessedArgs,
    fee_amount: u64,
    broadcast: bool,
) -> crate::error::Result<Txid> {
    tracing::debug!("=== dcd arguments: {:?}", dcd_arguments);
    let base_contract_context = BaseContractContext {
        address_params: &AddressParams::LIQUID_TESTNET,
        lbtc_asset: LIQUID_TESTNET_BITCOIN_ASSET,
        genesis_block_hash: *LIQUID_TESTNET_GENESIS,
    };
    let dcd_taproot_pubkey_gen = TaprootPubkeyGen::build_from_str(
        &dcd_taproot_pubkey_gen,
        &dcd_arguments,
        base_contract_context.address_params,
        &simplicity_contracts::get_dcd_address,
    )
    .map_err(|e| SledError::TapRootGen(e.to_string()))?;

    let transaction = DcdManager::maker_settlement_termination(
        &CommonContext { keypair },
        MakerTerminationSettlementContext {
            settlement_asset_utxo,
            grantor_settlement_token_utxo,
            fee_utxo,
            fee_amount,
            grantor_settlement_amount_to_burn,
        },
        &DcdContractContext {
            dcd_taproot_pubkey_gen,
            dcd_arguments,
            base_contract_context,
        },
    )
    .map_err(|err| crate::error::CliError::DcdManager(err.to_string()))?;

    match broadcast {
        true => println!("Broadcasted txid: {}", broadcast_tx_inner(&transaction)?),
        false => println!("{}", transaction.serialize().to_lower_hex_string()),
    }

    Ok(transaction.txid())
}

pub fn save_args_to_cache() -> crate::error::Result<()> {
    let store = Store::load()?;
    //todo: move store to cli function
    Ok(())
}
