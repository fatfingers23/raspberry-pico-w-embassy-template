use heapless::Vec;

const _ENV_DATA: &str = include_str!("../.env");

pub fn env_value(key: &str) -> &'static str {
    for line in _ENV_DATA.lines() {
        let parts: Vec<&str, 2> = line.split('=').collect();
        if parts.len() == 2 {
            if parts[0].trim() == key {
                let mut value = parts[1].trim().chars();
                value.next();
                value.next_back();
                return value.as_str();
            }
        }
    }
    panic!("Key: {:?} not found in .env file. May also need to provide your own .env from a copy of .env.save", key);
}
