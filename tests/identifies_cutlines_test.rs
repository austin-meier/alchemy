use alchemy::alchemy::document::AlchemyDocument;


#[test]
fn identify_alchemy_doc_cutline_frames() {
  let doc_json = include_str!("../resources/documents/text_image.json");
  let doc: AlchemyDocument = serde_json::from_str(&doc_json).unwrap();

  let cutline_page1 = doc.get_cutline_for_page(0).unwrap_or_else(|| panic!("Failed to find cutline on page 1"));
  let cutline_page2 = doc.get_cutline_for_page(1).unwrap_or_else(|| panic!("Failed to find cutline on page 2"));

  assert_eq!(cutline_page1.id, "CD114816-C3AB-2E5D-18D2-898581CAF7DE");
  assert_eq!(cutline_page2.id, "F28A6D9A-AA38-70AC-55D0-014591AB7445");
}