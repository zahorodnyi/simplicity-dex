use crate::types::{CustomKind, TakerOrderKind};
use nostr::{EventId, Filter, SingleLetterTag};
use nostr_relay_connector::relay_client::RelayClient;
use nostr_sdk::prelude::Events;
use std::collections::{BTreeMap, BTreeSet};

pub async fn handle(client: &RelayClient, event_id: EventId) -> anyhow::Result<Events> {
    let events = client
        .req_and_wait(Filter {
            ids: None,
            authors: None,
            kinds: Some(BTreeSet::from([TakerOrderKind::get_kind()])),
            search: None,
            since: None,
            until: None,
            limit: None,
            generic_tags: BTreeMap::from([(SingleLetterTag::from_char('e')?, BTreeSet::from([event_id.to_string()]))]),
        })
        .await?;
    Ok(events)
}
