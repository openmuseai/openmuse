// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
use std::sync::{Arc, Mutex};

// Java field: int ADDITIVE_MODE
pub(crate) const ADDITIVE_MODE: i32 = 0;
// Java field: int MULTIPLICATIVE_MODE
pub(crate) const MULTIPLICATIVE_MODE: i32 = 0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;
// Java field: int DEFAULT_INITIAL_CAPACITY
pub(crate) const DEFAULT_INITIAL_CAPACITY: i32 = 0;
// Java field: double DEFAULT_EXPANSION_FACTOR
pub(crate) const DEFAULT_EXPANSION_FACTOR: f64 = 0.0;
// Java field: double DEFAULT_CONTRACTION_DELTA
pub(crate) const DEFAULT_CONTRACTION_DELTA: f64 = 0.0;

// Java: public class is
pub struct is {
}

// Java: public class ResizableDoubleArray implements DoubleArray, Serializable
pub struct ResizableDoubleArray {
    // Java field: double contractionCriterion
    pub(crate) contractionCriterion: f64,
    // Java field: double expansionFactor
    expansionFactor: f64,
    // Java field: ExpansionMode expansionMode
    expansionMode: Option<ExpansionMode>,
    // Java field: double[] internalArray
    pub(crate) internalArray: Arc<Mutex<Vec<f64>>>,
    // Java field: int numElements
    pub(crate) numElements: i32,
    // Java field: int startIndex
    pub(crate) startIndex: i32,
}

// Java: public enum ExpansionMode
pub struct ExpansionMode {
}

// Java: public class is
impl is {
}

// Java: public class ResizableDoubleArray implements DoubleArray, Serializable
impl ResizableDoubleArray {
    // Java: ResizableDoubleArray()
    pub fn ResizableDoubleArray(&self) {
        // Java: this(DEFAULT_INITIAL_CAPACITY);
    }

    // Java: ResizableDoubleArray(int initialCapacity)
    pub fn ResizableDoubleArray_1(&self, initialCapacity: i32) {
    }

    // Java: ResizableDoubleArray(double[] initialArray)
    pub fn ResizableDoubleArray_2(&self, initialArray: &[f64]) {
        // Java: this(DEFAULT_INITIAL_CAPACITY,
             // Java: DEFAULT_EXPANSION_FACTOR,
             // Java: DEFAULT_CONTRACTION_DELTA + DEFAULT_EXPANSION_FACTOR,
             // Java: ExpansionMode.MULTIPLICATIVE,
             // Java: initialArray);
    }

    // Java: ResizableDoubleArray()
    pub fn ResizableDoubleArray_3(&self) {
    }

    // Java: ResizableDoubleArray()
    pub fn ResizableDoubleArray_4(&self) {
    }

    // Java: ResizableDoubleArray()
    pub fn ResizableDoubleArray_5(&self) {
    }

    // Java: ResizableDoubleArray()
    pub fn ResizableDoubleArray_6(&self) {
    }

    // Java: ResizableDoubleArray()
    pub fn ResizableDoubleArray_7(&self) {
    }

    // Java: ResizableDoubleArray()
    pub fn ResizableDoubleArray_8(&self) {
    }

    // Java: ResizableDoubleArray(ResizableDoubleArray original)
    pub fn ResizableDoubleArray_9(&self, original: &ResizableDoubleArray) {
    }

    // Java: void addElement(double value)
    pub fn addElement(&self, value: f64) {
        // Java: if (internalArray.length <= startIndex + numElements)
        {
            // Java: expand();
        }
        // Java: internalArray[startIndex + numElements++] = value;
    }

    // Java: void addElements(double[] values)
    pub fn addElements(&self, values: &[f64]) {
        // Java: final double[] tempArray = new double[numElements + values.length + 1];
        // Java: System.arraycopy(internalArray, startIndex, tempArray, 0, numElements);
        // Java: System.arraycopy(values, 0, tempArray, numElements, values.length);
        // Java: internalArray = tempArray;
        // Java: startIndex = 0;
        // Java: numElements += values.length;
    }

    // Java: double addElementRolling(double value)
    pub fn addElementRolling(&self, value: f64) -> f64 {
        // Java: double discarded = internalArray[startIndex];
        // Java: if ((startIndex + (numElements + 1)) > internalArray.length)
        {
            // Java: expand();
        }
        // Java: // Increment the start index
        // Java: startIndex += 1;
        // Java: // Add the new value
        // Java: internalArray[startIndex + (numElements - 1)] = value;
        // Java: // Check the contraction criterion.
        // Java: if (shouldContract())
        {
            // Java: contract();
        }
        // Java: return discarded;
        todo!();
    }

    // Java: double substituteMostRecentElement(double value)
    pub fn substituteMostRecentElement(&self, value: f64) -> f64 {
        todo!();
    }

    // Java: void checkContractExpand(float contraction, float expansion)
    pub fn checkContractExpand(&self, contraction: f32, expansion: f32) {
    }

    // Java: void checkContractExpand()
    pub fn checkContractExpand_1(&self) {
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: numElements = 0;
        // Java: startIndex = 0;
    }

    // Java: void contract()
    pub fn contract(&self) {
        // Java: final double[] tempArray = new double[numElements + 1];
        // Java: // Copy and swap - copy only the element array from the src array.
        // Java: System.arraycopy(internalArray, startIndex, tempArray, 0, numElements);
        // Java: internalArray = tempArray;
        // Java: // Reset the start index to zero
        // Java: startIndex = 0;
    }

    // Java: void discardFrontElements(int i)
    pub fn discardFrontElements(&self, i: i32) {
    }

    // Java: void discardMostRecentElements(int i)
    pub fn discardMostRecentElements(&self, i: i32) {
    }

    // Java: void discardExtremeElements()
    pub fn discardExtremeElements(&self) {
    }

    // Java: void expand()
    pub fn expand(&self) {
        // Java: // notice the use of FastMath.ceil(), this guarantees that we will always
        // Java: // have an array of at least currentSize + 1.   Assume that the
        // Java: // current initial capacity is 1 and the expansion factor
        // Java: // is 1.000000000000000001.  The newly calculated size will be
        // Java: // rounded up to 2 after the multiplication is performed.
        // Java: int newSize = 0;
        // Java: if (expansionMode == ExpansionMode.MULTIPLICATIVE)
        {
            // Java: newSize = (int) FastMath.ceil(internalArray.length * expansionFactor);
        }
        // Java: else
        {
            // Java: newSize = (int) (internalArray.length + FastMath.round(expansionFactor));
        }
        // Java: final double[] tempArray = new double[newSize];
        // Java: // Copy and swap
        // Java: System.arraycopy(internalArray, 0, tempArray, 0, internalArray.length);
        // Java: internalArray = tempArray;
    }

    // Java: void expandTo(int size)
    pub fn expandTo(&self, size: i32) {
        // Java: final double[] tempArray = new double[size];
        // Java: // Copy and swap
        // Java: System.arraycopy(internalArray, 0, tempArray, 0, internalArray.length);
        // Java: internalArray = tempArray;
    }

    // Java: float getContractionCriteria()
    pub fn getContractionCriteria(&self) -> f32 {
        // Java: return (float) getContractionCriterion();
        todo!();
    }

    // Java: double getContractionCriterion()
    pub fn getContractionCriterion(&self) -> f64 {
        // Java: return contractionCriterion;
        todo!();
    }

    // Java: double getElement(int index)
    pub fn getElement(&self, index: i32) -> f64 {
        // Java: if (index >= numElements)
        {
            // Java: throw new ArrayIndexOutOfBoundsException(index);
        }
        // Java: else if (index >= 0)
        {
            // Java: return internalArray[startIndex + index];
        }
        // Java: else
        {
            // Java: throw new ArrayIndexOutOfBoundsException(index);
        }
        todo!();
    }

    // Java: double[] getElements()
    pub fn getElements(&self) -> [f64; 2] {
        // Java: final double[] elementArray = new double[numElements];
        // Java: System.arraycopy(internalArray, startIndex, elementArray, 0, numElements);
        // Java: return elementArray;
        todo!();
    }

    // Java: float getExpansionFactor()
    pub fn getExpansionFactor(&self) -> f32 {
        // Java: return (float) expansionFactor;
        todo!();
    }

    // Java: int getExpansionMode()
    pub fn getExpansionMode(&self) -> i32 {
        // Java: synchronized (this)
        {
            // Java: switch (expansionMode)
            {
                // Java: case MULTIPLICATIVE:
                    // Java: return MULTIPLICATIVE_MODE;
                // Java: case ADDITIVE:
                    // Java: return ADDITIVE_MODE;
                // Java: default:
                    // Java: throw new MathInternalError(); // Should never happen.
            }
        }
        todo!();
    }

    // Java: int getInternalLength()
    pub fn getInternalLength(&self) -> i32 {
        // Java: return internalArray.length;
        todo!();
    }

    // Java: int getCapacity()
    pub fn getCapacity(&self) -> i32 {
        // Java: return internalArray.length;
        todo!();
    }

    // Java: int getNumElements()
    pub fn getNumElements(&self) -> i32 {
        // Java: return numElements;
        todo!();
    }

    // Java: double[] getInternalValues()
    pub fn getInternalValues(&self) -> [f64; 2] {
        // Java: return internalArray;
        todo!();
    }

    // Java: double[] getArrayRef()
    pub fn getArrayRef(&self) -> [f64; 2] {
        // Java: return internalArray;
        todo!();
    }

    // Java: int getStartIndex()
    pub fn getStartIndex(&self) -> i32 {
        // Java: return startIndex;
        todo!();
    }

    // Java: void setContractionCriteria(float contractionCriteria)
    pub fn setContractionCriteria(&self, contractionCriteria: f32) {
    }

    // Java: double compute(MathArrays.Function f)
    pub fn compute(&self, f: &dyn Fn(&[f64], i32, i32) -> f64) -> f64 {
        // Java: final double[] array;
        // Java: final int start;
        // Java: final int num;
        // Java: synchronized(this)
        {
            // Java: array = internalArray;
            // Java: start = startIndex;
            // Java: num   = numElements;
        }
        // Java: return f.evaluate(array, start, num);
        todo!();
    }

    // Java: void setElement(int index, double value)
    pub fn setElement(&self, index: i32, value: f64) {
        // Java: if (index < 0)
        {
            // Java: throw new ArrayIndexOutOfBoundsException(index);
        }
        // Java: if (index + 1 > numElements)
        {
            // Java: numElements = index + 1;
        }
        // Java: if ((startIndex + index) >= internalArray.length)
        {
            // Java: expandTo(startIndex + (index + 1));
        }
        // Java: internalArray[startIndex + index] = value;
    }

    // Java: void setExpansionFactor(float expansionFactor)
    pub fn setExpansionFactor(&self, expansionFactor: f32) {
        // Java: checkContractExpand(getContractionCriterion(), expansionFactor);
        // Java: // The check above verifies that the expansion factor is > 1.0;
        // Java: synchronized(this)
        {
            // Java: this.expansionFactor = expansionFactor;
        }
    }

    // Java: void setExpansionMode(int expansionMode)
    pub fn setExpansionMode(&self, expansionMode: i32) {
    }

    // Java: void setExpansionMode(ExpansionMode expansionMode)
    pub fn setExpansionMode_1(&self, expansionMode: &ExpansionMode) {
        // Java: synchronized(this)
        {
            // Java: this.expansionMode = expansionMode;
        }
    }

    // Java: void setInitialCapacity(int initialCapacity)
    pub fn setInitialCapacity(&self, initialCapacity: i32) {
    }

    // Java: void setNumElements(int i)
    pub fn setNumElements(&self, i: i32) {
    }

    // Java: boolean shouldContract()
    pub fn shouldContract(&self) -> bool {
        // Java: if (expansionMode == ExpansionMode.MULTIPLICATIVE)
        {
            // Java: return (internalArray.length / ((float) numElements)) > contractionCriterion;
        }
        // Java: else
        {
            // Java: return (internalArray.length - numElements) > contractionCriterion;
        }
        todo!();
    }

    // Java: int start()
    pub fn start(&self) -> i32 {
        // Java: return startIndex;
        todo!();
    }

    // Java: static void copy()
    pub fn copy(&self) {
    }

    // Java: ResizableDoubleArray copy()
    pub fn copy_1(&self) -> ResizableDoubleArray {
        // Java: final ResizableDoubleArray result = new ResizableDoubleArray();
        // Java: copy(this, result);
        // Java: return result;
        todo!();
    }

    // Java: boolean equals(Object object)
    pub fn equals(&self, object: &dyn std::any::Any) -> bool {
        // Java: if (object == this )
        {
            // Java: return true;
        }
        // Java: if (object instanceof ResizableDoubleArray == false)
        {
            // Java: return false;
        }
        // Java: synchronized(this)
        {
            // Java: synchronized(object)
            {
                // Java: boolean result = true;
                // Java: final ResizableDoubleArray other = (ResizableDoubleArray) object;
                // Java: result = result && (other.contractionCriterion == contractionCriterion);
                // Java: result = result && (other.expansionFactor == expansionFactor);
                // Java: result = result && (other.expansionMode == expansionMode);
                // Java: result = result && (other.numElements == numElements);
                // Java: result = result && (other.startIndex == startIndex);
                // Java: if (!result)
                {
                    // Java: return false;
                }
                // Java: else
                {
                    // Java: return Arrays.equals(internalArray, other.internalArray);
                }
            }
        }
        todo!();
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: final int[] hashData = new int[6];
        // Java: hashData[0] = Double.valueOf(expansionFactor).hashCode();
        // Java: hashData[1] = Double.valueOf(contractionCriterion).hashCode();
        // Java: hashData[2] = expansionMode.hashCode();
        // Java: hashData[3] = Arrays.hashCode(internalArray);
        // Java: hashData[4] = numElements;
        // Java: hashData[5] = startIndex;
        // Java: return Arrays.hashCode(hashData);
        todo!();
    }

}

// Java: public enum ExpansionMode
impl ExpansionMode {
}

impl super::DoubleArray::DoubleArray for ResizableDoubleArray {
    fn getNumElements(&self) -> i32 { todo!(); }
    fn getElement(&self, index: i32) -> f64 { todo!(); }
    fn setElement(&mut self, index: i32, value: f64) { todo!(); }
    fn addElement(&mut self, value: f64) { todo!(); }
    fn addElements(&mut self, values: &[f64]) { todo!(); }
    fn addElementRolling(&mut self, value: f64) -> f64 { todo!(); }
    fn getElements(&self) -> Vec<f64> { todo!(); }
    fn clear(&mut self) { todo!(); }
}
