use std::{collections::HashMap, error::Error, fs::File, io::Write, path::{Path, PathBuf}, str::FromStr};
use image::EncodableLayout;
use printpdf::{PdfDocument, PdfDocumentReference, PdfLayerIndex, PdfLayerReference, PdfPageIndex, PdfPageReference};

use crate::ingredient::{self, traits::dimension::Dimensionable};

use super::{document::AlchemyDocument, error::AlchemyPDFError};

#[derive(Eq, Hash, Debug)]
pub enum Layer {
  Root
}


impl AlchemyDocument {
	pub fn to_pdf(&self, title: &str, path: &Path) -> Result<PathBuf, Box<dyn Error>> {

    let largest_ingredient =
      self
      .get_biggest_ingredient()
      .ok_or(AlchemyPDFError::NoIngredientsInDocument)?;

    let (mut doc, page1, root_layer) = PdfDocument::new(
      title,
      largest_ingredient.width().into(),
      largest_ingredient.height().into(),
      "Root"
    );
    let mut lookup = PdfLookup::new(page1, root_layer);

    doc = add_ingredients_to_pdf(doc, &mut lookup, self)?;

    /* Write the PDF */
    let pdf_path = path.join(title.to_owned() + ".pdf");
    let mut file = File::create(&pdf_path)?;
    file.write(doc.save_to_bytes()?.as_bytes())?;
		Ok(pdf_path)
	}
}

pub struct PdfLookup {
  pages: Vec<PdfPageIndex>,
  layers: HashMap<Layer, PdfLayerIndex>
}

impl PdfLookup {
  pub fn new(page1: PdfPageIndex, root_layer: PdfLayerIndex) -> Self {
    let mut map: HashMap<Layer, PdfLayerIndex> = HashMap::new();
    map.insert(Layer::Root, root_layer);
    PdfLookup {
      pages: vec![page1],
      layers: map
    }
  }

  pub fn get_page_idx(&self, idx: usize) -> Option<&PdfPageIndex> {
    self.pages.get(idx)
  }

  pub fn get_layer_idx(&self, layer: &Layer) -> Option<&PdfLayerIndex> {
    self.layers.get(&layer)
  }
}

pub fn add_ingredients_to_pdf(pdf: PdfDocumentReference, pdf_lookup: &mut PdfLookup, doc: &AlchemyDocument)
-> Result<PdfDocumentReference, AlchemyPDFError> {

  for (idx, ingredients) in doc.pages.iter().enumerate() {
    let page_idx = pdf_lookup.get_page_idx(idx)
      .unwrap_or_else(|| {
        /* Create a the new page */
        let new_page = pdf.add_page()
      })
  }

  Ok(pdf);
}
