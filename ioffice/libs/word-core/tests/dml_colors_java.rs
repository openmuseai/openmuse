use word_core::cn::wps::moffice::writer::io::reader::dmlReader::{
    context::{
        ColorChoice::ColorChoice, ColorTransform::ColorTransform,
        PrstClr::PrstClr, SrgbClr::SrgbClr, ThemeContext::ThemeContext,
    },
    helper::{ColorsByName::ColorsByName, ColorsTrans::ColorsTrans, HSLHelper::HSLHelper},
};
use word_core::cn::wps::moffice::writer::io::tools::ColorTransform::ColorTransform as IOColorTransform;

#[test]
fn java_preset_lookup_and_theme_guard() {
    let mut choice = ColorChoice::new();
    choice.mPrstClr = Some(PrstClr { base: ColorTransform::default(), mVal: Some(ColorsByName::INT_RED) });
    assert_eq!(ColorsTrans::toColor4Theme(&choice), Some(0x00ff_0000));
    assert_eq!(ColorsTrans::toColor(None, &choice, Some(&ThemeContext::default())), Some(0x00ff_0000));
    choice.mPrstClr.as_mut().unwrap().mVal = Some(0);
    assert_eq!(ColorsTrans::toColor4Theme(&choice), None);
}

#[test]
fn java_alpha_and_hue_conversion() {
    let mut choice = ColorChoice::new();
    let mut transform = ColorTransform::default();
    transform.mAlpha = Some(0.5);
    transform.mAlphaMod = Some(0.5);
    transform.mAlphaOff = Some(0.1);
    transform.mHue = Some(216); // Java divides by 216 as an integer, yielding H=1.
    choice.mSrgbClr = Some(SrgbClr { base: transform, mVal: Some(0x0012_3456) });
    assert_eq!(ColorsTrans::toAlpha(&choice, 1.0), Some(0.35));
    assert_eq!(ColorsTrans::toColor4Theme(&choice), Some(0x0012_3456));
    assert_eq!(ColorsTrans::toColor(None, &choice, Some(&ThemeContext::default())), Some(-1));
    assert_eq!(HSLHelper::new(0x0012_3456).H().is_finite(), true);
    assert_eq!(IOColorTransform::setBlue(0xff12_3456_u32 as i32, 0xab), 0xff12_34ab_u32 as i32);
}
