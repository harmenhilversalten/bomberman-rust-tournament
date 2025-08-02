/// Cache eviction strategies.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvictionPolicy {
    /// Least recently used entry is evicted first.
    Lru,
    /// First in, first out eviction.
    Fifo,
}
