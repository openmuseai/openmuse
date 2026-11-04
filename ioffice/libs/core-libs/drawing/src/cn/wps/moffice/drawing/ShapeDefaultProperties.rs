#![allow(non_snake_case, dead_code, unused_variables, unused_mut)]

// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.moffice.drawing.graphics.SolidFill
// Java: import cn.wps.moffice.drawing.line.LineProperty
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import java.io.BufferedReader
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
// Java: import java.io.InputStreamReader

use std::sync::Arc;
use std::any::Any;
use std::io::BufRead;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class ShapeDefaultProperties
pub struct ShapeDefaultProperties {
    // Java field: String mPath
    mPath: Option<String>,
    // Java field: String NAME
    // Java field: String SPLIT
    // Java field: String LINECOLOR
    // Java field: String FILLCOLOR
    // Java field: String TEXTCOLOR
    // Java field: String TEXTSIZE
    // Java field: String TEXTName
}

pub(crate) const NAME: &str = "shape.dpr";
pub(crate) const SPLIT: &str = ":";
pub(crate) const LINECOLOR: &str = "lineColor";
pub(crate) const FILLCOLOR: &str = "fillColor";
pub(crate) const TEXTCOLOR: &str = "textColor";
pub(crate) const TEXTSIZE: &str = "textSize";
pub(crate) const TEXTName: &str = "textName";

// Java: public class ShapeDefaultProperties
impl ShapeDefaultProperties {
    // Java: ShapeDefaultProperties()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.mPath = Platform.getShapePath() + "shape.dpr";
      // TODO: Platform.getShapePath not yet available
      ShapeDefaultProperties { mPath: None }
    }

    // Java: Shape getDefaultProperties()
    pub fn getDefaultProperties(&self) -> Element {
      // Java: File file = new File(this.mPath);
      let path = match &self.mPath {
          Some(p) => p.clone(),
          None => return None,
      };
      // Java: if (file != null && file.exists())
      if std::path::Path::new(&path).exists()
      {
         // Java: try
         {
            // Java: FileInputStream fs = new FileInputStream(file);
            // Java: InputStreamReader streamReader = new InputStreamReader(fs);
            // Java: BufferedReader reader = new BufferedReader(streamReader);
            match std::fs::File::open(&path) {
                Ok(file) => {
                    let reader = std::io::BufReader::new(file);
                    // Java: Shape shape = new Shape((IDrawingContainer)null);
                    // TODO: Shape not yet available
                    // Java: String line;
                    // Java: while((line = reader.readLine()) != null)
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            // Java: if (line.startsWith("lineColor"))
                            if line.starts_with("lineColor")
                            {
                               // Java: this.getLine(shape).setColor(Integer.valueOf(line.substring(line.indexOf(":"))));
                               // TODO: Shape.getLineProperty not yet available
                            }
                            // Java: else if (line.startsWith("fillColor"))
                            else if line.starts_with("fillColor")
                            {
                               // Java: this.getSolidFill(shape).setColor(Integer.valueOf(line.substring(line.indexOf(":"))));
                               // TODO: Shape.getSolidFill not yet available
                            }
                        }
                    }
                    // Java: reader.close();
                    // Java: streamReader.close();
                    // Java: fs.close();
                    // Java: return shape;
                    // TODO: Shape not yet available
                    return None;
                }
                Err(_) => {}
            }
         }
         // Java: catch (FileNotFoundException e)
         {
            // Java: e.printStackTrace();
         }
         // Java: catch (IOException e)
         {
            // Java: e.printStackTrace();
         }
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: return null;
         return None;
      }
    }

    // Java: void setDefaultProperties(Shape shape)
    pub fn setDefaultProperties(&self, _shape: &Element) {
      // Java: try
      {
         // Java: File file = new File(this.mPath);
         let path = match &self.mPath {
             Some(p) => p.clone(),
             None => return,
         };
         // Java: if (!file.exists())
         if !std::path::Path::new(&path).exists()
         {
            // Java: file.createNewFile();
            let _ = std::fs::File::create(&path);
         }
         // Java: FileOutputStream fout = new FileOutputStream(file);
         // Java: StringBuffer props = new StringBuffer();
         let mut props = String::new();
         // Java: LineProperty line = shape.getLineProperty();
         // TODO: Shape not yet available
         // Java: if (line != null && line.getfLine())
         {
            // Java: int clr = line.getColor();
            // Java: props.append("lineColor");
            // Java: props.append(":");
            // Java: props.append(clr);
         }
         // Java: SolidFill fill = shape.getSolidFill();
         // TODO: Shape not yet available
         // Java: if (fill != null && fill.getFillOn())
         {
            // Java: int clr = fill.getColor();
            // Java: props.append("fillColor");
            // Java: props.append(":");
            // Java: props.append(clr);
         }
         // Java: if (props.length() > 0)
         if !props.is_empty()
         {
            // Java: fout.write(props.toString().getBytes());
            let _ = std::fs::write(&path, props.as_bytes());
         }
         // Java: fout.close();
      }
      // Java: catch (FileNotFoundException e)
      {
         // Java: e.printStackTrace();
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
      }
    }

    // Java: LineProperty getLine(Shape shape)
    pub fn getLine(&self, _shape: &Element) -> Element {
      // Java: if (shape == null)
      if _shape.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: LineProperty line = shape.getLineProperty();
         // TODO: Shape.getLineProperty not yet available
         // Java: if (line == null)
         {
            // Java: line = new LineProperty();
            // Java: shape.setLineProperty(line);
         }
         // Java: return line;
         return None;
      }
    }

    // Java: SolidFill getSolidFill(Shape shape)
    pub fn getSolidFill(&self, _shape: &Element) -> Element {
      // Java: if (shape == null)
      if _shape.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: SolidFill fill = shape.getSolidFill();
         // TODO: Shape.getSolidFill not yet available
         // Java: if (fill == null)
         {
            // Java: fill = new SolidFill();
            // Java: shape.setFill(fill);
         }
         // Java: return fill;
         return None;
      }
    }

}
