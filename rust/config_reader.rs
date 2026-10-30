pub struct Config {
    pub host: String,
    pub port: u16,
    pub retries: u32,
    pub verbose: bool,
}

pub fn parse_config(input: &str) -> Result<Config, String> {
    let mut host = String::from("localhost");
    let mut port = 8080;
    let mut retries = 3;
    let mut verbose = false;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line.split_once('=').ok_or(format!("bad line: {line}"))?;
        match key.trim() {
            "host" => host = value.trim().to_string(),
            "port" => port = value.trim().parse().map_err(|_| "bad port".to_string())?,
            "retries" => retries = value.trim().parse().map_err(|_| "bad retries".to_string())?,
            "verbose" => verbose = value.trim() == "true",
            other => return Err(format!("unknown key: {other}")),
        }
    }
    Ok(Config { host, port, retries, verbose })
}
