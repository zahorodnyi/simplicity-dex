use crate::types::{CustomKind, MakerOrderKind};
use nostr::{Filter, NostrSigner};
use nostr_relay_connector::relay_client::RelayClient;
use nostr_sdk::prelude::Events;
use std::collections::{BTreeMap, BTreeSet};

pub async fn handle(client: &RelayClient) -> anyhow::Result<Events> {
    let events = client
        .req_and_wait(Filter {
            ids: None,
            authors: Some(BTreeSet::from([client.get_signer().await?.get_public_key().await?])),
            kinds: Some(BTreeSet::from([MakerOrderKind::get_kind()])),
            search: None,
            since: None,
            until: None,
            limit: None,
            generic_tags: BTreeMap::default(),
        })
        .await?;
    Ok(events)
}
