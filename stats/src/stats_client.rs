use datadog_statsd::Client;
use domain::stats::StatsSender;

pub struct StatsClient {
    statsd_client: Client
}

impl StatsClient {
    pub fn new(host: String, port: u16) -> Self {
        StatsClient {
            statsd_client: Client::new(format!("{}:{}", host, port), "mycetes", Some(vec!["start"])).unwrap()
        }
    }
}

impl StatsSender for StatsClient {
    fn incr(&self, stat_name: &str, tags: Vec<&str>) {
        self.statsd_client.incr(stat_name, &Some(tags));
    }
}