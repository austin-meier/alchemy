use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AlchemyPDFError {
    InvalidOutputPath,
    NoIngredientsInDocument,
}

impl Error for AlchemyPDFError {}

impl Display for AlchemyPDFError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidOutputPath => f.write_str("The PDF output path failed to be created"),
            Self::NoIngredientsInDocument => {
                f.write_str("The passed document contains no ingredients")
            }
        }
    }
}
