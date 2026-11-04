#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

// Java: import android.graphics.Path
// Java: import java.util.ArrayList

#[derive(Clone, Copy, PartialEq)]
pub enum FillType {
    WINDING,
    EVEN_ODD,
    INVERSE_WINDING,
    INVERSE_EVEN_ODD,
}

#[derive(Clone)]
pub struct PathElem {
    pub(crate) kind: i32,
    pub(crate) points: Vec<f32>,
}

impl PathElem {
    pub fn toString(&self) -> String {
        format!("PathElem(kind={}, points={:?})", self.kind, self.points)
    }
}

// Java: public class GraphicsPath
#[derive(Clone)]
pub struct GraphicsPath {
    // Java field: Path.FillType fillType
    fillType: Option<FillType>,
    // Java field: boolean needStroke
    pub(crate) needStroke: bool,
    // Java field: ArrayList<PathElem> elems
    pub(crate) elems: Vec<PathElem>,
}

// Java: public class GraphicsPath
impl GraphicsPath {
    // Java: GraphicsPath()
    pub fn new() -> Self {
        // Java: super();
        GraphicsPath {
            fillType: None,
            needStroke: false,
            elems: Vec::new(),
        }
    }

    // Java: void setNeedStroke(boolean value)
    pub fn setNeedStroke(&mut self, value: bool) {
        // Java: this.needStroke = value;
        self.needStroke = value;
    }

    // Java: boolean getNeedSroke()
    pub fn getNeedSroke(&self) -> bool {
        // Java: return this.needStroke;
        self.needStroke
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.elems.size();
        self.elems.len() as i32
    }

    pub fn getFillType(&self) -> Option<FillType> {
        self.fillType
    }

    // Java: void setFillType(Path.FillType fillType)
    pub fn setFillType(&mut self, fillType: FillType) {
        // Java: this.fillType = fillType;
        self.fillType = Some(fillType);
    }

    // Java: PathElem getElem(int index)
    pub fn getElem(&self, index: i32) -> &PathElem {
        // Java: return (PathElem)this.elems.get(index);
        &self.elems[index as usize]
    }

    // Java: void addElem(PathElem pathElem)
    pub fn addElem(&mut self, pathElem: PathElem) {
        // Java: this.elems.add(pathElem);
        self.elems.push(pathElem);
    }

    // Java: void removeElem(int index)
    pub fn removeElem(&mut self, index: i32) {
        // Java: this.elems.remove(index);
        self.elems.remove(index as usize);
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.elems.clear();
        self.elems.clear();
    }

    // Java: void addPath(GraphicsPath path)
    pub fn addPath(&mut self, path: GraphicsPath) {
        // Java: for(PathElem elem : path.elems)
        for elem in path.elems {
            // Java: this.addElem(elem);
            self.elems.push(elem);
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: String value = "";
        let mut value = String::new();
        // Java: int size = this.size();
        let size = self.size();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: PathElem elem = this.getElem(i);
            let elem = self.getElem(i);
            // Java: value = value + elem.toString() + "\n";
            value.push_str(&elem.toString());
            value.push('\n');
        }
        // Java: return value;
        value
    }
}
