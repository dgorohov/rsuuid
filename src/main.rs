extern crate core;

use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{Rng, RngCore};
use uuid::{Builder, Error, Uuid, Variant, Version};
use uuid::v1::{Context, Timestamp};

struct AppContext<const S: usize>([u8; S]);

impl<const S: usize> AppContext<S> {
    fn new() -> Self {
        let mut node_id: [u8; S] = [0u8; S];
        rand::thread_rng().fill_bytes(&mut node_id);
        AppContext(node_id)
    }
}

trait FnGetUuid {
    fn uuid(&self) -> Result<Uuid, Error>;
}

impl FnGetUuid for AppContext<6> {
    fn uuid(&self) -> Result<Uuid, Error> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let mut rng = rand::thread_rng();
        let context = Context::new(rng.gen::<u16>());

        let ts = Timestamp::from_unix(
            &context,
            now.as_secs(),
            now.subsec_nanos(),
        );
        let (ticks, counter) = ts.to_rfc4122();

        let time_low = (ticks & 0xFFFF_FFFF) as u32;
        let time_mid = ((ticks >> 32) & 0xFFFF) as u16;
        let time_high_and_version = (((ticks >> 48) & 0x0FFF) as u16) | (1 << 12);

        let mut d4 = [0u8; 8];
        d4[0] = (((counter & 0x3F00) >> 8) as u8) | 0x80;
        d4[1] = (counter & 0xFF) as u8;
        d4[2..].copy_from_slice(&self.0);

        Ok(Uuid::from_fields(time_low, time_mid, time_high_and_version, &d4))
    }
}

impl FnGetUuid for AppContext<16> {
    fn uuid(&self) -> Result<Uuid, Error> {
        Builder::from_slice(&self.0)
            .map(|u| u.with_variant(Variant::RFC4122).with_version(Version::Random))
            .map(|u| u.into_uuid())
    }
}


struct App(Box<dyn FnGetUuid>);

impl App {
    fn new(f: Box<dyn FnGetUuid>) -> Self {
        Self(f)
    }

    fn from_command(cmd: Option<String>) -> Result<Self, simple_error::SimpleError> {
        match cmd.as_deref() {
            Some("v1") => Ok(App::new(Box::new(AppContext::<6>::new()))),
            Some("v4") => Ok(App::new(Box::new(AppContext::<16>::new()))),
            _ => Err(simple_error::SimpleError::new("Please select what types of UUID you'd like to generate: v1 or v4"))
        }
    }
}

fn main() {
    let code = match App::from_command(std::env::args().nth(1))
        .map(|a| a.0) {
        Ok(b) => match b.uuid() {
            Ok(u) => {
                println!("{}", u);
                0
            }
            Err(e) => {
                println!("{}", e);
                -1
            }
        },
        Err(e) => {
            println!("Error: {}", e);
            1
        }
    };
    exit(code)
}
