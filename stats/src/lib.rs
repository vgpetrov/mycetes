pub mod stats_stub;
pub mod stats_client;

pub trait StatsSender: Send + Sync {
    fn incr(&self);
}