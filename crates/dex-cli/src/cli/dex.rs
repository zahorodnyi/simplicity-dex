use clap::Subcommand;
use nostr::EventId;

#[derive(Debug, Subcommand)]
pub enum DexCommands {
    #[command(about = "Fetch replies for a specific order event from Nostr relays [no authentication required]")]
    GetOrderReplies {
        #[arg(short = 'i', long)]
        event_id: EventId,
    },
    #[command(about = "List all currently available orders discovered on Nostr relays [no authentication required]")]
    ListOrders,
    #[command(about = "Fetch an arbitrary Nostr event by its ID [no authentication required]")]
    GetEventsById {
        #[arg(short = 'i', long)]
        event_id: EventId,
    },
    #[command(about = "Fetch a single order by its event ID from Nostr relays [no authentication required]")]
    GetOrderById {
        #[arg(short = 'i', long)]
        event_id: EventId,
    },
}
