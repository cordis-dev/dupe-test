use std::thread::sleep;
use std::time::Duration;

pub fn with_retry<T, E, F>(mut action: F, attempts: u32) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut delay = Duration::from_millis(100);
    let mut attempt = 1;
    loop {
        match action() {
            Ok(value) => return Ok(value),
            Err(error) if attempt >= attempts => return Err(error),
            Err(_) => {
                sleep(delay);
                delay *= 2;
                attempt += 1;
            }
        }
    }
}
