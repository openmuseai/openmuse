#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class ExtTextOutW extends EMFRecord
pub struct ExtTextOutW {
    // Java field: int ETO_GLYPH_INDEX
    // Java field: String text
    text: Option<String>,
    // Java field: PointF pos
    pos: Option<i32>,
    // Java field: int graphicsMode
    graphicsMode: i32,
    // Java field: int options
    options: i32,
    // Java field: float[] widths
    widths: Option<Vec<f32>>,
}

pub(crate) const ETO_GLYPH_INDEX: i32 = 16;

impl ExtTextOutW {
    // Java: ExtTextOutW()
    pub fn new() -> Self {
        // Java: super();
        ExtTextOutW {
            text: None,
            pos: None,
            graphicsMode: 0,
            options: 0,
            widths: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: ExtTextOutW tag = new ExtTextOutW();
        // Java: emf.skip(16);
        // Java: tag.graphicsMode = emf.readInt();
        // Java: emf.skip(8);
        // Java: tag.pos = emf.readIntPoint();
        // Java: int tLength = emf.readUnsignedInt();
        // Java: emf.skip(4);
        // Java: tag.options = emf.readUnsignedInt();
        // Java: emf.skip(20);
        // Java: tag.text = emf.readString(tLength);
        // Java: if (2 * tLength % 4 != 0)
        {
        // Java: int num = 4 - 2 * tLength % 4;
        // Java: emf.skip(num);
        }
        // Java: float[] widths = new float[tLength];
        // Java: for(int i = 0; i < tLength; ++i)
        {
        // Java: widths[i] = (float)emf.readUnsignedInt();
        }
        // Java: tag.widths = widths;
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDC().setFontGraphicsMode(this.graphicsMode);
        // Java: String fontName = renderer.getDC().getFont().familyName;
        // Java: IFontFamily fontFamily = FontHost.getFontTable().get(fontName, true);
        // Java: if (fontFamily != null)
        {
        // Java: if ((this.options & 16) == 16)
        {
        // Java: if (fontFamily.getName().equals(fontName))
        {
        // Java: long typeface = getTypeface(fontFamily, renderer.getDC().getFont().fontStyle);
        // Java: if (typeface != 0L)
        {
        // Java: FreeTypeJNI.TTFHeader header = new FreeTypeJNI.TTFHeader();
        // Java: FreeTypeJNI.getTTFHeader(typeface, header);
        // Java: Path path = new Path();
        // Java: float x = this.pos.x;
        // Java: for(int i = 0; i < this.text.length(); ++i)
        {
        // Java: path.reset();
        // Java: if (FreeTypeJNI.getGlyphPaths(typeface, path, this.text.charAt(i)) && !path.isEmpty())
        {
        // Java: renderer.drawPath(path, x, this.pos.y, header.unitsPerEM);
        }
        // Java: x += this.widths[i];
        }
        // Java: FreeTypeJNI.deleteTypeface(typeface);
        // Java: return;
        }
        }
        }
        // Java: else if (!fontFamily.isSystemFont())
        {
        // Java: int fontStyle = renderer.getDC().getFont().fontStyle;
        // Java: Typeface androidTypeface = (Typeface)fontFamily.getTypeface(fontStyle).getNativeTypeface();
        // Java: if (hasSupplementaryCharacter(this.text))
        {
        // Java: long tf = getTypeface(fontFamily, fontStyle);
        // Java: if (tf != 0L)
        {
        // Java: int len = this.text.length();
        // Java: float x = this.pos.x;
        // Java: float[] width = new float[1];
        // Java: int[] unicodes = new int[1];
        // Java: int[] glyphs = new int[1];
        // Java: FreeTypeJNI.TTFHeader header = new FreeTypeJNI.TTFHeader();
        // Java: FreeTypeJNI.getTTFHeader(tf, header);
        // Java: for(int i = 0; i < len; ++i)
        {
        // Java: if (i + 1 < len && Character.isSurrogatePair(this.text.charAt(i), this.text.charAt(i + 1)))
        {
        // Java: unicodes[0] = Character.toCodePoint(this.text.charAt(i), this.text.charAt(i + 1));
        // Java: if (FreeTypeJNI.getGlyphIndexes(tf, unicodes, 0, unicodes.length, glyphs, 0))
        {
        // Java: Path path = new Path();
        // Java: if (FreeTypeJNI.getGlyphPaths(tf, path, glyphs[0]) && !path.isEmpty())
        {
        // Java: renderer.drawPath(path, x, this.pos.y, header.unitsPerEM);
        }
        }
        // Java: x += this.widths[i];
        // Java: ++i;
        }
        // Java: else
        {
        // Java: width[0] = this.widths[i];
        // Java: renderer.drawTextW(this.text.substring(i, i + 1), (int)x, (int)this.pos.y, width, androidTypeface);
        }
        // Java: x += this.widths[i];
        }
        // Java: FreeTypeJNI.deleteTypeface(tf);
        // Java: return;
        }
        }
        // Java: renderer.drawTextW(this.text, (int)this.pos.x, (int)this.pos.y, this.widths, androidTypeface);
        // Java: return;
        }
        }
        // Java: renderer.drawTextW(this.text, (int)this.pos.x, (int)this.pos.y, this.widths);
    }

    // Java: protected static final long getTypeface(IFontFamily fontFamily, int style)
    pub fn getTypeface(_fontFamily: i32, _style: i32) -> i64 {
        // Java: String[] paths = fontFamily.getAllFontFiles();
        // Java: if (paths != null && paths.length > 0)
        {
        // Java: String path = null;
        // Java: if (paths.length > style)
        {
        // Java: path = paths[style];
        }
        // Java: if (path == null)
        {
        // Java: for(int i = 0; i < paths.length; ++i)
        {
        // Java: if (paths[i] != null)
        {
        // Java: path = paths[i];
        // Java: break;
        }
        }
        }
        // Java: if (path != null)
        {
        // Java: return FreeTypeJNI.newTypeface(path, GDIFont.getFontIndex(path, fontFamily));
        }
        }
        // Java: return 0L;
        0
    }

    // Java: protected static final boolean hasSupplementaryCharacter(String text)
    pub fn hasSupplementaryCharacter(_text: i32) -> bool {
        // Java: int len = text.length();
        // Java: if (len < 2)
        {
        // Java: return false;
        }
        // Java: else
        {
        // Java: for(int i = 0; i < len - 1; ++i)
        {
        // Java: if (Character.isSurrogatePair(text.charAt(i), text.charAt(i + 1)))
        {
        // Java: return true;
        }
        }
        // Java: return false;
        }
        false
    }

}
