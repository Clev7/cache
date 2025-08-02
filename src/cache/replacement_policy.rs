use std::str::FromStr;

#[derive(Debug)]
pub enum ReplacementPolicy {
    Lru     = 0,
    Plru    = 1,
    Optimal = 2
}

impl FromStr for ReplacementPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "0" | "lru" => Ok(Self::Lru),
            "1" | "plru" => Ok(Self::Plru),
            "2" | "optimal" => Ok(Self::Optimal),
            _ => Err(format!("Invalid policy {}: Use 0 (LRU), 1 (PLRU), or 2 (Optimal)", s))
        }
    }
}
