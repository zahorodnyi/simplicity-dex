mod utils;

mod tests {
    use crate::utils::{DEFAULT_CLIENT_TIMEOUT, DEFAULT_RELAY_LIST, TEST_LOGGER};
    use nostr::{Keys, ToBech32};
    use nostr_relay_connector::relay_client::ClientConfig;
    use nostr_relay_processor::relay_processor::{OrderPlaceEventTags, OrderReplyEventTags, RelayProcessor};
    use std::str::FromStr;
    use std::time::Duration;
    use tracing::{info, instrument};

    #[instrument]
    #[tokio::test]
    async fn test_wss_metadata() -> anyhow::Result<()> {
        let _guard = &*TEST_LOGGER;
        let key_maker = Keys::from_str("nsec13acgqjyjhw4krqalc3dj38mw5ef2rqfnqcrvdh9ltyp3uzskljcsegg0x3")?;
        info!(
            "=== Maker pubkey: {}, privatekey: {}",
            key_maker.public_key.to_bech32()?,
            key_maker.secret_key().to_bech32()?
        );
        let relay_processor_maker = RelayProcessor::try_from_config(
            DEFAULT_RELAY_LIST,
            Some(key_maker.clone()),
            ClientConfig {
                timeout: Duration::from_secs(DEFAULT_CLIENT_TIMEOUT),
            },
        )
        .await?;

        let placed_order_event_id = relay_processor_maker
            .place_order(OrderPlaceEventTags::default())
            .await?;
        info!("=== placed order event id: {}", placed_order_event_id);

        let key_taker = Keys::generate();
        let relay_processor_taker = RelayProcessor::try_from_config(
            DEFAULT_RELAY_LIST,
            Some(key_taker.clone()),
            ClientConfig {
                timeout: Duration::from_secs(DEFAULT_CLIENT_TIMEOUT),
            },
        )
        .await?;
        info!(
            "=== Taker pubkey: {}, privatekey: {}",
            key_taker.public_key.to_bech32()?,
            key_taker.secret_key().to_bech32()?
        );
        let reply_event_id = relay_processor_taker
            .reply_order(
                placed_order_event_id,
                key_maker.public_key,
                OrderReplyEventTags::default(),
            )
            .await?;
        info!("=== order reply event id: {}", reply_event_id);

        let order_replies = relay_processor_maker.get_order_replies(placed_order_event_id).await?;
        info!(
            "=== order replies, amount: {}, orders: {:#?}",
            order_replies.len(),
            order_replies
        );

        let orders_listed = relay_processor_maker.list_orders().await?;
        info!(
            "=== orders listed, amount: {}, orders: {:#?}",
            orders_listed.len(),
            orders_listed
        );

        Ok(())
    }
}
