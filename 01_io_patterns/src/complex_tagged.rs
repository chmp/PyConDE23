use anyhow::{bail, Result};
use serde::Serialize;

#[derive(Serialize)]
enum TaggedOutput {
    #[serde(rename = "str")]
    String(String),
    #[serde(rename = "float")]
    Float(f64),
}

fn main() -> Result<()> {
    let inp: String = serde_json::from_reader(std::io::stdin())?;

    let out = match inp.as_str() {
        "string" => TaggedOutput::String(String::from("foo")),
        "float" => TaggedOutput::Float(42.0),
        out => bail!("Did not understand: {out}"),
    };

    serde_json::to_writer(std::io::stdout(), &out)?;

    Ok(())
}
