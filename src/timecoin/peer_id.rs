use ipfs::PeerId;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct IPFSPeerId(PeerId);

impl IPFSPeerId {
    pub fn from_libp2p(peer_id: libp2p::PeerId) -> Result<Self, String> {
        PeerId::from_bytes(&peer_id.to_bytes()) // Use a reference to Vec<u8>
            .map(Self)
            .map_err(|e| format!("Failed to convert PeerId: {}", e))
    }

    pub fn to_libp2p(&self) -> libp2p::PeerId {
        libp2p::PeerId::from_bytes(&self.0.to_bytes())
            .expect("Conversion back to libp2p::PeerId should not fail")
    }

    pub fn to_base58(&self) -> String {
        self.0.to_base58()
    }

    pub fn clone_inner(&self) -> PeerId {
        self.0.clone()
    }
}

impl std::fmt::Display for IPFSPeerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}
