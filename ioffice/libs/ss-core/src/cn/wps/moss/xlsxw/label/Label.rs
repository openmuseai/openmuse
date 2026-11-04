use kfc_kfc_alpha::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java public interface Label { void write(XmlWriter var1); }
pub trait Label{fn write(&self,writer:&mut dyn XmlWriter);}
