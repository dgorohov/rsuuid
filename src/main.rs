use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::{Uuid};
use uuid::v1::{Context, Timestamp};

fn _v1(now: Duration) -> Result<String, String> {
    let context = Context::new(42);
    let ts = Timestamp::from_unix(
        &context,
        now.as_secs(),
        now.subsec_nanos(),
    );

    let _uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]);
    return match _uuid {
        Ok(v) => Result::Ok(v.to_hyphenated().to_string()),
        Err(e) => Result::Err(e.to_string())
    };
}

fn v1() -> Result<String, String> {
    return match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(now) => _v1(now),
        Err(e) => Result::Err(e.to_string())
    };
}

fn v4() -> String { Uuid::new_v4().to_hyphenated().to_string() }

fn main() {
    let command = std::env::args().nth(1);

    match command.as_deref() {
        Some("v1") => match v1() {
            Err(e) => println!("Failed to generate UUID v1: {}", e),
            Ok(res) => println!("{}", res),
        },
        Some("v4") => println!("{}", v4()),
        _ => println!("Please select what types of UUID you'd like to generate: v1 or v4")
    }
}
