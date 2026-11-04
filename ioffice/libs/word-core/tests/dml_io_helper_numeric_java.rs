use word_core::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;

#[test]
fn java_percentage_and_angle_conversions() {
    assert_eq!(IOHelper::Percentage2Float(Some("50000")), Some(0.5));
    assert_eq!(IOHelper::PositiveFixedPercentage2Float(Some("100000")), Some(1.0));
    // Java TypeConversion.string2Float receives the unstripped `%` and returns null.
    assert_eq!(IOHelper::Percentage2Float(Some("50%")), None);
    assert_eq!(IOHelper::FixedPercentage2Float(None), None);
    assert!(IOHelper::checkPercentSymbol(Some(" 50% ")));
    assert_eq!(IOHelper::fixedAngle2Integer(Some("60000")), Some(60000));
    assert_eq!(IOHelper::positiveFixedAngle2Float(Some("60000")), Some(1.0));
    assert_eq!(IOHelper::colorTemperature2Float(Some("6500")), Some(6500.0));
    assert_eq!(IOHelper::addAlphaToCoreColor(0x00abcdef, 0.5), 0x7fabcdef);
    assert_eq!(IOHelper::toOpacity(0.25), 0.75);
    assert_eq!(IOHelper::toTransparency(0.75), 0.25);
}
