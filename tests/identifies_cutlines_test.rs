use alchemy::alchemy::document::AlchemyDocument;


#[test]
fn identify_alchemy_doc_cutline_frames() {
  let doc_json = include_str!("../resources/documents/text_image.json");
  let doc: AlchemyDocument = serde_json::from_str(&doc_json).unwrap();

  let cutline_page1 = doc.get_dielines_for_page(0).unwrap_or_else(|| panic!("Failed to find any dielines on page 1"));
  let cutline_page2 = doc.get_dielines_for_page(1).unwrap_or_else(|| panic!("Failed to find any dielines on page 2"));

  assert_eq!(cutline_page1.len(), 1);
  assert_eq!(cutline_page2.len(), 5);
}