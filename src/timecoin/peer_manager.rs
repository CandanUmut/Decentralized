use std::collections::HashMap;
use crate::timecoin::peer_id::IPFSPeerId;

/// Struct to manage PeerID ↔ Wallet Address mappings
#[derive(Default)]
pub struct PeerManager {
    peer_map: HashMap<IPFSPeerId, String>, // Maps PeerID to Wallet Address
}

impl PeerManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_mapping(&mut self, peer_id: IPFSPeerId, wallet_address: String) {
        self.peer_map.insert(peer_id, wallet_address);
    }

    pub fn remove_peer(&mut self, peer_id: &IPFSPeerId) {
        self.peer_map.remove(peer_id);
    }

    pub fn get_wallet(&self, peer_id: &IPFSPeerId) -> Option<&String> {
        self.peer_map.get(peer_id)
    }

    pub fn list_peers(&self) -> Vec<IPFSPeerId> {
        self.peer_map.keys().cloned().collect()
    }

    pub fn is_registered(&self, peer_id: &IPFSPeerId) -> bool {
        self.peer_map.contains_key(peer_id)
    }
}
