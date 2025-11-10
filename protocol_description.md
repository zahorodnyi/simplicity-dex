**Creation**

Based on the provided text, the "Creation" option in the contract is responsible for minting three distinct types of
tokens. These tokens represent the claims of the Maker and Taker on the collateral and settlement assets they have
deposited into the contract. They are used to manage the contract's lifecycle, including early termination and final
settlement.

* _filler_token_asset_id_hex_le_: This is the Taker's token. Takers receive these tokens when they deposit collateral
  into the contract (TakerFundingPath). They can later burn these tokens to either exit the contract early (
  TakerEarlyTermination) or claim their payout at settlement (TakerSettlement).
* _grantor_collateral_token_asset_id_hex_le_: This is the Maker's collateral token. It represents the Maker's claim on
  the collateral they deposited. The Maker can burn these tokens to get their collateral back before settlement (
  MakerCollateralTermination) or as part of the final settlement (MakerSettlement).
* _grantor_settlement_token_asset_id_hex_le_: This is the Maker's settlement token. It represents the Maker's claim on
  the settlement asset they deposited. The Maker can burn these tokens to get the settlement asset back (
  MakerSettlementTermination) or as part of the final settlement (MakerSettlement).

* _Collateral Token_ (grantor_collateral_token_asset_id_hex_le): Represents the Maker's claim on the collateral they
  deposited.
* _Settlement Token_ (grantor_settlement_token_asset_id_hex_le): Represents the Maker's claim on the settlement asset
  they deposited.

## Maker Funding

* This command constructs a transaction that performs several actions:
    * **Spends Reissuance Tokens:** It consumes the three reissuance token UTXOs (_filler-token-utxo_, _grantor-collateral-token-utxo_, _grantor-settlement-token-utxo_) that were created in the DCD Creation step.
    * **Deposits Assets:** It takes the Maker's settlement asset (from settlement-asset-utxo) and collateral (L-BTC, from _fee-utxo_) and locks them into the DCD covenant address.
    * **Issues New Tokens:** Using the spent reissuance tokens, it issues new tokens representing claims on the deposited assets:
        * **Filler Tokens:** A new supply is issued and sent to the covenant address, making them available for Takers.
        * **Grantor Collateral Tokens:** These are sent to the Maker's personal address as a receipt for their deposited collateral.
        * **Grantor Settlement Tokens:** These are also sent to the Maker's personal address as a receipt for their deposited settlement asset.
        * **Manages Change:** Any remaining L-BTC (after depositing collateral and paying fees) and settlement assets are sent back to the Maker's address as change.

In short, this is the step where the Maker locks up their side of the deal and receives receipt tokens in return, while
also minting the "filler" tokens that allow Takers to participate.

**Taker funding**

TakerFundingPath command allows a "Taker" to participate in the Dual Currency Deposit (DCD) contract by depositing
collateral.

* This command constructs a transaction that:
    * **Spends UTXOs:** It spends two main UTXOs:
      A _filler-token-utxo_ that is currently held at the DCD covenant address.
      A _collateral-utxo_ (L-BTC) from the Taker's personal address.
    * **Locks Collateral:** It sends the Taker's deposited collateral (_collateral-amount-to-deposit_) to the DCD covenant address, adding it to the contract's collateral pool.
    * **Issues Receipt Tokens:** It calculates a proportional amount of "filler tokens" based on the deposited collateral and sends them to the Taker's personal address. These tokens act as a receipt for their deposit.
    * **Manages Change and Fees:**
      If the entire filler-token-utxo from the covenant is not consumed, the remainder is sent back to the covenant address as a new _filler-token-utxo_.
      Any remaining L-BTC from the Taker's _collateral-utxo_ (after the deposit and transaction fee) is sent back to the Taker's address as change. 
      A dedicated output is created for the transaction fee.

In essence, this is the step where a Taker enters the contract by locking up collateral and, in return, receives receipt
tokens (filler tokens) that represent their position.

**Taker early termination**

TakerEarlyTermination command allows a "Taker" to exit the Dual Currency Deposit (DCD) contract before its expiry by
returning their filler tokens in exchange for their original collateral.

* This command constructs a transaction that:
    * **Spends UTXOs:** It consumes three UTXOs:
      A collateral-utxo (L-BTC) that is currently held at the DCD covenant address.
      The Taker's _filler-token-utxo_, which they are returning to the contract.
      The Taker's _fee-utxo_ to pay for transaction fees.
    * **Calculates Collateral Return:** It calculates the amount of collateral to be returned to the Taker based on the
      _filler-token-amount-to-return_ and the contract's pre-defined ratio (filler_per_principal_collateral).
    * **Returns Assets:**
      It sends the calculated amount of collateral (L-BTC) back to the Taker's personal address.
      It sends the returned filler tokens back to the DCD covenant address.
    * **Manages Change and Fees:**
      If the _collateral-utxo_ at the covenant is not fully consumed, the remainder is sent back to the covenant address in a new UTXO.
      If the Taker's __filler-token-utxo_ is not fully spent, the change is sent back to the Taker's personal address.
      Any remaining L-BTC from the fee-utxo (after paying the transaction fee) is sent back to the Taker's address as change.
      A dedicated output is created for the transaction fee.

In short, this is the mechanism for a Taker to unwind their position early, receiving their proportional share of
collateral back by "selling" their filler tokens back to the contract.

**Maker collateral termination**

MakerCollateralTermination command allows the "Maker" to withdraw their collateral from the Dual Currency Deposit (DCD)
contract by returning their grantor collateral tokens.

* This command constructs a transaction that:
    * **Spends UTXOs:** It consumes three UTXOs:
      A collateral-utxo (L-BTC) that is currently held at the DCD covenant address.
      The Maker's _grantor-collateral-token-utxo_, which they are returning to the contract.
      The Maker's fee-utxo to pay for transaction fees.
    * **Calculates Collateral Return:** It calculates the amount of collateral to be returned to the Maker. This is based on
      the grantor-collateral-amount-to-burn and the contract's pre-defined ratio of collateral per grantor collateral token.
    * **Returns and Burns Assets:**
      It sends the calculated amount of collateral (L-BTC) back to the Maker's personal address.
      It "burns" the specified amount of grantor collateral tokens by sending them to an OP_RETURN output.
    * **Manages Change and Fees:**
      If the collateral-utxo at the covenant is not fully consumed, the remainder is sent back to the covenant address  in a new UTXO.
      If the Maker's _grantor-collateral-token-utxo_ is not fully spent, the change is sent back to the Maker's personal address.
      Any remaining L-BTC from the _fee-utxo_ (after paying the transaction fee) is sent back to the Maker's address as change.
      A dedicated output is created for the transaction fee.

In essence, this is a withdrawal path for the Maker, allowing them to reclaim their locked collateral by giving up the
corresponding receipt tokens.

**Maker settlement termination**

MakerSettlementTermination command allows the "Maker" to withdraw their settlement asset from the Dual Currency
Deposit (DCD) contract by returning their grantor settlement tokens.

* This command constructs a transaction that:
    * **Spends UTXOs:** It consumes three UTXOs:
      A _settlement-asset-utxo_ that is currently held at the DCD covenant address.
      The Maker's _grantor-settlement-token-utxo_, which they are returning to the contract.
      The Maker's _fee-utxo_ to pay for transaction fees.
    * **Calculates Settlement Asset Return:** It calculates the amount of the settlement asset to be returned to the Maker.
      This is based on the _grantor-settlement-amount-to-burn_ and the contract's pre-defined ratio of settlement asset per grantor settlement token.
    * **Returns and Burns Assets:**
      It sends the calculated amount of the settlement asset back to the Maker's personal address.
      It "burns" the specified amount of grantor settlement tokens by sending them to an OP_RETURN output.
    * **Manages Change and Fees:**
      If the _settlement-asset-utxo_ at the covenant is not fully consumed, the remainder is sent back to the covenant address in a new UTXO.
      If the Maker's _grantor-settlement-token-utxo_ is not fully spent, the change is sent back to the Maker's personal address.
      Any remaining L-BTC from the _fee-utxo_ (after paying the transaction fee) is sent back to the Maker's address as change.
      A dedicated output is created for the transaction fee.

In essence, this is a withdrawal path for the Maker, allowing them to reclaim their locked settlement asset by giving up
the corresponding receipt tokens.

**Maker settlement**

MakerSettlement command allows the "Maker" to settle their position at the contract's maturity, receiving either the
collateral or the settlement asset based on an oracle-provided price.

* This command constructs a transaction that:
    * **Spends UTXOs:** It consumes four UTXOs:
      An asset-utxo (either collateral or settlement asset) held at the DCD covenant address.
      The Maker's _grantor-collateral-token-utxo_.
      The Maker's _grantor-settlement-token-utxo_.
      The Maker's _fee-utxo_ to pay for transaction fees.
      Verifies Oracle Price: It uses an oracle-signature to verify the _price-at-current-block-height_ against the contract's oracle public key. The transaction is also time-locked to the settlement_height.
    * **Calculates Payout based on Price:**
      If _price_ <= _strike_price_: The Maker receives the settlement asset. The amount is calculated based on the _grantor-amount-to-burn_ and the contract's pre-defined ratio.
      If _price_ > _strike_price_: The Maker receives the collateral (L-BTC). The amount is calculated based on the _grantor-amount-to-burn_ and a different pre-defined ratio.
    * **Returns Assets and Burns Tokens:**
      It sends the calculated payout asset (either collateral or settlement asset) to the Maker's personal address.
      It "burns" the specified amount of both grantor collateral and grantor settlement tokens by sending them to an OP_RETURN output.
    * **Manages Change and Fees:**
      If the asset-utxo at the covenant is not fully consumed, the remainder is sent back to the covenant address.
      If the Maker's grantor token UTXOs are not fully spent, the change is sent back to the Maker's address.
      Any remaining L-BTC from the fee-utxo (after paying the transaction fee) is sent back to the Maker's address as change.
      A dedicated output is created for the transaction fee.

In essence, this is the final settlement step for the Maker, where they burn their receipt tokens to claim their final
payout, which is determined by the asset price at maturity

**Taker settlement**

TakerSettlement command allows the "Taker" to settle their position at the contract's maturity, receiving either the
collateral or the settlement asset based on an oracle-provided price.

* This command constructs a transaction that:
    * **Spends UTXOs:** It consumes three UTXOs:
      An _asset-utxo_ (either collateral or settlement asset) held at the DCD covenant address.
      The Taker's _filler-token-utxo_.
      The Taker's _fee-utxo_ to pay for transaction fees.
      Verifies Oracle Price: It uses an oracle-signature to verify the _price-at-current-block-height_ against the contract's oracle public key. The transaction is also time-locked to the _settlement_height_.
    * **Calculates Payout based on Price:**
      If _price_ <= _strike_price_: The Taker receives the collateral (L-BTC). The amount is calculated based on the
      _filler-amount-to-burn_ and the contract's pre-defined ratio (_filler_per_principal_collateral_).
      If _price_ > _strike_price_: The Taker receives the settlement asset. The amount is calculated based on the _filler-amount-to-burn_ and a different pre-defined ratio.
    * **Returns Assets and Burns Tokens:**
      It sends the calculated payout asset (either collateral or settlement asset) to the Taker's personal address.
      It "burns" the specified amount of filler tokens by sending them to an OP_RETURN output.
    * **Manages Change and Fees:**
      If the asset-utxo at the covenant is not fully consumed, the remainder is sent back to the covenant address.
      If the Taker's _filler-token-utxo_ is not fully spent, the change is sent back to the Taker's address.
      Any remaining L-BTC from the _fee-utxo_ (after paying the transaction fee) is sent back to the Taker's address as change.
      A dedicated output is created for the transaction fee.

In short, this is the final settlement step for the Taker, where they burn their filler tokens to claim their final
payout, which is determined by the asset price at maturity.