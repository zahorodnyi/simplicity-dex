use simplicityhl::elements;
use simplicityhl::elements::bitcoin::secp256k1;
use simplicityhl::elements::hex::ToHex;
use simplicityhl::elements::pset::{Input, Output, PartiallySignedTransaction};
use simplicityhl::elements::{AddressParams, AssetId, OutPoint, Script, Transaction, TxOut};

use crate::error::DcdManagerError;
use simplicityhl_core::{fetch_utxo, finalize_p2pk_transaction, get_p2pk_address};
use tracing::{debug, info, instrument};

#[instrument(skip_all, level = "debug", err)]
pub fn handle(
    keypair: secp256k1::Keypair,
    fee_utxo: OutPoint,
    parts_to_split: u64,
    mut fee_amount: u64,
    address_params: &'static AddressParams,
    change_asset: AssetId,
    genesis_block_hash: elements::BlockHash,
) -> crate::error::Result<Transaction> {
    let (utxo_tx_out, utxo_outpoint) = (
        fetch_utxo(fee_utxo).map_err(|err| DcdManagerError::Internal(err.to_string()))?,
        fee_utxo,
    );

    let change_recipient = get_p2pk_address(&keypair.x_only_public_key().0, address_params)
        .map_err(|err| DcdManagerError::Internal(err.to_string()))?;
    let total_input_utxo_value: u64 = obtain_utxo_value(&utxo_tx_out)?;

    let mut pst = PartiallySignedTransaction::new_v2();

    let issuance_tx = Input::from_prevout(utxo_outpoint);
    pst.add_input(issuance_tx);

    let split_amount = (total_input_utxo_value - fee_amount) / parts_to_split;
    debug!("Splitting utxo with amount: {total_input_utxo_value} on {split_amount}");
    fee_amount += total_input_utxo_value - fee_amount - split_amount * parts_to_split;

    for _ in 0..parts_to_split {
        let output = Output::new_explicit(change_recipient.script_pubkey(), split_amount, change_asset, None);
        pst.add_output(output);
    }

    // Add fee
    let output = Output::new_explicit(Script::new(), fee_amount, change_asset, None);
    pst.add_output(output);

    let mut tx = pst.extract_tx()?;
    debug!("Formed for sending tx_id: {}", tx.txid().to_hex());

    let utxos = [utxo_tx_out];

    tx = finalize_p2pk_transaction(tx.clone(), &utxos, &keypair, 0, address_params, genesis_block_hash)
        .map_err(|err| DcdManagerError::Internal(err.to_string()))?;

    tx.verify_tx_amt_proofs(secp256k1::SECP256K1, &utxos)?;

    info!("Successfully formed tx_id: {}", tx.txid().to_hex());
    Ok(tx)
}

#[inline]
fn obtain_utxo_value(tx_out: &TxOut) -> crate::error::Result<u64> {
    tx_out
        .value
        .explicit()
        .ok_or_else(|| DcdManagerError::Internal(format!("No value in utxo, check it, tx_out: {tx_out:?}")))
}
