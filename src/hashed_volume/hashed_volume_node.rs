use super::Children;
#[derive(Clone, Default)]
pub struct HashedVolumeNode {
    pub hash: u64,
    pub children: Children,
}

impl HashedVolumeNode {
    pub fn new(hash: u64, children: Children) -> Self {
        Self { hash, children }
    }
}
