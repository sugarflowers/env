use std::env;

fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(e) => panic!("{}", e),
    }
}

