// Java: import cn.wps.base.utils.Base64
use crate::compat::base64_util;
// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.writer.UofwPackage
use crate::cn::wps::io::uof::writer::UofwPackage::UofwPackage;
// Java: import cn.wps.kfc.xpxml.writer.XmlWriter
use core_libs_kfc::kfc::xpxml::writer::XmlWriter::XmlWriter;
// Java: import cn.wps.kfc.xpxml.writer.r#impl.BufferXmlWriter
use core_libs_kfc::kfc::xpxml::writer::r#impl::BufferXmlWriter::BufferXmlWriter;
// Java: import java.io.OutputStream
use std::io::Write;

// Java: public class UofwEncryption
pub struct UofwEncryption {
}

// Java: public class UofwEncryption
impl UofwEncryption {
    // Java: UofwEncryption()
    pub fn UofwEncryption() -> Self {
      // Java: super();
      UofwEncryption {}
    }

    // Java: static void exportEncryptInfo(UofwPackage pkg, UofEncryptInfo encInfo)
    pub fn exportEncryptInfo(pkg: &mut UofwPackage, encInfo: &UofEncryptInfo) {
      // Java: OutputStream os = pkg.createOutputStream("security/encryption.xml");
      let os = pkg.createOutputStream("security/encryption.xml".to_string());
      // Java: if (null != os)
      if os.is_some()
      {
         // Java: XmlWriter writer = new BufferXmlWriter(os, true);
         let buf: Vec<u8> = Vec::new();
         let mut writer = BufferXmlWriter::new_with_escape(Box::new(buf), true);
         // Java: writer.startDocument();
         writer.start_document();
         // Java: writer.startElement("安全:安全_F800");
         writer.write_ansi_str("<安全:安全_F800");
         // Java: writer.addNS("安全", "http://www.nits.org.cn/uof3/zh-cn/2021/security");
         writer.write_ansi_str(" xmlns:安全=\"http://www.nits.org.cn/uof3/zh-cn/2021/security\"");
         // Java: writer.addNS("xenc", "http://www.w3.org/2001/04/xmlenc#");
         writer.write_ansi_str(" xmlns:xenc=\"http://www.w3.org/2001/04/xmlenc#\"");
         // Java: writer.addNS("ds", "http://www.w3.org/2000/09/xmldsig");
         writer.write_ansi_str(" xmlns:ds=\"http://www.w3.org/2000/09/xmldsig\"");
         writer.write_ansi_str(">");
         // Java: writeSubElement(encInfo, writer);
         Self::writeSubElement(encInfo, &mut writer);
         // Java: writer.endDocument();
         writer.end_document();
      }
    }

    // Java: static void writeSubElement(UofEncryptInfo encInfo, XmlWriter writer)
    fn writeSubElement(encInfo: &UofEncryptInfo, writer: &mut BufferXmlWriter) {
      // Java: writer.startElement("安全:加密_F80A");
      writer.write_ansi_str("<安全:加密_F80A>");
      // Java: writer.startElement("ds:KeyInfo");
      writer.write_ansi_str("<ds:KeyInfo>");
      // Java: writer.startElement("ds:加密密钥");
      writer.write_ansi_str("<ds:加密密钥>");
      // Java: writer.startElement("xenc:EncryptionMethod");
      // Java: writer.addAttr("Algorithm", "http://www.w3.org/2001/04/xmldsig-more#sm4");
      writer.write_ansi_str("<xenc:EncryptionMethod Algorithm=\"http://www.w3.org/2001/04/xmldsig-more#sm4\">");
      // Java: writer.startElement("xenc:KeySize");
      writer.write_ansi_str("<xenc:KeySize>");
      // Java: writer.addText(Integer.toString(encInfo.getKeyBits()));
      writer.write_ansi_str(&encInfo.getKeyBits().to_string());
      // Java: writer.endElement("xenc:KeySize");
      writer.write_ansi_str("</xenc:KeySize>");
      // Java: writer.endElement("xenc:EncryptionMethod");
      writer.write_ansi_str("</xenc:EncryptionMethod>");
      // Java: writer.startElement("xenc:CipherData");
      writer.write_ansi_str("<xenc:CipherData>");
      // Java: writer.startElement("xenc:CipherValue");
      writer.write_ansi_str("<xenc:CipherValue>");
      // Java: writer.addText(Base64.encode(encInfo.getCryptedKey()));
      writer.write_ansi_str(&base64_util::encode(encInfo.getCryptedKey()));
      // Java: writer.endElement("xenc:CipherValue");
      writer.write_ansi_str("</xenc:CipherValue>");
      // Java: writer.endElement("xenc:CipherData");
      writer.write_ansi_str("</xenc:CipherData>");
      // Java: writer.endElement("ds:加密密钥");
      writer.write_ansi_str("</ds:加密密钥>");
      // Java: writer.endElement("ds:KeyInfo");
      writer.write_ansi_str("</ds:KeyInfo>");
      // Java: writer.startElement("xenc:CipherData");
      writer.write_ansi_str("<xenc:CipherData>");
      // Java: writer.startElement("xenc:CipherReference");
      // Java: writer.addAttr("URI", "/security/encryptedData.bin");
      writer.write_ansi_str("<xenc:CipherReference URI=\"/security/encryptedData.bin\"/>");
      // Java: writer.endElement("xenc:CipherReference");
      // Java: writer.endElement("xenc:CipherData");
      writer.write_ansi_str("</xenc:CipherData>");
      // Java: writer.startElement("xenc:EncryptionProperties");
      writer.write_ansi_str("<xenc:EncryptionProperties>");
      // Java: writer.startElement("xenc:EncryptionProperty");
      writer.write_ansi_str("<xenc:EncryptionProperty>");
      // Java: writer.startElement("xenc:分块大小");
      writer.write_ansi_str("<xenc:分块大小>");
      // Java: writer.addText(Integer.toString(encInfo.getBlockSize()));
      writer.write_ansi_str(&encInfo.getBlockSize().to_string());
      // Java: writer.endElement("xenc:分块大小");
      writer.write_ansi_str("</xenc:分块大小>");
      // Java: writer.endElement("xenc:EncryptionProperty");
      writer.write_ansi_str("</xenc:EncryptionProperty>");
      // Java: writer.startElement("xenc:EncryptionProperty");
      writer.write_ansi_str("<xenc:EncryptionProperty>");
      // Java: writer.startElement("xenc:加密盐值");
      writer.write_ansi_str("<xenc:加密盐值>");
      // Java: writer.addText(Base64.encode(encInfo.getSaltValue()));
      writer.write_ansi_str(&base64_util::encode(encInfo.getSaltValue()));
      // Java: writer.endElement("xenc:加密盐值");
      writer.write_ansi_str("</xenc:加密盐值>");
      // Java: writer.endElement("xenc:EncryptionProperty");
      writer.write_ansi_str("</xenc:EncryptionProperty>");
      // Java: writer.startElement("xenc:EncryptionProperty");
      writer.write_ansi_str("<xenc:EncryptionProperty>");
      // Java: writer.startElement("xenc:迭代运算次数");
      writer.write_ansi_str("<xenc:迭代运算次数>");
      // Java: writer.addText(Integer.toString(encInfo.getSpinCount()));
      writer.write_ansi_str(&encInfo.getSpinCount().to_string());
      // Java: writer.endElement("xenc:迭代运算次数");
      writer.write_ansi_str("</xenc:迭代运算次数>");
      // Java: writer.endElement("xenc:EncryptionProperty");
      writer.write_ansi_str("</xenc:EncryptionProperty>");
      // Java: writer.startElement("xenc:EncryptionProperty");
      writer.write_ansi_str("<xenc:EncryptionProperty>");
      // Java: writer.startElement("xenc:密码盐值");
      writer.write_ansi_str("<xenc:密码盐值>");
      // Java: writer.addText(Base64.encode(encInfo.getVerifiedSaltValue()));
      writer.write_ansi_str(&base64_util::encode(encInfo.getVerifiedSaltValue()));
      // Java: writer.endElement("xenc:密码盐值");
      writer.write_ansi_str("</xenc:密码盐值>");
      // Java: writer.endElement("xenc:EncryptionProperty");
      writer.write_ansi_str("</xenc:EncryptionProperty>");
      // Java: writer.startElement("xenc:EncryptionProperty");
      writer.write_ansi_str("<xenc:EncryptionProperty>");
      // Java: writer.startElement("xenc:密码校验值");
      writer.write_ansi_str("<xenc:密码校验值>");
      // Java: writer.startElement("ds:DigestMethod");
      // Java: writer.addAttr("Algorithm", "http://www.w3.org/2001/04/xmldsig-more#sm3");
      writer.write_ansi_str("<ds:DigestMethod Algorithm=\"http://www.w3.org/2001/04/xmldsig-more#sm3\"/>");
      // Java: writer.endElement("ds:DigestMethod");
      // Java: writer.startElement("ds:DigestValue");
      writer.write_ansi_str("<ds:DigestValue>");
      // Java: writer.addText(Base64.encode(encInfo.getVerifiedEncPassword()));
      writer.write_ansi_str(&base64_util::encode(encInfo.getVerifiedEncPassword()));
      // Java: writer.endElement("ds:DigestValue");
      writer.write_ansi_str("</ds:DigestValue>");
      // Java: writer.endElement("xenc:密码校验值");
      writer.write_ansi_str("</xenc:密码校验值>");
      // Java: writer.endElement("xenc:EncryptionProperty");
      writer.write_ansi_str("</xenc:EncryptionProperty>");
      // Java: writer.endElement("xenc:EncryptionProperties");
      writer.write_ansi_str("</xenc:EncryptionProperties>");
      // Java: writer.endElement("安全:加密_F80A");
      writer.write_ansi_str("</安全:加密_F80A>");
      // Java: writer.endElement("安全:安全_F800");
      writer.write_ansi_str("</安全:安全_F800>");
    }

}
