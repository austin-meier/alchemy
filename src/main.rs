use std::{error::Error, path::{Path, PathBuf}};

use alchemy::alchemy::document::AlchemyDocument;


fn main() -> Result<(), Box<dyn Error>> {
  /* Attempt to load the thing */

  let alchemy_doc = include_str!("../resources/documents/text_image.json");

  let doc: AlchemyDocument = serde_json::from_str(&alchemy_doc)?;

  doc.to_pdf("first.test", Path::new("./"))?;

  Ok(())
}

