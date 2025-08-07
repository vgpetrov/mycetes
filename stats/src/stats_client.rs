use datadog_statsd::Client;
use crate::StatsSender;

pub struct StatsClient {
    statsd_client: Client
}

impl StatsClient {
    pub fn new() -> Self {
        StatsClient {
            statsd_client: Client::new("192.168.1.3:8125", "mycetes.request", Some(vec!["start"])).unwrap()
        }
    }
}

impl StatsSender for StatsClient {
    fn incr(&self) {
        self.statsd_client.incr("hello", &Some(vec!["abc"]));
    }
}