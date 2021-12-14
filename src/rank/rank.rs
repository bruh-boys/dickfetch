
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BenchMarkRank {
  Lowest(String),
  Low(String),
  Medium(String),
  High(String),
  Highest(String),
  Rage(String)
}

impl ToString for BenchMarkRank {
  fn to_string(&self) -> String {
    match self {
      BenchMarkRank::Lowest(s) => s.to_string(),
      BenchMarkRank::Low(s) => s.to_string(),
      BenchMarkRank::Medium(s) => s.to_string(),
      BenchMarkRank::High(s) => s.to_string(),
      BenchMarkRank::Highest(s) => s.to_string(),
      BenchMarkRank::Rage(s) => s.to_string()
    }
  }
}