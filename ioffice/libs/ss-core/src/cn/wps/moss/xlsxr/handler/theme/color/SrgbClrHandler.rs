use crate::{
    cn::wps::moss::core::theme::{
        ColorElement::ColorElement, CsColor::ColorType, SrgbColor::SrgbColor,
    },
    java_runtime::{Rc, RefCell},
};
use kfc_kfc_alpha::kfc::xpxml::reader::{
    ElementAttrHandler::ElementAttrHandler, XmlRoAttr::XmlRoAttr,
};
// java public class SrgbClrHandler implements ElementAttrHandler
pub struct SrgbClrHandler {
    colorElement: Rc<ColorElement>,
    srgbColor: RefCell<SrgbColor>,
}
impl SrgbClrHandler {
    pub fn new(element: Rc<ColorElement>) -> Self {
        Self {
            colorElement: element,
            srgbColor: RefCell::new(SrgbColor::default()),
        }
    }
}
impl ElementAttrHandler for SrgbClrHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/theme/color/SrgbClrHandler.java:33
    // java public void addElementAttr(int element, XmlRoAttr attrsNotCopy) {
    // java XmlRoAttr tmp = null;
    // java if ((tmp = attrsNotCopy.getByName(1114158)) != null) {
    // java this.srgbColor.val = tmp.toString();
    // java int[] rgbClr = ValueHelper.colorToByte(tmp.toString());
    // java Color.RGB srgb = Color.RGB.create();
    // java srgb.putR(rgbClr[0]);
    // java srgb.putG(rgbClr[1]);
    // java srgb.putB(rgbClr[2]);
    // java srgb.save();
    // java this.color.forceWriteSrgb(srgb);
    // java }
    // java
    // java List<ColorTransform> colorTransforms = new ArrayList();
    // java double value = (double)0.0F;
    // java int count = attrsNotCopy.count();
    // java
    // java for(int i = 0; i < count; ++i) {
    // java tmp = attrsNotCopy.getAttrAt(i);
    // java ColorTransform colorTransform = null;
    // java switch (tmp.getName()) {
    // java case 1114165: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(27);
    // java colorTransform.putTint(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.tint, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114166: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(26);
    // java colorTransform.putShade(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.shade, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114167: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java colorTransform.putType(6);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.comp, val.toString());
    // java break;
    // java }
    // java case 1114168: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java colorTransform.putType(15);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.inv, val.toString());
    // java break;
    // java }
    // java case 1114169: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java colorTransform.putType(8);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.gray, val.toString());
    // java break;
    // java }
    // java case 1114170: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(0);
    // java colorTransform.putAlpha(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.alpha, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114171: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(2);
    // java colorTransform.putAlphaOff(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.alphaOff, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114172: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(1);
    // java colorTransform.putAlphaMod(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.alphaMod, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114173: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = (double)val.toInt();
    // java colorTransform.putType(12);
    // java colorTransform.putHue(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.hue, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114174: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = (double)val.toInt();
    // java colorTransform.putType(14);
    // java colorTransform.putHueOff(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.hueOff, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114175: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(13);
    // java colorTransform.putHueMod(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.hueMod, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114176: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(23);
    // java colorTransform.putSat(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.sat, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114177: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(25);
    // java colorTransform.putSatOff(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.satOff, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114178: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(24);
    // java colorTransform.putSatMod(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.satMod, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114179: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(17);
    // java colorTransform.putLum(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.lum, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114180: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(19);
    // java colorTransform.putLumOff(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.lumOff, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114181: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(18);
    // java colorTransform.putLumMod(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.lumMod, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114182: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(20);
    // java colorTransform.putRed(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.red, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114183: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(22);
    // java colorTransform.putRedOff(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.refOff, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114184: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(21);
    // java colorTransform.putRedMod(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.redMod, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114185: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(9);
    // java colorTransform.putGreen(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.green, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114186: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(11);
    // java colorTransform.putGreenOff(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.greenOff, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114187: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(10);
    // java colorTransform.putGreenMod(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.greenMod, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114188: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(3);
    // java colorTransform.putBlue(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.blue, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114189: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(5);
    // java colorTransform.putBlueOff(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.blueOff, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114190: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java if (val != null) {
    // java value = val.toPercentage();
    // java colorTransform.putType(4);
    // java colorTransform.putBlueMod(value);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.blueMod, val.toString());
    // java }
    // java break;
    // java }
    // java case 1114191: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java colorTransform.putType(7);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.gamma, val.toString());
    // java break;
    // java }
    // java case 1114192: {
    // java colorTransform = ColorTransform.create();
    // java XmlRoAttr val = tmp.getByName(1114158);
    // java colorTransform.putType(16);
    // java this.srgbColor.addDefineColor(CsColor.ColorType.invGamma, val.toString());
    // java break;
    // java }
    // java }
    // java
    // java if (null != colorTransform) {
    // java colorTransform.save();
    // java colorTransforms.add(colorTransform);
    // java }
    // java }
    // java
    // java if (colorTransforms.size() > 0) {
    // java Color.ColorTransformList colorTransformList = Color.ColorTransformList.create();
    // java colorTransformList.forceWrite(colorTransforms);
    // java colorTransformList.save();
    // java this.color.forceWriteTransformLst(colorTransformList);
    // java }
    // java
    // java if (this.colorElement != null) {
    // java this.colorElement.setColor(this.srgbColor.colorValue());
    // java } else if (this.outerShdw != null) {
    // java this.outerShdw.addColor(this.srgbColor);
    // java }
    // java
    // java }
    fn addElementAttr(&self, _: i32, a: &dyn XmlRoAttr) {
        let mut srgb = self.srgbColor.borrow_mut();
        // rust: if ((tmp = attrsNotCopy.getByName(1114158)) != null) { this.srgbColor.val = tmp.toString(); ... }
        if let Some(value) = a.get_by_name(1114158) {
            srgb.val = Some(value.to_string());
        }
        for i in 0..a.count() {
            let tmp = a.get_attr_at(i).unwrap();
            let ty = match tmp.get_name() {
                1114165 => ColorType::tint,
                1114166 => ColorType::shade,
                1114167 => ColorType::comp,
                1114168 => ColorType::inv,
                1114169 => ColorType::gray,
                1114170 => ColorType::alpha,
                1114171 => ColorType::alphaOff,
                1114172 => ColorType::alphaMod,
                1114173 => ColorType::hue,
                1114174 => ColorType::hueOff,
                1114175 => ColorType::hueMod,
                1114176 => ColorType::sat,
                1114177 => ColorType::satOff,
                1114178 => ColorType::satMod,
                1114179 => ColorType::lum,
                1114180 => ColorType::lumOff,
                1114181 => ColorType::lumMod,
                1114182 => ColorType::red,
                1114183 => ColorType::refOff,
                1114184 => ColorType::redMod,
                1114185 => ColorType::green,
                1114186 => ColorType::greenOff,
                1114187 => ColorType::greenMod,
                1114188 => ColorType::blue,
                1114189 => ColorType::blueOff,
                1114190 => ColorType::blueMod,
                1114191 => ColorType::gamma,
                1114192 => ColorType::invGamma,
                _ => continue,
            };
            let value = tmp.get_by_name(1114158);
            // rust: Original unguarded comp/inv/gray/gamma/invGamma val dereferences throw when absent.
            if [1114167, 1114168, 1114169, 1114191, 1114192].contains(&tmp.get_name()) {
                srgb.base
                    .addDefineColor(ty, value.expect("NullPointerException").to_string());
            } else if let Some(value) = value {
                srgb.base.addDefineColor(ty, value.to_string());
            }
        }
        // java if (this.colorElement != null) { this.colorElement.setColor(this.srgbColor.colorValue()); }
        self.colorElement.setColor(srgb.colorValue());
    }
}
