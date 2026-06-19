use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct VndbStats {
    pub chars: u64,
    pub producers: u64,
    pub releases: u64,
    pub staff: u64,
    pub tags: u64,
    pub traits: u64,
    pub vn: u64,
}
