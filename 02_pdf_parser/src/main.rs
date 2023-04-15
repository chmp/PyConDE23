use std::io::Write;

use lopdf::Document;
use pdf_extract::{MediaBox, OutputDev, OutputError};
use serde::{Deserialize, Serialize};

fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).expect("Usage: pdf-parser PATH");
    let doc = Document::load(&path)?;
    pdf_extract::output_doc(&doc, &mut Output::default())?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct Output {
    current_page: Option<Page>,
}

impl OutputDev for Output {
    fn begin_page(
        &mut self,
        page_num: u32,
        _media_box: &MediaBox,
        _art_box: Option<(f64, f64, f64, f64)>,
    ) -> Result<(), OutputError> {
        self.current_page = Some(Page {
            number: page_num,
            words: Vec::new(),
        });
        Ok(())
    }

    fn end_page(&mut self) -> std::result::Result<(), OutputError> {
        let Some(page) = self.current_page.take() else {
            return Err(OutputError::FormatError(std::fmt::Error));
        };
        let mut page = page;
        page.words.sort_by(|a, b| a.partial_cmp(&b).unwrap());

        let mut stdout = std::io::stdout();
        if let Err(_err) = serde_json::to_writer(&mut stdout, &page) {
            return Err(OutputError::FormatError(std::fmt::Error));
        }
        stdout.write_all(b"\n")?;
        stdout.flush()?;
        Ok(())
    }

    fn output_character(
        &mut self,
        trm: &pdf_extract::Transform,
        _width: f64,
        _spacing: f64,
        _font_size: f64,
        char: &str,
    ) -> std::result::Result<(), OutputError> {
        let Some(current_page) = self.current_page.as_mut() else {
            return Err(OutputError::FormatError(std::fmt::Error));
        };
        let Some(current_word) = current_page.words.last_mut() else {
            return Err(OutputError::FormatError(std::fmt::Error));
        };

        if current_word.text.is_empty() {
            current_word.x = trm.m31;
            current_word.y = trm.m32;
        }
        current_word.text.push_str(char);

        Ok(())
    }

    fn begin_word(&mut self) -> std::result::Result<(), OutputError> {
        let Some(current_page) = self.current_page.as_mut() else {
            return Err(OutputError::FormatError(std::fmt::Error));
        };
        current_page.words.push(Word::default());
        Ok(())
    }

    fn end_word(&mut self) -> std::result::Result<(), OutputError> {
        Ok(())
    }

    fn end_line(&mut self) -> std::result::Result<(), OutputError> {
        Ok(())
    }

    fn fill(
        &mut self,
        _: &pdf_extract::Transform,
        _: &pdf_extract::ColorSpace,
        _: &[f64],
        _: &pdf_extract::Path,
    ) -> std::result::Result<(), OutputError> {
        Ok(())
    }

    fn stroke(
        &mut self,
        _: &pdf_extract::Transform,
        _: &pdf_extract::ColorSpace,
        _: &[f64],
        _: &pdf_extract::Path,
    ) -> std::result::Result<(), OutputError> {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Page {
    number: u32,
    words: Vec<Word>,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
struct Word {
    x: f64,
    y: f64,
    text: String,
}
