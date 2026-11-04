// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;

// Java: public interface RealMatrix extends AnyMatrix
pub trait RealMatrix {
    // Java field: throws NotStrictlyPositiveException
    // Java field: throws MatrixDimensionMismatchException
    // Java field: throws MatrixDimensionMismatchException
    // Java field: throws DimensionMismatchException
    // Java field: throws DimensionMismatchException
    // Java field: throws NotPositiveException, NonSquareMatrixException
    // Java field: throws OutOfRangeException, NumberIsTooSmallException
    // Java field: throws NullArgumentException, NoDataException, OutOfRangeException
    // Java field: throws OutOfRangeException, NumberIsTooSmallException
    // Java field: MatrixDimensionMismatchException
    // Java field: throws OutOfRangeException, NullArgumentException, NoDataException
    // Java field: MatrixDimensionMismatchException
    // Java field: throws NoDataException, OutOfRangeException
    // Java field: DimensionMismatchException, NullArgumentException
    // Java field: throws OutOfRangeException, MatrixDimensionMismatchException
    // Java field: throws OutOfRangeException
    // Java field: throws OutOfRangeException, MatrixDimensionMismatchException
    // Java field: throws OutOfRangeException
    // Java field: throws OutOfRangeException, MatrixDimensionMismatchException
    // Java field: throws OutOfRangeException
    // Java field: throws OutOfRangeException, MatrixDimensionMismatchException
    // Java field: throws OutOfRangeException, MatrixDimensionMismatchException
    // Java field: throws OutOfRangeException, MatrixDimensionMismatchException
    // Java field: throws OutOfRangeException, NumberIsTooSmallException
    // Java field: throws OutOfRangeException, NumberIsTooSmallException
    // Java field: throws OutOfRangeException, NumberIsTooSmallException
    // Java field: throws OutOfRangeException, NumberIsTooSmallException
    // Java field: throws OutOfRangeException, NumberIsTooSmallException
    // Java field: throws OutOfRangeException, NumberIsTooSmallException
    // Java: abstract RealMatrix createMatrix(int rowDimension, int columnDimension)
    fn createMatrix(&self, rowDimension: i32, columnDimension: i32) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix copy()
    fn copy(&self) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix add(RealMatrix m)
    fn add(&self, m: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix subtract(RealMatrix m)
    fn subtract(&self, m: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix scalarAdd(double d)
    fn scalarAdd(&self, d: f64) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix scalarMultiply(double d)
    fn scalarMultiply(&self, d: f64) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix multiply(RealMatrix m)
    fn multiply(&self, m: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix preMultiply(RealMatrix m)
    fn preMultiply(&self, m: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix power(final int p)
    fn power(&self, p: i32) -> Box<dyn std::any::Any>;

    // Java: abstract double[][] getData()
    fn getData(&self) -> Vec<Vec<f64>>;

    // Java: abstract double getNorm()
    fn getNorm(&self) -> f64;

    // Java: abstract double getFrobeniusNorm()
    fn getFrobeniusNorm(&self) -> f64;

    // Java: abstract RealMatrix getSubMatrix()
    fn getSubMatrix(&self) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix getSubMatrix(int[] selectedRows, int[] selectedColumns)
    fn getSubMatrix_1(&self, selectedRows: &[i32], selectedColumns: &[i32]) -> Box<dyn std::any::Any>;

    // Java: abstract void copySubMatrix()
    fn copySubMatrix(&self);

    // Java: abstract void copySubMatrix()
    fn copySubMatrix_1(&self);

    // Java: abstract void setSubMatrix(double[][] subMatrix, int row, int column)
    fn setSubMatrix(&self, subMatrix: &mut Vec<Vec<f64>>, row: i32, column: i32);

    // Java: abstract RealMatrix getRowMatrix(int row)
    fn getRowMatrix(&self, row: i32) -> Box<dyn std::any::Any>;

    // Java: abstract void setRowMatrix(int row, RealMatrix matrix)
    fn setRowMatrix(&self, row: i32, matrix: &dyn std::any::Any);

    // Java: abstract RealMatrix getColumnMatrix(int column)
    fn getColumnMatrix(&self, column: i32) -> Box<dyn std::any::Any>;

    // Java: abstract void setColumnMatrix(int column, RealMatrix matrix)
    fn setColumnMatrix(&self, column: i32, matrix: &dyn std::any::Any);

    // Java: abstract RealVector getRowVector(int row)
    fn getRowVector(&self, row: i32) -> Box<dyn std::any::Any>;

    // Java: abstract void setRowVector(int row, RealVector vector)
    fn setRowVector(&self, row: i32, vector: &dyn std::any::Any);

    // Java: abstract RealVector getColumnVector(int column)
    fn getColumnVector(&self, column: i32) -> Box<dyn std::any::Any>;

    // Java: abstract void setColumnVector(int column, RealVector vector)
    fn setColumnVector(&self, column: i32, vector: &dyn std::any::Any);

    // Java: abstract double[] getRow(int row)
    fn getRow(&self, row: i32) -> Vec<f64>;

    // Java: abstract void setRow(int row, double[] array)
    fn setRow(&self, row: i32, array: &[f64]);

    // Java: abstract double[] getColumn(int column)
    fn getColumn(&self, column: i32) -> Vec<f64>;

    // Java: abstract void setColumn(int column, double[] array)
    fn setColumn(&self, column: i32, array: &[f64]);

    // Java: abstract double getEntry(int row, int column)
    fn getEntry(&self, row: i32, column: i32) -> f64;

    // Java: abstract void setEntry(int row, int column, double value)
    fn setEntry(&self, row: i32, column: i32, value: f64);

    // Java: abstract void addToEntry(int row, int column, double increment)
    fn addToEntry(&self, row: i32, column: i32, increment: f64);

    // Java: abstract void multiplyEntry(int row, int column, double factor)
    fn multiplyEntry(&self, row: i32, column: i32, factor: f64);

    // Java: abstract RealMatrix transpose()
    fn transpose(&self) -> Box<dyn std::any::Any>;

    // Java: abstract double getTrace()
    fn getTrace(&self) -> f64;

    // Java: abstract double[] operate(double[] v)
    fn operate(&self, v: &[f64]) -> Vec<f64>;

    // Java: abstract RealVector operate(RealVector v)
    fn operate_1(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract double[] preMultiply(double[] v)
    fn preMultiply_1(&self, v: &[f64]) -> Vec<f64>;

    // Java: abstract RealVector preMultiply(RealVector v)
    fn preMultiply_2(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract double walkInRowOrder(RealMatrixChangingVisitor visitor)
    fn walkInRowOrder(&self, visitor: &dyn std::any::Any) -> f64;

    // Java: abstract double walkInRowOrder(RealMatrixPreservingVisitor visitor)
    fn walkInRowOrder_1(&self, visitor: &dyn std::any::Any) -> f64;

    // Java: abstract double walkInRowOrder()
    fn walkInRowOrder_2(&self) -> f64;

    // Java: abstract double walkInRowOrder()
    fn walkInRowOrder_3(&self) -> f64;

    // Java: abstract double walkInColumnOrder(RealMatrixChangingVisitor visitor)
    fn walkInColumnOrder(&self, visitor: &dyn std::any::Any) -> f64;

    // Java: abstract double walkInColumnOrder(RealMatrixPreservingVisitor visitor)
    fn walkInColumnOrder_1(&self, visitor: &dyn std::any::Any) -> f64;

    // Java: abstract double walkInColumnOrder()
    fn walkInColumnOrder_2(&self) -> f64;

    // Java: abstract double walkInColumnOrder()
    fn walkInColumnOrder_3(&self) -> f64;

    // Java: abstract double walkInOptimizedOrder(RealMatrixChangingVisitor visitor)
    fn walkInOptimizedOrder(&self, visitor: &dyn std::any::Any) -> f64;

    // Java: abstract double walkInOptimizedOrder(RealMatrixPreservingVisitor visitor)
    fn walkInOptimizedOrder_1(&self, visitor: &dyn std::any::Any) -> f64;

    // Java: abstract double walkInOptimizedOrder()
    fn walkInOptimizedOrder_2(&self) -> f64;

    // Java: abstract double walkInOptimizedOrder()
    fn walkInOptimizedOrder_3(&self) -> f64;

}
