pub trait StatsSender: Send + Sync {
    fn incr(&self, stat_name: &str, tags: Vec<&str>);
}