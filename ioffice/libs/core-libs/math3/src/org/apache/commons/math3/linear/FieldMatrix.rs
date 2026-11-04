// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field::Field;
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement::FieldElement;
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

// Java: public interface FieldMatrix extends FieldElement
pub trait FieldMatrix<T: Clone> {
    // Java field: throws NotStrictlyPositiveException
    // Java field: throws NumberIsTooSmallException, OutOfRangeException
    // Java field: throws NoDataException, NullArgumentException, OutOfRangeException
    // Java field: throws MatrixDimensionMismatchException, NumberIsTooSmallException
    // Java field: OutOfRangeException
    // Java field: throws MatrixDimensionMismatchException, NoDataException, NullArgumentException
    // Java field: OutOfRangeException
    // Java field: throws DimensionMismatchException, OutOfRangeException
    // Java field: NoDataException, NullArgumentException
    // Java field: throws MatrixDimensionMismatchException, OutOfRangeException
    // Java field: throws MatrixDimensionMismatchException, OutOfRangeException
    // Java field: throws MatrixDimensionMismatchException, OutOfRangeException
    // Java field: throws MatrixDimensionMismatchException, OutOfRangeException
    // Java field: throws OutOfRangeException, NumberIsTooSmallException
    // Java field: throws OutOfRangeException, NumberIsTooSmallException
    // Java field: throws NumberIsTooSmallException, OutOfRangeException
    // Java field: throws NumberIsTooSmallException, OutOfRangeException
    // Java field: throws NumberIsTooSmallException, OutOfRangeException
    // Java field: throws NumberIsTooSmallException, OutOfRangeException
    // Java: abstract Field<T> getField()
    fn getField(&self) -> std::any::TypeId;

    // Java: abstract FieldMatrix<T> createMatrix(final int rowDimension, final int columnDimension)
    fn createMatrix(&self, rowDimension: i32, columnDimension: i32) -> Box<dyn std::any::Any>;

    // Java: abstract FieldMatrix<T> copy()
    fn copy(&self) -> Box<dyn std::any::Any>;

    // Java: abstract FieldMatrix<T> add(FieldMatrix<T> m)
    fn add(&self, m: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldMatrix<T> subtract(FieldMatrix<T> m)
    fn subtract(&self, m: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldMatrix<T> scalarAdd(T d)
    fn scalarAdd(&self, d: T) -> Box<dyn std::any::Any>;

    // Java: abstract FieldMatrix<T> scalarMultiply(T d)
    fn scalarMultiply(&self, d: T) -> Box<dyn std::any::Any>;

    // Java: abstract FieldMatrix<T> multiply(FieldMatrix<T> m)
    fn multiply(&self, m: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldMatrix<T> preMultiply(FieldMatrix<T> m)
    fn preMultiply(&self, m: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldMatrix<T> power(final int p)
    fn power(&self, p: i32) -> Box<dyn std::any::Any>;

    // Java: abstract T[][] getData()
    fn getData(&self) -> Vec<Vec<T>>;

    // Java: abstract FieldMatrix<T> getSubMatrix(int startRow, int endRow, int startColumn, int endColumn)
    fn getSubMatrix(&self, startRow: i32, endRow: i32, startColumn: i32, endColumn: i32) -> Box<dyn std::any::Any>;

    // Java: abstract FieldMatrix<T> getSubMatrix(int[] selectedRows, int[] selectedColumns)
    fn getSubMatrix_1(&self, selectedRows: &[i32], selectedColumns: &[i32]) -> Box<dyn std::any::Any>;

    // Java: abstract void copySubMatrix()
    fn copySubMatrix(&self);

    // Java: abstract void copySubMatrix(int[] selectedRows, int[] selectedColumns, T[][] destination)
    fn copySubMatrix_1(&self, selectedRows: &[i32], selectedColumns: &[i32], destination: &mut Vec<Vec<T>>);

    // Java: abstract void setSubMatrix(T[][] subMatrix, int row, int column)
    fn setSubMatrix(&self, subMatrix: &mut Vec<Vec<T>>, row: i32, column: i32);

    // Java: abstract FieldMatrix<T> getRowMatrix(int row)
    fn getRowMatrix(&self, row: i32) -> Box<dyn std::any::Any>;

    // Java: abstract void setRowMatrix(int row, FieldMatrix<T> matrix)
    fn setRowMatrix(&self, row: i32, matrix: &dyn std::any::Any);

    // Java: abstract FieldMatrix<T> getColumnMatrix(int column)
    fn getColumnMatrix(&self, column: i32) -> Box<dyn std::any::Any>;

    // Java: abstract void setColumnMatrix(int column, FieldMatrix<T> matrix)
    fn setColumnMatrix(&self, column: i32, matrix: &dyn std::any::Any);

    // Java: abstract FieldVector<T> getRowVector(int row)
    fn getRowVector(&self, row: i32) -> Vec<T>;

    // Java: abstract void setRowVector(int row, FieldVector<T> vector)
    fn setRowVector(&self, row: i32, vector: &dyn std::any::Any);

    // Java: abstract FieldVector<T> getColumnVector(int column)
    fn getColumnVector(&self, column: i32) -> Vec<T>;

    // Java: abstract void setColumnVector(int column, FieldVector<T> vector)
    fn setColumnVector(&self, column: i32, vector: &dyn std::any::Any);

    // Java: abstract T[] getRow(int row)
    fn getRow(&self, row: i32) -> Vec<T>;

    // Java: abstract void setRow(int row, T[] array)
    fn setRow(&self, row: i32, array: &[T]);

    // Java: abstract T[] getColumn(int column)
    fn getColumn(&self, column: i32) -> Vec<T>;

    // Java: abstract void setColumn(int column, T[] array)
    fn setColumn(&self, column: i32, array: &[T]);

    // Java: abstract T getEntry(int row, int column)
    fn getEntry(&self, row: i32, column: i32) -> T;

    // Java: abstract void setEntry(int row, int column, T value)
    fn setEntry(&self, row: i32, column: i32, value: T);

    // Java: abstract void addToEntry(int row, int column, T increment)
    fn addToEntry(&self, row: i32, column: i32, increment: T);

    // Java: abstract void multiplyEntry(int row, int column, T factor)
    fn multiplyEntry(&self, row: i32, column: i32, factor: T);

    // Java: abstract FieldMatrix<T> transpose()
    fn transpose(&self) -> Box<dyn std::any::Any>;

    // Java: abstract T getTrace()
    fn getTrace(&self) -> T;

    // Java: abstract T[] operate(T[] v)
    fn operate(&self, v: &[T]) -> Vec<T>;

    // Java: abstract FieldVector<T> operate(FieldVector<T> v)
    fn operate_1(&self, v: &dyn std::any::Any) -> Vec<T>;

    // Java: abstract T[] preMultiply(T[] v)
    fn preMultiply_1(&self, v: &[T]) -> Vec<T>;

    // Java: abstract FieldVector<T> preMultiply(FieldVector<T> v)
    fn preMultiply_2(&self, v: &dyn std::any::Any) -> Vec<T>;

    // Java: abstract T walkInRowOrder(FieldMatrixChangingVisitor<T> visitor)
    fn walkInRowOrder(&self, visitor: &dyn std::any::Any) -> T;

    // Java: abstract T walkInRowOrder(FieldMatrixPreservingVisitor<T> visitor)
    fn walkInRowOrder_1(&self, visitor: &dyn std::any::Any) -> T;

    // Java: abstract T walkInRowOrder()
    fn walkInRowOrder_2(&self) -> T;

    // Java: abstract T walkInRowOrder()
    fn walkInRowOrder_3(&self) -> T;

    // Java: abstract T walkInColumnOrder(FieldMatrixChangingVisitor<T> visitor)
    fn walkInColumnOrder(&self, visitor: &dyn std::any::Any) -> T;

    // Java: abstract T walkInColumnOrder(FieldMatrixPreservingVisitor<T> visitor)
    fn walkInColumnOrder_1(&self, visitor: &dyn std::any::Any) -> T;

    // Java: abstract T walkInColumnOrder()
    fn walkInColumnOrder_2(&self) -> T;

    // Java: abstract T walkInColumnOrder()
    fn walkInColumnOrder_3(&self) -> T;

    // Java: abstract T walkInOptimizedOrder(FieldMatrixChangingVisitor<T> visitor)
    fn walkInOptimizedOrder(&self, visitor: &dyn std::any::Any) -> T;

    // Java: abstract T walkInOptimizedOrder(FieldMatrixPreservingVisitor<T> visitor)
    fn walkInOptimizedOrder_1(&self, visitor: &dyn std::any::Any) -> T;

    // Java: abstract T walkInOptimizedOrder()
    fn walkInOptimizedOrder_2(&self) -> T;

    // Java: abstract T walkInOptimizedOrder()
    fn walkInOptimizedOrder_3(&self) -> T;

}
