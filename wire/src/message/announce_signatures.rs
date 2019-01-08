use super::ChannelId;
use super::ShortChannelId;
use super::types::Signature;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct AnnounceSignatures {
    channel_id: ChannelId,
    short_channel_id: ShortChannelId,
    node_signature: Signature,
    bitcoin_signature: Signature,
}
