// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;
// Java: import org.apache.commons.math3.util.OpenIntToFieldHashMap
use crate::org::apache::commons::math3::util::OpenIntToFieldHashMap;

// Java: public class SparseFieldMatrix extends FieldElement
pub struct SparseFieldMatrix {
    // Java field: OpenIntToFieldHashMap<T> entries
    pub(crate) entries: Box<dyn std::any::Any>,
    // Java field: int rows
    pub(crate) rows: i32,
    // Java field: int columns
    pub(crate) columns: i32,
}

// Java: public class SparseFieldMatrix extends FieldElement
impl SparseFieldMatrix {
    // Java: SparseFieldMatrix(final Field<T> field)
    pub fn SparseFieldMatrix(&self) {
        // Java: super(field);
        // Java: rows = 0;
        // Java: columns= 0;
        // Java: entries = new OpenIntToFieldHashMap<T>(field);
    }

    // Java: SparseFieldMatrix()
    pub fn SparseFieldMatrix_2(&self) {
    }

    // Java: SparseFieldMatrix(SparseFieldMatrix<T> other)
    pub fn SparseFieldMatrix_3(&self) {
        // Java: super(other.getField(), other.getRowDimension(), other.getColumnDimension());
        // Java: rows = other.getRowDimension();
        // Java: columns = other.getColumnDimension();
        // Java: entries = new OpenIntToFieldHashMap<T>(other.entries);
    }

    // Java: SparseFieldMatrix(FieldMatrix<T> other)
    pub fn SparseFieldMatrix_4(&self) {
        // Java: super(other.getField(), other.getRowDimension(), other.getColumnDimension());
        // Java: rows = other.getRowDimension();
        // Java: columns = other.getColumnDimension();
        // Java: entries = new OpenIntToFieldHashMap<T>(getField());
        // Java: for (int i = 0; i < rows; i++)
        {
            // Java: for (int j = 0; j < columns; j++)
            {
                // Java: setEntry(i, j, other.getEntry(i, j));
            }
        }
    }

    // Java: void addToEntry(int row, int column, T increment)
    pub fn addToEntry(&self) {
        // Java: checkRowIndex(row);
        // Java: checkColumnIndex(column);
        // Java: final int key = computeKey(row, column);
        // Java: final T value = entries.get(key).add(increment);
        // Java: if (getField().getZero().equals(value))
        {
            // Java: entries.remove(key);
        }
        // Java: else
        {
            // Java: entries.put(key, value);
        }
    }

    // Java: FieldMatrix<T> copy()
    pub fn copy(&self) -> Box<dyn std::any::Any> {
        // Java: return new SparseFieldMatrix<T>(this);
        todo!()
    }

    // Java: FieldMatrix<T> createMatrix(int rowDimension, int columnDimension)
    pub fn createMatrix(&self) -> Box<dyn std::any::Any> {
        // Java: return new SparseFieldMatrix<T>(getField(), rowDimension, columnDimension);
        todo!()
    }

    // Java: int getColumnDimension()
    pub fn getColumnDimension(&self) -> i32 {
        // Java: return columns;
        todo!()
    }

    // Java: T getEntry(int row, int column)
    pub fn getEntry(&self) -> Box<dyn std::any::Any> {
        // Java: checkRowIndex(row);
        // Java: checkColumnIndex(column);
        // Java: return entries.get(computeKey(row, column));
        todo!()
    }

    // Java: int getRowDimension()
    pub fn getRowDimension(&self) -> i32 {
        // Java: return rows;
        todo!()
    }

    // Java: void multiplyEntry(int row, int column, T factor)
    pub fn multiplyEntry(&self) {
        // Java: checkRowIndex(row);
        // Java: checkColumnIndex(column);
        // Java: final int key = computeKey(row, column);
        // Java: final T value = entries.get(key).multiply(factor);
        // Java: if (getField().getZero().equals(value))
        {
            // Java: entries.remove(key);
        }
        // Java: else
        {
            // Java: entries.put(key, value);
        }
    }

    // Java: void setEntry(int row, int column, T value)
    pub fn setEntry(&self) {
        // Java: checkRowIndex(row);
        // Java: checkColumnIndex(column);
        // Java: if (getField().getZero().equals(value))
        {
            // Java: entries.remove(computeKey(row, column));
        }
        // Java: else
        {
            // Java: entries.put(computeKey(row, column), value);
        }
    }

    // Java: int computeKey(int row, int column)
    pub fn computeKey(&self) -> i32 {
        // Java: return row * columns + column;
        todo!()
    }

}
