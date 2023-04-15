use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Input {
    floats: Vec<f64>,
    flexible: Flexible,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Flexible {
    String(String),
    Int(i64),
}

#[derive(Serialize)]
struct Output {
    sum: f64,
    flexible: Flexible,
}

fn main() -> Result<()> {
    let inp: Input = serde_json::from_reader(std::io::stdin())?;
    let out = process(inp);
    serde_json::to_writer(std::io::stdout(), &out)?;

    Ok(())
}

fn process(inp: Input) -> Output {
    Output {
        sum: inp.floats.iter().sum(),
        flexible: inp.flexible,
    }
}
