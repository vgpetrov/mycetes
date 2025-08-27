pub mod domain_event;
pub mod stats;
pub mod repository;

pub struct User {
    pub name: String,
    pub email: String,
}

pub struct Place {
    pub id: Option<i64>,
    pub name: String,
    pub user_id: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub metadata: String,
    pub is_deleted: bool,
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
