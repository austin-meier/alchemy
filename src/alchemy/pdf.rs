use printpdf::{Mm, PdfDocument, PdfPage, PdfSaveOptions};
use std::{
    error::Error,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use super::{document::AlchemyDocument, error::AlchemyPDFError};

#[derive(Debug)]
pub enum Layer {
    Root,
}

impl AlchemyDocument {
    pub fn to_pdf(&self, title: &str, path: &Path) -> Result<PathBuf, Box<dyn Error>> {
        let mut doc = PdfDocument::new(title);

        let pages = alchemy_pages_to_pdf_pages(self)?;

        /* Write the PDF */
        let pdf_path = path.join(title.to_owned() + ".pdf");
        let mut file = File::create(&pdf_path)?;

        let pdf_save_options = PdfSaveOptions {
            optimize: false,
            subset_fonts: true,
        };

        file.write(&doc.with_pages(pages).save(&pdf_save_options))?;
        Ok(pdf_path)
    }
}

pub fn alchemy_pages_to_pdf_pages(doc: &AlchemyDocument) -> Result<Vec<PdfPage>, AlchemyPDFError> {
    let pages = doc
        .pages
        .iter()
        .enumerate()
        .map(|(idx, ingredient_ids)| PdfPage::new(Mm(10.0), Mm(10.0), Vec::new()))
        .collect();

    Ok(pages)
}
