pub trait StatsSender: Send + Sync {
    fn incr(&self);
}