use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Input {
    value: i64,
}

#[derive(Serialize)]
struct Output {
    value: i64,
}

fn main() -> anyhow::Result<()> {
    let inp: Input = serde_json::from_reader(std::io::stdin())?;

    let out = Output {
        value: 2 * inp.value,
    };

    serde_json::to_writer(std::io::stdout(), &out)?;
    Ok(())
}
