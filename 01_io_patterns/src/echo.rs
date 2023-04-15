use std::io::Write;

type Input = String;

fn main() -> anyhow::Result<()> {
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;

        if line.is_empty() {
            break;
        }

        let inp: Input = serde_json::from_str(&line)?;
        let out = process(inp);

        let out = serde_json::to_vec(&out)?;

        std::io::stdout().write_all(&out)?;
        std::io::stdout().write_all(b"\n")?;
        std::io::stdout().flush()?;
    }

    Ok(())
}

fn process(inp: String) -> String {
    format!("Echo: {inp}")
}
