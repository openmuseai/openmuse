// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
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
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: int BLOCK_SIZE
pub(crate) const BLOCK_SIZE: i32 = 36;
// Java field: long serialVersionUID

// Java: public class BlockFieldMatrix extends FieldElement implements Serializable
pub struct BlockFieldMatrix {
    // Java field: int rows
    pub(crate) rows: i32,
    // Java field: int columns
    pub(crate) columns: i32,
    // Java field: int blockRows
    blockRows: i32,
    // Java field: int blockColumns
    blockColumns: i32,
}

// Java: public class BlockFieldMatrix extends FieldElement implements Serializable
impl BlockFieldMatrix {
    // Java: BlockFieldMatrix()
    pub fn BlockFieldMatrix(&self) {
    }

    // Java: BlockFieldMatrix(final T[][] rawData)
    pub fn BlockFieldMatrix_2(&self) {
    }

    // Java: BlockFieldMatrix()
    pub fn BlockFieldMatrix_3(&self) {
    }

    // Java: static public static <T extends FieldElement<T>> T[][] toBlocksLayout(final T[][] rawData)
    pub fn toBlocksLayout(&self) -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

    // Java: static public static <T extends FieldElement<T>> T[][] createBlocksLayout()
    pub fn createBlocksLayout(&self) -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

    // Java: FieldMatrix<T> createMatrix()
    pub fn createMatrix(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: FieldMatrix<T> copy()
    pub fn copy(&self) -> Box<dyn std::any::Any> {
        // Java: // create an empty matrix
        // Java: BlockFieldMatrix<T> copied = new BlockFieldMatrix<T>(getField(), rows, columns);
        // Java: // copy the blocks
        // Java: for (int i = 0; i < blocks.length; ++i)
        {
            // Java: System.arraycopy(blocks[i], 0, copied.blocks[i], 0, blocks[i].length);
        }
        // Java: return copied;
        todo!()
    }

    // Java: FieldMatrix<T> add(final FieldMatrix<T> m)
    pub fn add(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: BlockFieldMatrix<T> add(final BlockFieldMatrix<T> m)
    pub fn add_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: FieldMatrix<T> subtract(final FieldMatrix<T> m)
    pub fn subtract(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: BlockFieldMatrix<T> subtract(final BlockFieldMatrix<T> m)
    pub fn subtract_2(&self) -> Box<dyn std::any::Any> {
        // Java: // safety check
        // Java: checkSubtractionCompatible(m);
        // Java: final BlockFieldMatrix<T> out = new BlockFieldMatrix<T>(getField(), rows, columns);
        // Java: // perform subtraction block-wise, to ensure good cache behavior
        // Java: for (int blockIndex = 0; blockIndex < out.blocks.length; ++blockIndex)
        {
            // Java: final T[] outBlock = out.blocks[blockIndex];
            // Java: final T[] tBlock   = blocks[blockIndex];
            // Java: final T[] mBlock   = m.blocks[blockIndex];
            // Java: for (int k = 0; k < outBlock.length; ++k)
            {
                // Java: outBlock[k] = tBlock[k].subtract(mBlock[k]);
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: FieldMatrix<T> scalarAdd(final T d)
    pub fn scalarAdd(&self) -> Box<dyn std::any::Any> {
        // Java: final BlockFieldMatrix<T> out = new BlockFieldMatrix<T>(getField(), rows, columns);
        // Java: // perform subtraction block-wise, to ensure good cache behavior
        // Java: for (int blockIndex = 0; blockIndex < out.blocks.length; ++blockIndex)
        {
            // Java: final T[] outBlock = out.blocks[blockIndex];
            // Java: final T[] tBlock   = blocks[blockIndex];
            // Java: for (int k = 0; k < outBlock.length; ++k)
            {
                // Java: outBlock[k] = tBlock[k].add(d);
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: FieldMatrix<T> scalarMultiply(final T d)
    pub fn scalarMultiply(&self) -> Box<dyn std::any::Any> {
        // Java: final BlockFieldMatrix<T> out = new BlockFieldMatrix<T>(getField(), rows, columns);
        // Java: // perform subtraction block-wise, to ensure good cache behavior
        // Java: for (int blockIndex = 0; blockIndex < out.blocks.length; ++blockIndex)
        {
            // Java: final T[] outBlock = out.blocks[blockIndex];
            // Java: final T[] tBlock   = blocks[blockIndex];
            // Java: for (int k = 0; k < outBlock.length; ++k)
            {
                // Java: outBlock[k] = tBlock[k].multiply(d);
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: FieldMatrix<T> multiply(final FieldMatrix<T> m)
    pub fn multiply(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: BlockFieldMatrix<T> multiply(BlockFieldMatrix<T> m)
    pub fn multiply_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T[][] getData()
    pub fn getData(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: final T[][] data = MathArrays.buildArray(getField(), getRowDimension(), getColumnDimension());
        // Java: final int lastColumns = columns - (blockColumns - 1) * BLOCK_SIZE;
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int pStart = iBlock * BLOCK_SIZE;
            // Java: final int pEnd   = FastMath.min(pStart + BLOCK_SIZE, rows);
            // Java: int regularPos   = 0;
            // Java: int lastPos      = 0;
            // Java: for (int p = pStart; p < pEnd; ++p)
            {
                // Java: final T[] dataP = data[p];
                // Java: int blockIndex = iBlock * blockColumns;
                // Java: int dataPos    = 0;
                // Java: for (int jBlock = 0; jBlock < blockColumns - 1; ++jBlock)
                {
                    // Java: System.arraycopy(blocks[blockIndex++], regularPos, dataP, dataPos, BLOCK_SIZE);
                    // Java: dataPos += BLOCK_SIZE;
                }
                // Java: System.arraycopy(blocks[blockIndex], lastPos, dataP, dataPos, lastColumns);
                // Java: regularPos += BLOCK_SIZE;
                // Java: lastPos    += lastColumns;
            }
        }
        // Java: return data;
        todo!()
    }

    // Java: FieldMatrix<T> getSubMatrix()
    pub fn getSubMatrix(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void copyBlockPart()
    pub fn copyBlockPart(&self) {
    }

    // Java: void setSubMatrix()
    pub fn setSubMatrix(&self) {
    }

    // Java: FieldMatrix<T> getRowMatrix(final int row)
    pub fn getRowMatrix(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void setRowMatrix(final int row, final FieldMatrix<T> matrix)
    pub fn setRowMatrix(&self) {
    }

    // Java: void setRowMatrix(final int row, final BlockFieldMatrix<T> matrix)
    pub fn setRowMatrix_2(&self) {
    }

    // Java: FieldMatrix<T> getColumnMatrix(final int column)
    pub fn getColumnMatrix(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void setColumnMatrix(final int column, final FieldMatrix<T> matrix)
    pub fn setColumnMatrix(&self) {
    }

    // Java: void setColumnMatrix(final int column, final BlockFieldMatrix<T> matrix)
    pub fn setColumnMatrix_2(&self) {
    }

    // Java: FieldVector<T> getRowVector(final int row)
    pub fn getRowVector(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void setRowVector(final int row, final FieldVector<T> vector)
    pub fn setRowVector(&self) {
    }

    // Java: FieldVector<T> getColumnVector(final int column)
    pub fn getColumnVector(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void setColumnVector(final int column, final FieldVector<T> vector)
    pub fn setColumnVector(&self) {
    }

    // Java: T[] getRow(final int row)
    pub fn getRow(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: checkRowIndex(row);
        // Java: final T[] out = MathArrays.buildArray(getField(), columns);
        // Java: // perform copy block-wise, to ensure good cache behavior
        // Java: final int iBlock  = row / BLOCK_SIZE;
        // Java: final int iRow    = row - iBlock * BLOCK_SIZE;
        // Java: int outIndex      = 0;
        // Java: for (int jBlock = 0; jBlock < blockColumns; ++jBlock)
        {
            // Java: final int jWidth     = blockWidth(jBlock);
            // Java: final T[] block = blocks[iBlock * blockColumns + jBlock];
            // Java: System.arraycopy(block, iRow * jWidth, out, outIndex, jWidth);
            // Java: outIndex += jWidth;
        }
        // Java: return out;
        todo!()
    }

    // Java: void setRow(final int row, final T[] array)
    pub fn setRow(&self) {
    }

    // Java: T[] getColumn(final int column)
    pub fn getColumn(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: checkColumnIndex(column);
        // Java: final T[] out = MathArrays.buildArray(getField(), rows);
        // Java: // perform copy block-wise, to ensure good cache behavior
        // Java: final int jBlock  = column / BLOCK_SIZE;
        // Java: final int jColumn = column - jBlock * BLOCK_SIZE;
        // Java: final int jWidth  = blockWidth(jBlock);
        // Java: int outIndex      = 0;
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int iHeight = blockHeight(iBlock);
            // Java: final T[] block = blocks[iBlock * blockColumns + jBlock];
            // Java: for (int i = 0; i < iHeight; ++i)
            {
                // Java: out[outIndex++] = block[i * jWidth + jColumn];
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: void setColumn(final int column, final T[] array)
    pub fn setColumn(&self) {
    }

    // Java: T getEntry(final int row, final int column)
    pub fn getEntry(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void setEntry(final int row, final int column, final T value)
    pub fn setEntry(&self) {
    }

    // Java: void addToEntry(final int row, final int column, final T increment)
    pub fn addToEntry(&self) {
    }

    // Java: void multiplyEntry(final int row, final int column, final T factor)
    pub fn multiplyEntry(&self) {
    }

    // Java: FieldMatrix<T> transpose()
    pub fn transpose(&self) -> Box<dyn std::any::Any> {
        // Java: final int nRows = getRowDimension();
        // Java: final int nCols = getColumnDimension();
        // Java: final BlockFieldMatrix<T> out = new BlockFieldMatrix<T>(getField(), nCols, nRows);
        // Java: // perform transpose block-wise, to ensure good cache behavior
        // Java: int blockIndex = 0;
        // Java: for (int iBlock = 0; iBlock < blockColumns; ++iBlock)
        {
            // Java: for (int jBlock = 0; jBlock < blockRows; ++jBlock)
            {
                // Java: // transpose current block
                // Java: final T[] outBlock = out.blocks[blockIndex];
                // Java: final T[] tBlock   = blocks[jBlock * blockColumns + iBlock];
                // Java: final int      pStart   = iBlock * BLOCK_SIZE;
                // Java: final int      pEnd     = FastMath.min(pStart + BLOCK_SIZE, columns);
                // Java: final int      qStart   = jBlock * BLOCK_SIZE;
                // Java: final int      qEnd     = FastMath.min(qStart + BLOCK_SIZE, rows);
                // Java: int k = 0;
                // Java: for (int p = pStart; p < pEnd; ++p)
                {
                    // Java: final int lInc = pEnd - pStart;
                    // Java: int l = p - pStart;
                    // Java: for (int q = qStart; q < qEnd; ++q)
                    {
                        // Java: outBlock[k] = tBlock[l];
                        // Java: ++k;
                        // Java: l+= lInc;
                    }
                }
                // Java: // go to next block
                // Java: ++blockIndex;
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: int getRowDimension()
    pub fn getRowDimension(&self) -> i32 {
        // Java: return rows;
        todo!()
    }

    // Java: int getColumnDimension()
    pub fn getColumnDimension(&self) -> i32 {
        // Java: return columns;
        todo!()
    }

    // Java: T[] operate(final T[] v)
    pub fn operate(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: if (v.length != columns)
        {
            // Java: throw new DimensionMismatchException(v.length, columns);
        }
        // Java: final T[] out = MathArrays.buildArray(getField(), rows);
        // Java: final T zero = getField().getZero();
        // Java: // perform multiplication block-wise, to ensure good cache behavior
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int pStart = iBlock * BLOCK_SIZE;
            // Java: final int pEnd   = FastMath.min(pStart + BLOCK_SIZE, rows);
            // Java: for (int jBlock = 0; jBlock < blockColumns; ++jBlock)
            {
                // Java: final T[] block  = blocks[iBlock * blockColumns + jBlock];
                // Java: final int      qStart = jBlock * BLOCK_SIZE;
                // Java: final int      qEnd   = FastMath.min(qStart + BLOCK_SIZE, columns);
                // Java: int k = 0;
                // Java: for (int p = pStart; p < pEnd; ++p)
                {
                    // Java: T sum = zero;
                    // Java: int q = qStart;
                    // Java: while (q < qEnd - 3)
                    {
                        // Java: sum = sum.
                              // Java: add(block[k].multiply(v[q])).
                              // Java: add(block[k + 1].multiply(v[q + 1])).
                              // Java: add(block[k + 2].multiply(v[q + 2])).
                              // Java: add(block[k + 3].multiply(v[q + 3]));
                        // Java: k += 4;
                        // Java: q += 4;
                    }
                    // Java: while (q < qEnd)
                    {
                        // Java: sum = sum.add(block[k++].multiply(v[q++]));
                    }
                    // Java: out[p] = out[p].add(sum);
                }
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: T[] preMultiply(final T[] v)
    pub fn preMultiply(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: if (v.length != rows)
        {
            // Java: throw new DimensionMismatchException(v.length, rows);
        }
        // Java: final T[] out = MathArrays.buildArray(getField(), columns);
        // Java: final T zero = getField().getZero();
        // Java: // perform multiplication block-wise, to ensure good cache behavior
        // Java: for (int jBlock = 0; jBlock < blockColumns; ++jBlock)
        {
            // Java: final int jWidth  = blockWidth(jBlock);
            // Java: final int jWidth2 = jWidth  + jWidth;
            // Java: final int jWidth3 = jWidth2 + jWidth;
            // Java: final int jWidth4 = jWidth3 + jWidth;
            // Java: final int qStart = jBlock * BLOCK_SIZE;
            // Java: final int qEnd   = FastMath.min(qStart + BLOCK_SIZE, columns);
            // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
            {
                // Java: final T[] block  = blocks[iBlock * blockColumns + jBlock];
                // Java: final int      pStart = iBlock * BLOCK_SIZE;
                // Java: final int      pEnd   = FastMath.min(pStart + BLOCK_SIZE, rows);
                // Java: for (int q = qStart; q < qEnd; ++q)
                {
                    // Java: int k = q - qStart;
                    // Java: T sum = zero;
                    // Java: int p = pStart;
                    // Java: while (p < pEnd - 3)
                    {
                        // Java: sum = sum.
                              // Java: add(block[k].multiply(v[p])).
                              // Java: add(block[k + jWidth].multiply(v[p + 1])).
                              // Java: add(block[k + jWidth2].multiply(v[p + 2])).
                              // Java: add(block[k + jWidth3].multiply(v[p + 3]));
                        // Java: k += jWidth4;
                        // Java: p += 4;
                    }
                    // Java: while (p < pEnd)
                    {
                        // Java: sum = sum.add(block[k].multiply(v[p++]));
                        // Java: k += jWidth;
                    }
                    // Java: out[q] = out[q].add(sum);
                }
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: T walkInRowOrder(final FieldMatrixChangingVisitor<T> visitor)
    pub fn walkInRowOrder(&self) -> Box<dyn std::any::Any> {
        // Java: visitor.start(rows, columns, 0, rows - 1, 0, columns - 1);
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int pStart = iBlock * BLOCK_SIZE;
            // Java: final int pEnd   = FastMath.min(pStart + BLOCK_SIZE, rows);
            // Java: for (int p = pStart; p < pEnd; ++p)
            {
                // Java: for (int jBlock = 0; jBlock < blockColumns; ++jBlock)
                {
                    // Java: final int jWidth = blockWidth(jBlock);
                    // Java: final int qStart = jBlock * BLOCK_SIZE;
                    // Java: final int qEnd   = FastMath.min(qStart + BLOCK_SIZE, columns);
                    // Java: final T[] block = blocks[iBlock * blockColumns + jBlock];
                    // Java: int k = (p - pStart) * jWidth;
                    // Java: for (int q = qStart; q < qEnd; ++q)
                    {
                        // Java: block[k] = visitor.visit(p, q, block[k]);
                        // Java: ++k;
                    }
                }
            }
        }
        // Java: return visitor.end();
        todo!()
    }

    // Java: T walkInRowOrder(final FieldMatrixPreservingVisitor<T> visitor)
    pub fn walkInRowOrder_2(&self) -> Box<dyn std::any::Any> {
        // Java: visitor.start(rows, columns, 0, rows - 1, 0, columns - 1);
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int pStart = iBlock * BLOCK_SIZE;
            // Java: final int pEnd   = FastMath.min(pStart + BLOCK_SIZE, rows);
            // Java: for (int p = pStart; p < pEnd; ++p)
            {
                // Java: for (int jBlock = 0; jBlock < blockColumns; ++jBlock)
                {
                    // Java: final int jWidth = blockWidth(jBlock);
                    // Java: final int qStart = jBlock * BLOCK_SIZE;
                    // Java: final int qEnd   = FastMath.min(qStart + BLOCK_SIZE, columns);
                    // Java: final T[] block = blocks[iBlock * blockColumns + jBlock];
                    // Java: int k = (p - pStart) * jWidth;
                    // Java: for (int q = qStart; q < qEnd; ++q)
                    {
                        // Java: visitor.visit(p, q, block[k]);
                        // Java: ++k;
                    }
                }
            }
        }
        // Java: return visitor.end();
        todo!()
    }

    // Java: T walkInRowOrder()
    pub fn walkInRowOrder_3(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T walkInRowOrder()
    pub fn walkInRowOrder_4(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T walkInOptimizedOrder(final FieldMatrixChangingVisitor<T> visitor)
    pub fn walkInOptimizedOrder(&self) -> Box<dyn std::any::Any> {
        // Java: visitor.start(rows, columns, 0, rows - 1, 0, columns - 1);
        // Java: int blockIndex = 0;
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int pStart = iBlock * BLOCK_SIZE;
            // Java: final int pEnd   = FastMath.min(pStart + BLOCK_SIZE, rows);
            // Java: for (int jBlock = 0; jBlock < blockColumns; ++jBlock)
            {
                // Java: final int qStart = jBlock * BLOCK_SIZE;
                // Java: final int qEnd   = FastMath.min(qStart + BLOCK_SIZE, columns);
                // Java: final T[] block = blocks[blockIndex];
                // Java: int k = 0;
                // Java: for (int p = pStart; p < pEnd; ++p)
                {
                    // Java: for (int q = qStart; q < qEnd; ++q)
                    {
                        // Java: block[k] = visitor.visit(p, q, block[k]);
                        // Java: ++k;
                    }
                }
                // Java: ++blockIndex;
            }
        }
        // Java: return visitor.end();
        todo!()
    }

    // Java: T walkInOptimizedOrder(final FieldMatrixPreservingVisitor<T> visitor)
    pub fn walkInOptimizedOrder_2(&self) -> Box<dyn std::any::Any> {
        // Java: visitor.start(rows, columns, 0, rows - 1, 0, columns - 1);
        // Java: int blockIndex = 0;
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int pStart = iBlock * BLOCK_SIZE;
            // Java: final int pEnd   = FastMath.min(pStart + BLOCK_SIZE, rows);
            // Java: for (int jBlock = 0; jBlock < blockColumns; ++jBlock)
            {
                // Java: final int qStart = jBlock * BLOCK_SIZE;
                // Java: final int qEnd   = FastMath.min(qStart + BLOCK_SIZE, columns);
                // Java: final T[] block = blocks[blockIndex];
                // Java: int k = 0;
                // Java: for (int p = pStart; p < pEnd; ++p)
                {
                    // Java: for (int q = qStart; q < qEnd; ++q)
                    {
                        // Java: visitor.visit(p, q, block[k]);
                        // Java: ++k;
                    }
                }
                // Java: ++blockIndex;
            }
        }
        // Java: return visitor.end();
        todo!()
    }

    // Java: T walkInOptimizedOrder()
    pub fn walkInOptimizedOrder_3(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T walkInOptimizedOrder()
    pub fn walkInOptimizedOrder_4(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: int blockHeight(final int blockRow)
    pub fn blockHeight(&self) -> i32 {
        // Java: return (blockRow == blockRows - 1) ? rows - blockRow * BLOCK_SIZE : BLOCK_SIZE;
        todo!()
    }

    // Java: int blockWidth(final int blockColumn)
    pub fn blockWidth(&self) -> i32 {
        // Java: return (blockColumn == blockColumns - 1) ? columns - blockColumn * BLOCK_SIZE : BLOCK_SIZE;
        todo!()
    }

}

