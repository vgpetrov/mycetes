use domain::stats::StatsSender;

pub struct StatsStub {}
impl StatsStub {
    pub fn new() -> Self {
        StatsStub {}
    }
}

impl StatsSender for StatsStub {
    fn incr(&self, stat_name: &str, tags: Vec<&str>) {
        println!("incr");
    }
}