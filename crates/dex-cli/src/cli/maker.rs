use crate::common::{DCDCliArguments, DCDCliMakerFundArguments, InitOrderArgs};
use clap::Subcommand;
use simplicity::elements::OutPoint;

#[derive(Debug, Subcommand)]
pub enum MakerCommands {
    #[command(
        about = "Responsible for minting three distinct types of tokens. Initializes Maker offer to Taker, which later has to be funded.",
        long_about = "Responsible for minting three distinct types of tokens. \
        These tokens represent the claims of the Maker and Taker on the collateral and \
        settlement assets they have deposited into the contract (used to manage \
        the contract's lifecycle, including early termination and final settlement)."
    )]
    InitOrder {
        /// Utxos to construct assets on them
        #[arg(long = "fee-utxos", value_delimiter = ',')]
        fee_utxos: Vec<OutPoint>,
        #[command(flatten)]
        init_order_args: InitOrderArgs,
        /// Fee amount
        #[arg(long = "fee-amount", default_value_t = 1500)]
        fee_amount: u64,
        /// Account index to use for change address
        #[arg(long = "account-index", default_value_t = 0)]
        account_index: u32,
        /// When set, broadcast the built transaction via Esplora and print txid
        #[arg(long = "broadcast", default_value_t = true)]
        broadcast: bool,
    },
    #[command(
        about = "Constructs funding transaction, which transfers appropriate users tokens \
        onto contract address. Creates order as Maker on Relays specified [authentication required]",
        alias = "fund"
    )]
    Fund {
        /// Expects only 5 utxos in this order (filler_token, grantor_collateral_token, grantor_settlement_token, settlement_asset, fee_utxo)
        #[arg(long = "fee-utxos", value_delimiter = ',')]
        fee_utxos: Vec<OutPoint>,
        /// Fee amount
        #[arg(long = "fee-amount", default_value_t = 1500)]
        fee_amount: u64,
        /// Storage taproot pubkey gen
        #[arg(long = "taproot-pubkey-gen")]
        dcd_taproot_pubkey_gen: String,
        #[command(flatten)]
        dcd_arguments: Option<DCDCliMakerFundArguments>,
        /// Account index to use for change address
        #[arg(long = "account-index", default_value_t = 0)]
        account_index: u32,
        /// When set, broadcast the built transaction via Esplora and print txid
        #[arg(long = "broadcast", default_value_t = true)]
        broadcast: bool,
    },
    #[command(about = "Allows the Maker to withdraw their collateral from the \
        Dual Currency Deposit (DCD) contract by returning their grantor collateral tokens")]
    TerminationCollateral {
        /// Expects only 5 utxos in this order (filler_token, grantor_collateral_token, grantor_settlement_token, settlement_asset, fee_utxo)
        #[arg(long = "fee-utxos", value_delimiter = ',')]
        fee_utxos: Vec<OutPoint>,
        /// Fee amount
        #[arg(long = "fee-amount", default_value_t = 1500)]
        fee_amount: u64,
        /// Storage taproot pubkey gen
        #[arg(long = "taproot-pubkey-gen")]
        dcd_taproot_pubkey_gen: String,
        /// Fee amount
        #[arg(long = "grantor-coll-burn")]
        grantor_collateral_amount_to_burn: u64,
        #[command(flatten)]
        dcd_arguments: Option<DCDCliArguments>,
        /// Account index to use for change address
        #[arg(long = "account-index", default_value_t = 0)]
        account_index: u32,
        /// When set, broadcast the built transaction via Esplora and print txid
        #[arg(long = "broadcast", default_value_t = true)]
        broadcast: bool,
    },
    #[command(about = "Allows the Maker to withdraw their settlement asset from the \
        Dual Currency Deposit (DCD) contract by returning their grantor settlement tokens")]
    TerminationSettlement {
        /// Expects only 5 utxos in this order (filler_token, grantor_collateral_token, grantor_settlement_token, settlement_asset, fee_utxo)
        #[arg(long = "fee-utxos", value_delimiter = ',')]
        fee_utxos: Vec<OutPoint>,
        /// Fee amount
        #[arg(long = "fee-amount", default_value_t = 1500)]
        fee_amount: u64,
        /// Fee amount
        #[arg(long = "grantor-settl-burn")]
        grantor_settlement_amount_to_burn: u64,
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
    #[command(about = "Allows the Maker to settle their position at the contract's maturity, \
        receiving either the collateral or the settlement asset based on an \
        oracle-provided price")]
    Settlement {
        /// Expects only 5 utxos in this order (filler_token, grantor_collateral_token, grantor_settlement_token, settlement_asset, fee_utxo)
        #[arg(long = "fee-utxos", value_delimiter = ',')]
        fee_utxos: Vec<OutPoint>,
        /// Fee amount
        #[arg(long = "fee-amount", default_value_t = 1500)]
        fee_amount: u64,
        /// price_at_current_block_height
        #[arg(long = "grantor-settl-burn")]
        price_at_current_block_height: u64,
        /// oracle_signature
        #[arg(long = "oracle-sign")]
        oracle_signature: String,
        /// grantor_amount_to_burn
        #[arg(long = "grantor-amount-burn")]
        grantor_amount_to_burn: u64,
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
