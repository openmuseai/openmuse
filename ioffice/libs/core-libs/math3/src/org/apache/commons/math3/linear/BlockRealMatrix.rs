// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
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
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: int BLOCK_SIZE
pub(crate) const BLOCK_SIZE: i32 = 36;
// Java field: long serialVersionUID


// Java: public class BlockRealMatrix extends AbstractRealMatrix implements Serializable
pub struct BlockRealMatrix {
    // Java field: int rows
    pub(crate) rows: i32,
    // Java field: int columns
    pub(crate) columns: i32,
    // Java field: int blockRows
    blockRows: i32,
    // Java field: int blockColumns
    blockColumns: i32,
}

// Java: public class BlockRealMatrix extends AbstractRealMatrix implements Serializable
impl BlockRealMatrix {
    // Java: BlockRealMatrix(final int rows, final int columns)
    pub fn BlockRealMatrix(&self) {
    }

    // Java: BlockRealMatrix(final double[][] rawData)
    pub fn BlockRealMatrix_2(&self) {
    }

    // Java: BlockRealMatrix()
    pub fn BlockRealMatrix_3(&self) {
    }

    // Java: static double[][] toBlocksLayout(final double[][] rawData)
    pub fn toBlocksLayout(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static double[][] createBlocksLayout(final int rows, final int columns)
    pub fn createBlocksLayout(&self) -> Box<dyn std::any::Any> {
        // Java: final int blockRows = (rows    + BLOCK_SIZE - 1) / BLOCK_SIZE;
        // Java: final int blockColumns = (columns + BLOCK_SIZE - 1) / BLOCK_SIZE;
        // Java: final double[][] blocks = new double[blockRows * blockColumns][];
        // Java: int blockIndex = 0;
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int pStart = iBlock * BLOCK_SIZE;
            // Java: final int pEnd = FastMath.min(pStart + BLOCK_SIZE, rows);
            // Java: final int iHeight = pEnd - pStart;
            // Java: for (int jBlock = 0; jBlock < blockColumns; ++jBlock)
            {
                // Java: final int qStart = jBlock * BLOCK_SIZE;
                // Java: final int qEnd = FastMath.min(qStart + BLOCK_SIZE, columns);
                // Java: final int jWidth = qEnd - qStart;
                // Java: blocks[blockIndex] = new double[iHeight * jWidth];
                // Java: ++blockIndex;
            }
        }
        // Java: return blocks;
        todo!()
    }

    // Java: BlockRealMatrix createMatrix()
    pub fn createMatrix(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: BlockRealMatrix copy()
    pub fn copy(&self) -> Box<dyn std::any::Any> {
        // Java: // create an empty matrix
        // Java: BlockRealMatrix copied = new BlockRealMatrix(rows, columns);
        // Java: // copy the blocks
        // Java: for (int i = 0; i < blocks.length; ++i)
        {
            // Java: System.arraycopy(blocks[i], 0, copied.blocks[i], 0, blocks[i].length);
        }
        // Java: return copied;
        todo!()
    }

    // Java: BlockRealMatrix add(final RealMatrix m)
    pub fn add(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: BlockRealMatrix add(final BlockRealMatrix m)
    pub fn add_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: BlockRealMatrix subtract(final RealMatrix m)
    pub fn subtract(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: BlockRealMatrix subtract(final BlockRealMatrix m)
    pub fn subtract_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: BlockRealMatrix scalarAdd(final double d)
    pub fn scalarAdd(&self) -> Box<dyn std::any::Any> {
        // Java: final BlockRealMatrix out = new BlockRealMatrix(rows, columns);
        // Java: // perform subtraction block-wise, to ensure good cache behavior
        // Java: for (int blockIndex = 0; blockIndex < out.blocks.length; ++blockIndex)
        {
            // Java: final double[] outBlock = out.blocks[blockIndex];
            // Java: final double[] tBlock = blocks[blockIndex];
            // Java: for (int k = 0; k < outBlock.length; ++k)
            {
                // Java: outBlock[k] = tBlock[k] + d;
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: RealMatrix scalarMultiply(final double d)
    pub fn scalarMultiply(&self) -> Box<dyn std::any::Any> {
        // Java: final BlockRealMatrix out = new BlockRealMatrix(rows, columns);
        // Java: // perform subtraction block-wise, to ensure good cache behavior
        // Java: for (int blockIndex = 0; blockIndex < out.blocks.length; ++blockIndex)
        {
            // Java: final double[] outBlock = out.blocks[blockIndex];
            // Java: final double[] tBlock = blocks[blockIndex];
            // Java: for (int k = 0; k < outBlock.length; ++k)
            {
                // Java: outBlock[k] = tBlock[k] * d;
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: BlockRealMatrix multiply(final RealMatrix m)
    pub fn multiply(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: BlockRealMatrix multiply(BlockRealMatrix m)
    pub fn multiply_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: double[][] getData()
    pub fn getData(&self) -> Box<dyn std::any::Any> {
        // Java: final double[][] data = new double[getRowDimension()][getColumnDimension()];
        // Java: final int lastColumns = columns - (blockColumns - 1) * BLOCK_SIZE;
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int pStart = iBlock * BLOCK_SIZE;
            // Java: final int pEnd = FastMath.min(pStart + BLOCK_SIZE, rows);
            // Java: int regularPos = 0;
            // Java: int lastPos = 0;
            // Java: for (int p = pStart; p < pEnd; ++p)
            {
                // Java: final double[] dataP = data[p];
                // Java: int blockIndex = iBlock * blockColumns;
                // Java: int dataPos = 0;
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

    // Java: double getNorm()
    pub fn getNorm(&self) -> f64 {
        // Java: final double[] colSums = new double[BLOCK_SIZE];
        // Java: double maxColSum = 0;
        // Java: for (int jBlock = 0; jBlock < blockColumns; jBlock++)
        {
            // Java: final int jWidth = blockWidth(jBlock);
            // Java: Arrays.fill(colSums, 0, jWidth, 0.0);
            // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
            {
                // Java: final int iHeight = blockHeight(iBlock);
                // Java: final double[] block = blocks[iBlock * blockColumns + jBlock];
                // Java: for (int j = 0; j < jWidth; ++j)
                {
                    // Java: double sum = 0;
                    // Java: for (int i = 0; i < iHeight; ++i)
                    {
                        // Java: sum += FastMath.abs(block[i * jWidth + j]);
                    }
                    // Java: colSums[j] += sum;
                }
            }
            // Java: for (int j = 0; j < jWidth; ++j)
            {
                // Java: maxColSum = FastMath.max(maxColSum, colSums[j]);
            }
        }
        // Java: return maxColSum;
        todo!()
    }

    // Java: double getFrobeniusNorm()
    pub fn getFrobeniusNorm(&self) -> f64 {
        // Java: double sum2 = 0;
        // Java: for (int blockIndex = 0; blockIndex < blocks.length; ++blockIndex)
        {
            // Java: for (final double entry : blocks[blockIndex])
            {
                // Java: sum2 += entry * entry;
            }
        }
        // Java: return FastMath.sqrt(sum2);
        todo!()
    }

    // Java: BlockRealMatrix getSubMatrix()
    pub fn getSubMatrix(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void copyBlockPart()
    pub fn copyBlockPart(&self) {
    }

    // Java: void setSubMatrix()
    pub fn setSubMatrix(&self) {
    }

    // Java: BlockRealMatrix getRowMatrix(final int row)
    pub fn getRowMatrix(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void setRowMatrix(final int row, final RealMatrix matrix)
    pub fn setRowMatrix(&self) {
    }

    // Java: void setRowMatrix(final int row, final BlockRealMatrix matrix)
    pub fn setRowMatrix_2(&self) {
    }

    // Java: BlockRealMatrix getColumnMatrix(final int column)
    pub fn getColumnMatrix(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void setColumnMatrix(final int column, final RealMatrix matrix)
    pub fn setColumnMatrix(&self) {
    }

    // Java: void setColumnMatrix(final int column, final BlockRealMatrix matrix)
    pub fn setColumnMatrix_2(&self) {
    }

    // Java: RealVector getRowVector(final int row)
    pub fn getRowVector(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void setRowVector(final int row, final RealVector vector)
    pub fn setRowVector(&self) {
    }

    // Java: RealVector getColumnVector(final int column)
    pub fn getColumnVector(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void setColumnVector(final int column, final RealVector vector)
    pub fn setColumnVector(&self) {
    }

    // Java: double[] getRow(final int row)
    pub fn getRow(&self) -> Box<dyn std::any::Any> {
        // Java: MatrixUtils.checkRowIndex(this, row);
        // Java: final double[] out = new double[columns];
        // Java: // perform copy block-wise, to ensure good cache behavior
        // Java: final int iBlock = row / BLOCK_SIZE;
        // Java: final int iRow = row - iBlock * BLOCK_SIZE;
        // Java: int outIndex = 0;
        // Java: for (int jBlock = 0; jBlock < blockColumns; ++jBlock)
        {
            // Java: final int jWidth     = blockWidth(jBlock);
            // Java: final double[] block = blocks[iBlock * blockColumns + jBlock];
            // Java: System.arraycopy(block, iRow * jWidth, out, outIndex, jWidth);
            // Java: outIndex += jWidth;
        }
        // Java: return out;
        todo!()
    }

    // Java: void setRow(final int row, final double[] array)
    pub fn setRow(&self) {
    }

    // Java: double[] getColumn(final int column)
    pub fn getColumn(&self) -> Box<dyn std::any::Any> {
        // Java: MatrixUtils.checkColumnIndex(this, column);
        // Java: final double[] out = new double[rows];
        // Java: // perform copy block-wise, to ensure good cache behavior
        // Java: final int jBlock  = column / BLOCK_SIZE;
        // Java: final int jColumn = column - jBlock * BLOCK_SIZE;
        // Java: final int jWidth  = blockWidth(jBlock);
        // Java: int outIndex = 0;
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int iHeight = blockHeight(iBlock);
            // Java: final double[] block = blocks[iBlock * blockColumns + jBlock];
            // Java: for (int i = 0; i < iHeight; ++i)
            {
                // Java: out[outIndex++] = block[i * jWidth + jColumn];
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: void setColumn(final int column, final double[] array)
    pub fn setColumn(&self) {
    }

    // Java: double getEntry(final int row, final int column)
    pub fn getEntry(&self) -> f64 {
        todo!()
    }

    // Java: void setEntry(final int row, final int column, final double value)
    pub fn setEntry(&self) {
    }

    // Java: void addToEntry()
    pub fn addToEntry(&self) {
    }

    // Java: void multiplyEntry()
    pub fn multiplyEntry(&self) {
    }

    // Java: BlockRealMatrix transpose()
    pub fn transpose(&self) -> Box<dyn std::any::Any> {
        // Java: final int nRows = getRowDimension();
        // Java: final int nCols = getColumnDimension();
        // Java: final BlockRealMatrix out = new BlockRealMatrix(nCols, nRows);
        // Java: // perform transpose block-wise, to ensure good cache behavior
        // Java: int blockIndex = 0;
        // Java: for (int iBlock = 0; iBlock < blockColumns; ++iBlock)
        {
            // Java: for (int jBlock = 0; jBlock < blockRows; ++jBlock)
            {
                // Java: // transpose current block
                // Java: final double[] outBlock = out.blocks[blockIndex];
                // Java: final double[] tBlock = blocks[jBlock * blockColumns + iBlock];
                // Java: final int pStart = iBlock * BLOCK_SIZE;
                // Java: final int pEnd = FastMath.min(pStart + BLOCK_SIZE, columns);
                // Java: final int qStart = jBlock * BLOCK_SIZE;
                // Java: final int qEnd = FastMath.min(qStart + BLOCK_SIZE, rows);
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

    // Java: double[] operate(final double[] v)
    pub fn operate(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: double[] preMultiply(final double[] v)
    pub fn preMultiply(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: double walkInRowOrder(final RealMatrixChangingVisitor visitor)
    pub fn walkInRowOrder(&self) -> f64 {
        // Java: visitor.start(rows, columns, 0, rows - 1, 0, columns - 1);
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int pStart = iBlock * BLOCK_SIZE;
            // Java: final int pEnd = FastMath.min(pStart + BLOCK_SIZE, rows);
            // Java: for (int p = pStart; p < pEnd; ++p)
            {
                // Java: for (int jBlock = 0; jBlock < blockColumns; ++jBlock)
                {
                    // Java: final int jWidth = blockWidth(jBlock);
                    // Java: final int qStart = jBlock * BLOCK_SIZE;
                    // Java: final int qEnd = FastMath.min(qStart + BLOCK_SIZE, columns);
                    // Java: final double[] block = blocks[iBlock * blockColumns + jBlock];
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

    // Java: double walkInRowOrder(final RealMatrixPreservingVisitor visitor)
    pub fn walkInRowOrder_2(&self) -> f64 {
        // Java: visitor.start(rows, columns, 0, rows - 1, 0, columns - 1);
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int pStart = iBlock * BLOCK_SIZE;
            // Java: final int pEnd = FastMath.min(pStart + BLOCK_SIZE, rows);
            // Java: for (int p = pStart; p < pEnd; ++p)
            {
                // Java: for (int jBlock = 0; jBlock < blockColumns; ++jBlock)
                {
                    // Java: final int jWidth = blockWidth(jBlock);
                    // Java: final int qStart = jBlock * BLOCK_SIZE;
                    // Java: final int qEnd = FastMath.min(qStart + BLOCK_SIZE, columns);
                    // Java: final double[] block = blocks[iBlock * blockColumns + jBlock];
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

    // Java: double walkInRowOrder()
    pub fn walkInRowOrder_3(&self) -> f64 {
        todo!()
    }

    // Java: double walkInRowOrder()
    pub fn walkInRowOrder_4(&self) -> f64 {
        todo!()
    }

    // Java: double walkInOptimizedOrder(final RealMatrixChangingVisitor visitor)
    pub fn walkInOptimizedOrder(&self) -> f64 {
        // Java: visitor.start(rows, columns, 0, rows - 1, 0, columns - 1);
        // Java: int blockIndex = 0;
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int pStart = iBlock * BLOCK_SIZE;
            // Java: final int pEnd = FastMath.min(pStart + BLOCK_SIZE, rows);
            // Java: for (int jBlock = 0; jBlock < blockColumns; ++jBlock)
            {
                // Java: final int qStart = jBlock * BLOCK_SIZE;
                // Java: final int qEnd = FastMath.min(qStart + BLOCK_SIZE, columns);
                // Java: final double[] block = blocks[blockIndex];
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

    // Java: double walkInOptimizedOrder(final RealMatrixPreservingVisitor visitor)
    pub fn walkInOptimizedOrder_2(&self) -> f64 {
        // Java: visitor.start(rows, columns, 0, rows - 1, 0, columns - 1);
        // Java: int blockIndex = 0;
        // Java: for (int iBlock = 0; iBlock < blockRows; ++iBlock)
        {
            // Java: final int pStart = iBlock * BLOCK_SIZE;
            // Java: final int pEnd = FastMath.min(pStart + BLOCK_SIZE, rows);
            // Java: for (int jBlock = 0; jBlock < blockColumns; ++jBlock)
            {
                // Java: final int qStart = jBlock * BLOCK_SIZE;
                // Java: final int qEnd = FastMath.min(qStart + BLOCK_SIZE, columns);
                // Java: final double[] block = blocks[blockIndex];
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

    // Java: double walkInOptimizedOrder()
    pub fn walkInOptimizedOrder_3(&self) -> f64 {
        todo!()
    }

    // Java: double walkInOptimizedOrder()
    pub fn walkInOptimizedOrder_4(&self) -> f64 {
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

