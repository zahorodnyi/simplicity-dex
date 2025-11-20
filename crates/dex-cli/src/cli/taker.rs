use crate::common::DCDCliArguments;
use clap::Subcommand;
use simplicity::elements::OutPoint;

#[derive(Debug, Subcommand)]
pub enum TakerCommands {
    #[command(about = "Replies order as Taker on Relays specified [authentication required]")]
    FundOrder {
        /// Expects only 5 utxos in this order (filler_token, grantor_collateral_token, grantor_settlement_token, settlement_asset, fee_utxo)
        #[arg(long = "fee-utxos", value_delimiter = ',')]
        fee_utxos: Vec<OutPoint>,
        /// Fee amount
        #[arg(long = "fee-amount", default_value_t = 1500)]
        fee_amount: u64,
        /// collateral_amount_to_deposit
        #[arg(long = "coll-amount-deposit")]
        collateral_amount_to_deposit: u64,
        /// Storage taproot pubkey gen
        #[arg(long = "taproot-pubkey-gen")]
        dcd_taproot_pubkey_gen: String,
        #[command(flatten)]
        dcd_arguments: Option<DCDCliArguments>,
        /// Account index to use for change address
        #[arg(long = "account-index", default_value_t = 0)]
        account_index: u32,
        /// When set, broadcast the built transaction via Esplora and print txid
        #[arg(long = "broadcast", default_value_t = true)]
        broadcast: bool,
        // #[arg(short = 'i', long)]
        // maker_event_id: EventId,
        // #[arg(short = 'p', long, help = " Pubkey in bech32 or hex format")]
        // maker_pubkey: PublicKey,
        // #[arg(short = 't', long, help = "Txid from funding transaction step", required = false)]
        // tx_id: String,
    },
    #[command(
        about = "Allows a Taker to exit the Dual Currency Deposit (DCD) contract before its expiry \
            by returning their filler tokens in exchange for their original collateral."
    )]
    TerminationEarly {
        /// Expects only 5 utxos in this order (filler_token, grantor_collateral_token, grantor_settlement_token, settlement_asset, fee_utxo)
        #[arg(long = "fee-utxos", value_delimiter = ',')]
        fee_utxos: Vec<OutPoint>,
        /// Fee amount
        #[arg(long = "fee-amount", default_value_t = 1500)]
        fee_amount: u64,
        /// collateral_amount_to_deposit
        #[arg(long = "filler-to-return")]
        filler_token_amount_to_return: u64,
        /// Storage taproot pubkey gen
        #[arg(long = "taproot-pubkey-gen")]
        dcd_taproot_pubkey_gen: String,
        #[command(flatten)]
        dcd_arguments: Option<DCDCliArguments>,
        /// Account index to use for change address
        #[arg(long = "account-index", default_value_t = 0)]
        account_index: u32,
        /// When set, broadcast the built transaction via Esplora and print txid
        #[arg(long = "broadcast", default_value_t = true)]
        broadcast: bool,
    },
    #[command(about = "Allows the Taker to settle their position at the contract's maturity, \
        receiving either the collateral or the settlement asset based on an oracle-provided price")]
    Settlement {
        /// Expects only 5 utxos in this order (filler_token, grantor_collateral_token, grantor_settlement_token, settlement_asset, fee_utxo)
        #[arg(long = "fee-utxos", value_delimiter = ',')]
        fee_utxos: Vec<OutPoint>,
        /// Fee amount
        #[arg(long = "fee-amount", default_value_t = 1500)]
        fee_amount: u64,
        /// filler_amount_to_burn
        #[arg(long = "filler-to-burn")]
        filler_amount_to_burn: u64,
        /// price_at_current_block_height
        #[arg(long = "price-now")]
        price_at_current_block_height: u64,
        /// Oracle signature
        #[arg(long = "oracle-sign")]
        oracle_signature: String,
        /// Storage taproot pubkey gen
        #[arg(long = "taproot-pubkey-gen")]
        dcd_taproot_pubkey_gen: String,
        #[command(flatten)]
        dcd_arguments: Option<DCDCliArguments>,
        /// Account index to use for change address
        #[arg(long = "account-index", default_value_t = 0)]
        account_index: u32,
        /// When set, broadcast the built transaction via Esplora and print txid
        #[arg(long = "broadcast", default_value_t = true)]
        broadcast: bool,
    },
}
