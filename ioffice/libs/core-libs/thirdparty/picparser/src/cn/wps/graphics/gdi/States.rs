#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables, unused_imports, unused_assignments)]

// Java: import android.graphics.Canvas
// Java: import android.graphics.Matrix
// Java: import android.graphics.Path
// Java: import android.graphics.Rect
// Java: import android.graphics.RectF
// Java: import android.graphics.Path.Direction
// Java: import android.graphics.Path.Op
// Java: import android.os.Build.VERSION
// Java: import java.util.Stack

pub struct Canvas {
    save_count: i32,
    matrix: Matrix,
    clip_bounds: Rect,
}

impl Canvas {
    pub fn getMatrix(&self) -> Matrix {
        self.matrix.clone()
    }
    pub fn getClipBounds(&self) -> Rect {
        self.clip_bounds.clone()
    }
    pub fn setMatrix(&mut self, m: &Matrix) {
        self.matrix = m.clone();
    }
    pub fn clipPath(&mut self, _path: &Path) {}
    pub fn clipOutPath(&mut self, _path: &Path) {}
    pub fn save(&mut self) {
        self.save_count += 1;
    }
    pub fn restore(&mut self) {
        if self.save_count > 0 {
            self.save_count -= 1;
        }
    }
}

#[derive(Clone)]
pub struct Matrix {
    values: [f32; 9],
}

impl Matrix {
    pub fn new() -> Self {
        Matrix { values: [0.0; 9] }
    }
    pub fn from_matrix(other: &Matrix) -> Self {
        other.clone()
    }
}

#[derive(Clone)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Clone)]
pub struct RectF {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    CW,
    CCW,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Op {
    INTERSECT,
    UNION,
    XOR,
    DIFFERENCE,
}

#[derive(Clone)]
pub struct Path {
    _data: Vec<u8>,
}

impl Path {
    pub fn new() -> Self {
        Path { _data: Vec::new() }
    }
    pub fn from_path(other: &Path) -> Self {
        other.clone()
    }
    pub fn addRect_f32(&mut self, _left: f32, _top: f32, _right: f32, _bottom: f32, _dir: Direction) {}
    pub fn addRect_rectf(&mut self, _rect: &RectF, _dir: Direction) {}
    pub fn op(&mut self, _other: &Path, _op: Op) {}
    pub fn reset(&mut self) {
        self._data.clear();
    }
}

const SDK_INT: i32 = 30;

// Java: public class States
pub struct States {
    // Java field: Canvas canvas
    canvas: Canvas,
    // Java field: State state
    pub(crate) state: State,
    // Java field: Stack<State> states
    states: Vec<State>,
}

// Java: public class State
#[derive(Clone)]
pub struct State {
    // Java field: Matrix matrix
    pub(crate) matrix: Matrix,
    // Java field: Path clip
    pub(crate) clip: Path,
}

// Java: public class States
impl States {
    // Java: States(Canvas canvas, Path clip)
    pub fn new(canvas: Canvas, clip: Option<&Path>) -> Self {
        // Java: super();
        // Java: this.canvas = canvas;
        // Java: this.state = new State();
        let mut state = State::new();
        // Java: if (clip != null)
        if let Some(c) = clip {
            // Java: this.state.clip = new Path(clip);
            state.clip = Path::from_path(c);
        }
        // Java: else
        else {
            // Java: Rect rect = canvas.getClipBounds();
            let rect = canvas.getClipBounds();
            // Java: this.state.clip = new Path();
            state.clip = Path::new();
            // Java: this.state.clip.addRect((float)rect.left, (float)rect.top, (float)rect.right, (float)rect.bottom, Direction.CW);
            state.clip.addRect_f32(rect.left as f32, rect.top as f32, rect.right as f32, rect.bottom as f32, Direction::CW);
        }
        let mut s = States {
            canvas,
            state,
            states: Vec::new(),
        };
        // Java: this.save();
        s.save();
        s
    }

    // Java: void save()
    pub fn save(&mut self) {
        // Java: this.state.matrix = this.canvas.getMatrix();
        self.state.matrix = self.canvas.getMatrix();
        // Java: if (this.states.size() == 1)
        if self.states.len() == 1 {
            // Java: Rect rect = this.canvas.getClipBounds();
            let rect = self.canvas.getClipBounds();
            // Java: this.state.clip.reset();
            self.state.clip.reset();
            // Java: this.state.clip.addRect((float)rect.left, (float)rect.top, (float)rect.right, (float)rect.bottom, Direction.CW);
            self.state.clip.addRect_f32(rect.left as f32, rect.top as f32, rect.right as f32, rect.bottom as f32, Direction::CW);
        }
        // Java: this.states.push(this.state);
        self.states.push(self.state.clone());
        // Java: this.state = new State(this.state);
        self.state = State::from_state(&self.state);
    }

    // Java: void restore()
    pub fn restore(&mut self) {
        // Java: this.state = (State)this.states.pop();
        self.state = self.states.pop().unwrap();
        // Java: int count = this.states.size();
        let count = self.states.len();
        // Java: for(int i = 0; i < count; ++i)
        for _i in 0..count {
            // Java: this.canvas.restore();
            self.canvas.restore();
        }
        // Java: for(int i = 0; i < count; ++i)
        for _i in 0..count {
            // Java: this.canvas.save();
            self.canvas.save();
        }
        // Java: this.canvas.setMatrix(this.state.matrix);
        self.canvas.setMatrix(&self.state.matrix);
        // Java: this.canvas.clipPath(this.state.clip);
        self.canvas.clipPath(&self.state.clip);
    }

    // Java: void clip(Path path, int mode)
    pub fn clip_path(&mut self, path: &Path, mode: i32) {
        // Java: this.state.clip.op(path, getOpByMode(mode));
        self.state.clip.op(path, States::getOpByMode(mode));
        // Java: if (mode == 1)
        if mode == 1 {
            // Java: this.canvas.clipPath(path);
            self.canvas.clipPath(path);
        }
        // Java: else if (mode == 4)
        else if mode == 4 {
            // Java: this.canvas.clipOutPath(path);
            self.canvas.clipOutPath(path);
        }
        // Java: else
        else {
            // Java: Matrix matrix = this.canvas.getMatrix();
            let matrix = self.canvas.getMatrix();
            // Java: int count = this.states.size();
            let count = self.states.len();
            // Java: for(int i = 0; i < count; ++i)
            for _i in 0..count {
                // Java: this.canvas.restore();
                self.canvas.restore();
            }
            // Java: for(int i = 0; i < count; ++i)
            for _i in 0..count {
                // Java: this.canvas.save();
                self.canvas.save();
            }
            // Java: this.canvas.setMatrix(matrix);
            self.canvas.setMatrix(&matrix);
            // Java: this.canvas.clipPath(this.state.clip);
            self.canvas.clipPath(&self.state.clip);
        }
    }

    // Java: void clip(RectF rect, int mode)
    pub fn clip_rect(&mut self, rect: &RectF, mode: i32) {
        // Java: Path path = new Path();
        let mut path = Path::new();
        // Java: path.addRect(rect, Direction.CW);
        path.addRect_rectf(rect, Direction::CW);
        // Java: this.clip(path, mode);
        self.clip_path(&path, mode);
    }

    // Java: static States getInstance(Canvas canvas, Path clip)
    pub fn getInstance(canvas: Canvas, clip: Option<&Path>) -> Option<States> {
        // Java: int ver = VERSION.SDK_INT;
        let ver = SDK_INT;
        // Java: return ver >= 28 ? new States(canvas, clip) : null;
        if ver >= 28 { Some(States::new(canvas, clip)) } else { None }
    }

    // Java: static Path.Op getOpByMode(int mode)
    pub fn getOpByMode(mode: i32) -> Op {
        // Java: switch (mode)
        match mode {
            // Java: case 1:
            1 => {
                // Java: return Op.INTERSECT;
                Op::INTERSECT
            }
            // Java: case 2:
            2 => {
                // Java: return Op.UNION;
                Op::UNION
            }
            // Java: case 3:
            3 => {
                // Java: return Op.XOR;
                Op::XOR
            }
            // Java: case 4:
            4 => {
                // Java: return Op.DIFFERENCE;
                Op::DIFFERENCE
            }
            // Java: case 5:
            5 => {
                // Java: return Op.UNION;
                Op::UNION
            }
            // Java: default:
            _ => {
                // Java: return Op.UNION;
                Op::UNION
            }
        }
    }
}

// Java: public class State
impl State {
    // Java: State()
    pub fn new() -> Self {
        // Java: super();
        State {
            matrix: Matrix::new(),
            clip: Path::new(),
        }
    }

    // Java: State(State state)
    pub fn from_state(state: &State) -> Self {
        // Java: super();
        // Java: this.matrix = new Matrix(state.matrix);
        // Java: this.clip = new Path(state.clip);
        State {
            matrix: Matrix::from_matrix(&state.matrix),
            clip: Path::from_path(&state.clip),
        }
    }
}
