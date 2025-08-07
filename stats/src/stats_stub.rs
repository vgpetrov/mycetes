use crate::StatsSender;

pub struct StatsStub {}
impl StatsStub {
    pub fn new() -> Self {
        StatsStub {}
    }
}

impl StatsSender for StatsStub {
    fn incr(&self) {
        println!("incr");
    }
}