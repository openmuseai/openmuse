use core_libs_io::io::util::URI::URI;

#[test]
fn external_template_target_with_chinese_name_keeps_the_full_path() {
    // The complex sample's settings relationship has this Java URI shape.
    // Previously parseURI used UTF-8 bytes as indexes into a char array and panicked.
    let target = "/Users/mac/templates/机械制造与自动化毕业论文.doc.docx";
    let uri = URI::new(target).expect("Java URI parser accepts this path");
    assert_eq!(uri.getRawPath(), Some(target));
}
