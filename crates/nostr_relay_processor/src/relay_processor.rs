use crate::handlers;
use nostr::prelude::IntoNostrSigner;
use nostr::{EventId, PublicKey, TryIntoUrl};
use nostr_relay_connector::relay_client::{ClientConfig, RelayClient};
use nostr_sdk::prelude::Events;

pub struct RelayProcessor {
    relay_client: RelayClient,
}

#[derive(Debug, Default, Clone)]
pub struct OrderPlaceEventTags {
    pub asset_to_sell: String,
    pub asset_to_buy: String,
    pub price: u64,
    pub expiry: u64,
    pub compiler_name: String,
    pub compiler_build_hash: String,
}

#[derive(Debug, Default, Clone)]
pub struct OrderReplyEventTags {
    pub tx_id: String,
}

impl RelayProcessor {
    pub async fn try_from_config(
        relay_urls: impl IntoIterator<Item = impl TryIntoUrl>,
        keys: Option<impl IntoNostrSigner>,
        client_config: ClientConfig,
    ) -> crate::error::Result<Self> {
        Ok(RelayProcessor {
            relay_client: RelayClient::connect(relay_urls, keys, client_config).await?,
        })
    }

    pub async fn place_order(&self, tags: OrderPlaceEventTags) -> anyhow::Result<EventId> {
        let event_id = handlers::place_order::handle(&self.relay_client, tags).await?;
        Ok(event_id)
    }

    pub async fn list_orders(&self) -> anyhow::Result<Events> {
        let events = handlers::list_orders::handle(&self.relay_client).await?;
        Ok(events)
    }

    pub async fn reply_order(
        &self,
        reply_to_event_id: EventId,
        reply_to_pubkey: PublicKey,
        tags: OrderReplyEventTags,
    ) -> anyhow::Result<EventId> {
        let event_id =
            handlers::reply_order::handle(&self.relay_client, reply_to_event_id, reply_to_pubkey, tags).await?;
        Ok(event_id)
    }

    pub async fn get_order_replies(&self, event_id: EventId) -> anyhow::Result<Events> {
        let events = handlers::order_replies::handle(&self.relay_client, event_id).await?;
        Ok(events)
    }
}
