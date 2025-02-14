pub mod http;


use configuration::initialize::AppConfig;
use errors::errors::AppResult;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub trait ClientBuilder: Sized {
    fn build_from_config(config: &AppConfig) -> AppResult<Self>;
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
