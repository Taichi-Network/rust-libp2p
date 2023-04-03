use std::{collections::HashMap, sync::Mutex};

use instant::Duration;
use libp2p_core::PeerId;

use crate::{time_cache::DuplicateCache, TopicHash};

#[derive(Default)]
pub struct TaichiMetrics {
    // Peers who have sent us a message for a topic we are subscribed to.
    topic_recv_peers: Mutex<HashMap<TopicHash, DuplicateCache<PeerId>>>,
}

impl TaichiMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_topic_recv_peer(&self, topic: &TopicHash, peer: &PeerId) {
        let mut topic_recv_peers = self.topic_recv_peers.lock().unwrap();

        // get entry
        let entry = topic_recv_peers
            .entry(topic.clone())
            .or_insert_with(|| DuplicateCache::new(Duration::from_secs(60 * 5)));

        entry.insert_with_update(peer.clone());
    }

    pub fn get_topic_recv_peers_count(&self, topic: &TopicHash) -> Option<usize> {
        let topic_recv_peers = self.topic_recv_peers.lock().unwrap();
        topic_recv_peers
            .get(topic)
            .map(|cache| cache.get_key_count())
    }
}
