use std::{fmt::Display, path::{Path, PathBuf}, str::FromStr};
use super::document::AlchemyDocument;


#[derive(Debug)]
pub enum AlchemyPDFError {
	InvalidOutputPath,
}

impl Display for AlchemyPDFError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AlchemyPDFError::InvalidOutputPath => f.write_str("The PDF output path failed to be created")
    }
  }
}


impl AlchemyDocument {
	pub fn to_pdf(&self, path: &Path) -> Result<PathBuf, AlchemyPDFError> {
    //let (doc, page1, layer1) = printpdf::PdfDocument::new();
		Ok(PathBuf::from_str("../").unwrap())
	}
}