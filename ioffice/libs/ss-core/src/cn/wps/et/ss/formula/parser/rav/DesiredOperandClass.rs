// Java field: byte[] r_desired_table
const R_DESIRED_TABLE: [i8; 9] = [0, 0, 0, 32, 64, 64, 64, 64, 64];
// Java field: byte[] v_desired_table
const V_DESIRED_TABLE: [i8; 9] = [32, 32, 64, 32, 32, 64, 32, 32, 64];
// Java field: byte[] a_desired_table
const A_DESIRED_TABLE: [i8; 9] = [64, 64, 64, 64, 64, 64, 64, 64, 64];
// Java field: byte[] x_desired_table
const X_DESIRED_TABLE: [i8; 9] = [0, 0, 0, 32, 32, 64, 32, 32, 64];
// Java field: byte[] y_desired_table
const Y_DESIRED_TABLE: [i8; 9] = [0, 0, 0, 64, 64, 64, 64, 64, 64];
// Java field: byte[] z_desired_table
const Z_DESIRED_TABLE: [i8; 9] = [0, 0, 0, 32, 32, 32, 32, 32, 32];
// Java field: byte[] l_desired_table
const L_DESIRED_TABLE: [i8; 9] = [32, 64, 64, 32, 64, 64, 64, 64, 64];
// Java field: byte[] m_desired_table
const M_DESIRED_TABLE: [i8; 9] = [32, 32, 32, 32, 32, 32, 32, 32, 32];
// Java field: byte[] n_desired_table
const N_DESIRED_TABLE: [i8; 9] = [0, 0, 0, 32, 32, 32, 32, 32, 32];

// Java: public enum DesiredOperandClass
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesiredOperandClass {
    R,
    V,
    A,
    X,
    Y,
    Z,
    L,
    M,
    N,
}

// Java: OperandClassTransformer.FormulaClass (inner enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormulaClass {
    CELL,
    ARRAY,
    NAME,
}

// Java: public enum DesiredOperandClass
impl DesiredOperandClass {
    // Java: static byte transform(OperandClassTransformer.FormulaClass fc, byte defaultOperandClass, DesiredOperandClass desiredOperandClass)
    pub fn transform(fc: FormulaClass, defaultOperandClass: i8, desiredOperandClass: DesiredOperandClass) -> i8 {
      // Java: int x = 0;
      let x: usize;
      // Java: switch (fc)
      match fc {
         // Java: case CELL:
         FormulaClass::CELL => {
            // Java: x = 0;
            x = 0;
            // Java: break;
         }
         // Java: case ARRAY:
         FormulaClass::ARRAY => {
            // Java: x = 1;
            x = 1;
            // Java: break;
         }
         // Java: case NAME:
         FormulaClass::NAME => {
            // Java: x = 2;
            x = 2;
            // Java: break;
         }
      }
      // Java: int y = 0;
      let y: usize;
      // Java: switch (defaultOperandClass)
      match defaultOperandClass {
         // Java: case 0:
         0 => {
            // Java: y = 0;
            y = 0;
            // Java: break;
         }
         // Java: case 32:
         32 => {
            // Java: y = 1;
            y = 1;
            // Java: break;
         }
         // Java: case 64:
         64 => {
            // Java: y = 2;
            y = 2;
            // Java: break;
         }
         // Java: default:
         _ => {
            // Java: throw new IllegalStateException();
            panic!("IllegalStateException");
         }
      }
      // Java: int pos = x + y * 3;
      let pos = x + y * 3;
      // Java: switch (desiredOperandClass)
      match desiredOperandClass {
         // Java: case R:
         DesiredOperandClass::R => {
            // Java: return r_desired_table[pos];
            R_DESIRED_TABLE[pos]
         }
         // Java: case V:
         DesiredOperandClass::V => {
            // Java: return v_desired_table[pos];
            V_DESIRED_TABLE[pos]
         }
         // Java: case A:
         DesiredOperandClass::A => {
            // Java: return a_desired_table[pos];
            A_DESIRED_TABLE[pos]
         }
         // Java: case X:
         DesiredOperandClass::X => {
            // Java: return x_desired_table[pos];
            X_DESIRED_TABLE[pos]
         }
         // Java: case Y:
         DesiredOperandClass::Y => {
            // Java: return y_desired_table[pos];
            Y_DESIRED_TABLE[pos]
         }
         // Java: case Z:
         DesiredOperandClass::Z => {
            // Java: return z_desired_table[pos];
            Z_DESIRED_TABLE[pos]
         }
         // Java: case L:
         DesiredOperandClass::L => {
            // Java: return l_desired_table[pos];
            L_DESIRED_TABLE[pos]
         }
         // Java: case M:
         DesiredOperandClass::M => {
            // Java: return m_desired_table[pos];
            M_DESIRED_TABLE[pos]
         }
         // Java: case N:
         DesiredOperandClass::N => {
            // Java: return n_desired_table[pos];
            N_DESIRED_TABLE[pos]
         }
      }
    }

}
