use std::{error::Error, path::Path};

use alchemy::alchemy::document::AlchemyDocument;


fn main() -> Result<(), Box<dyn Error>> {
  /* Attempt to load the thing */

  let alchemy_doc = include_str!("../resources/documents/text_image.json");

  let doc: AlchemyDocument = serde_json::from_str(&alchemy_doc)?;
  //println!("{:?}", doc);
  //let _ = doc.to_pdf(Path::new("../")).unwrap();

  println!("{:?}", doc.get_cutline_for_page(0));
  return Ok(())
}

