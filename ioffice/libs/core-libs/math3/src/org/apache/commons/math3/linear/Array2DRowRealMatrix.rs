// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class Array2DRowRealMatrix extends AbstractRealMatrix implements Serializable
pub struct Array2DRowRealMatrix {
}

// Java: public class Array2DRowRealMatrix extends AbstractRealMatrix implements Serializable
impl Array2DRowRealMatrix {
    // Java: Array2DRowRealMatrix()
    pub fn Array2DRowRealMatrix(&self) {
    }

    // Java: Array2DRowRealMatrix()
    pub fn Array2DRowRealMatrix_1(&self) {
    }

    // Java: Array2DRowRealMatrix(final double[][] d)
    pub fn Array2DRowRealMatrix_2(&self) {
    }

    // Java: Array2DRowRealMatrix(final double[][] d, final boolean copyArray)
    pub fn Array2DRowRealMatrix_3(&self, copyArray: bool) {
    }

    // Java: Array2DRowRealMatrix(final double[] v)
    pub fn Array2DRowRealMatrix_4(&self, v: &[f64]) {
        // Java: final int nRows = v.length;
        // Java: data = new double[nRows][1];
        // Java: for (int row = 0; row < nRows; row++)
        {
            // Java: data[row][0] = v[row];
        }
    }

    // Java: RealMatrix createMatrix()
    pub fn createMatrix(&self) -> Box<dyn std::any::Any> {
        todo!();
    }

    // Java: RealMatrix copy()
    pub fn copy(&self) -> Box<dyn std::any::Any> {
        // Java: return new Array2DRowRealMatrix(copyOut(), false);
        todo!();
    }

    // Java: Array2DRowRealMatrix add(final Array2DRowRealMatrix m)
    pub fn add(&self, m: &Array2DRowRealMatrix) -> Array2DRowRealMatrix {
        todo!();
    }

    // Java: Array2DRowRealMatrix subtract(final Array2DRowRealMatrix m)
    pub fn subtract(&self, m: &Array2DRowRealMatrix) -> Array2DRowRealMatrix {
        todo!();
    }

    // Java: Array2DRowRealMatrix multiply(final Array2DRowRealMatrix m)
    pub fn multiply(&self, m: &Array2DRowRealMatrix) -> Array2DRowRealMatrix {
        todo!();
    }

    // Java: double[][] getData()
    pub fn getData(&self) -> Vec<Vec<f64>> {
        // Java: return copyOut();
        todo!();
    }

    // Java: double[][] getDataRef()
    pub fn getDataRef(&self) -> Vec<Vec<f64>> {
        // Java: return data;
        todo!();
    }

    // Java: void setSubMatrix()
    pub fn setSubMatrix(&self) {
    }

    // Java: double getEntry(final int row, final int column)
    pub fn getEntry(&self, row: i32, column: i32) -> f64 {
        todo!();
    }

    // Java: void setEntry(final int row, final int column, final double value)
    pub fn setEntry(&self, row: i32, column: i32, value: f64) {
    }

    // Java: void addToEntry()
    pub fn addToEntry(&self) {
    }

    // Java: void multiplyEntry()
    pub fn multiplyEntry(&self) {
    }

    // Java: int getRowDimension()
    pub fn getRowDimension(&self) -> i32 {
        // Java: return (data == null) ? 0 : data.length;
        todo!();
    }

    // Java: int getColumnDimension()
    pub fn getColumnDimension(&self) -> i32 {
        // Java: return ((data == null) || (data[0] == null)) ? 0 : data[0].length;
        todo!();
    }

    // Java: double[] operate(final double[] v)
    pub fn operate(&self, v: &[f64]) -> Vec<f64> {
        todo!();
    }

    // Java: double[] preMultiply(final double[] v)
    pub fn preMultiply(&self, v: &[f64]) -> Vec<f64> {
        todo!();
    }

    // Java: double walkInRowOrder(final RealMatrixChangingVisitor visitor)
    pub fn walkInRowOrder(&self, visitor: &dyn std::any::Any) -> f64 {
        // Java: final int rows    = getRowDimension();
        // Java: final int columns = getColumnDimension();
        // Java: visitor.start(rows, columns, 0, rows - 1, 0, columns - 1);
        // Java: for (int i = 0; i < rows; ++i)
        {
            // Java: final double[] rowI = data[i];
            // Java: for (int j = 0; j < columns; ++j)
            {
                // Java: rowI[j] = visitor.visit(i, j, rowI[j]);
            }
        }
        // Java: return visitor.end();
        todo!();
    }

    // Java: double walkInRowOrder(final RealMatrixPreservingVisitor visitor)
    pub fn walkInRowOrder_1(&self, visitor: &dyn std::any::Any) -> f64 {
        // Java: final int rows    = getRowDimension();
        // Java: final int columns = getColumnDimension();
        // Java: visitor.start(rows, columns, 0, rows - 1, 0, columns - 1);
        // Java: for (int i = 0; i < rows; ++i)
        {
            // Java: final double[] rowI = data[i];
            // Java: for (int j = 0; j < columns; ++j)
            {
                // Java: visitor.visit(i, j, rowI[j]);
            }
        }
        // Java: return visitor.end();
        todo!();
    }

    // Java: double walkInRowOrder()
    pub fn walkInRowOrder_2(&self) -> f64 {
        todo!();
    }

    // Java: double walkInRowOrder()
    pub fn walkInRowOrder_3(&self) -> f64 {
        todo!();
    }

    // Java: double walkInColumnOrder(final RealMatrixChangingVisitor visitor)
    pub fn walkInColumnOrder(&self, visitor: &dyn std::any::Any) -> f64 {
        // Java: final int rows    = getRowDimension();
        // Java: final int columns = getColumnDimension();
        // Java: visitor.start(rows, columns, 0, rows - 1, 0, columns - 1);
        // Java: for (int j = 0; j < columns; ++j)
        {
            // Java: for (int i = 0; i < rows; ++i)
            {
                // Java: final double[] rowI = data[i];
                // Java: rowI[j] = visitor.visit(i, j, rowI[j]);
            }
        }
        // Java: return visitor.end();
        todo!();
    }

    // Java: double walkInColumnOrder(final RealMatrixPreservingVisitor visitor)
    pub fn walkInColumnOrder_1(&self, visitor: &dyn std::any::Any) -> f64 {
        // Java: final int rows    = getRowDimension();
        // Java: final int columns = getColumnDimension();
        // Java: visitor.start(rows, columns, 0, rows - 1, 0, columns - 1);
        // Java: for (int j = 0; j < columns; ++j)
        {
            // Java: for (int i = 0; i < rows; ++i)
            {
                // Java: visitor.visit(i, j, data[i][j]);
            }
        }
        // Java: return visitor.end();
        todo!();
    }

    // Java: double walkInColumnOrder()
    pub fn walkInColumnOrder_2(&self) -> f64 {
        todo!();
    }

    // Java: double walkInColumnOrder()
    pub fn walkInColumnOrder_3(&self) -> f64 {
        todo!();
    }

    // Java: double[][] copyOut()
    pub fn copyOut(&self) -> Vec<Vec<f64>> {
        // Java: final int nRows = this.getRowDimension();
        // Java: final double[][] out = new double[nRows][this.getColumnDimension()];
        // Java: // can't copy 2-d array in one shot, otherwise get row references
        // Java: for (int i = 0; i < nRows; i++)
        {
            // Java: System.arraycopy(data[i], 0, out[i], 0, data[i].length);
        }
        // Java: return out;
        todo!();
    }

    // Java: void copyIn(final double[][] in)
    pub fn copyIn(&self) {
    }

}

// impl Serializable for Array2DRowRealMatrix {
// }
