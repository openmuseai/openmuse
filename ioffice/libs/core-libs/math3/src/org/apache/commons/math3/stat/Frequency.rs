// Java: import java.io.Serializable
// Java: import java.text.NumberFormat
// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Iterator
// Java: import java.util.Comparator
// Java: import java.util.List
// Java: import java.util.Map
// Java: import java.util.Map.Entry
// Java: import java.util.SortedMap
// Java: import java.util.TreeMap
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID
// Java field: long serialVersionUID

// Java: public class Frequency implements Serializable
pub struct Frequency {
    // Java field: SortedMap<Comparable<?>, Long> freqTable
    pub(crate) freqTable: Box<dyn std::any::Any>,
}

// Java: public class NaturalComparator
pub struct NaturalComparator {
}

// Java: public class Frequency implements Serializable
impl Frequency {
    // Java: Frequency()
    pub fn Frequency(&self) {
        // Java: freqTable = new TreeMap<Comparable<?>, Long>();
    }

    // Java: Frequency(Comparator<?> comparator)
    pub fn Frequency_2(&self) {
        // Java: freqTable = new TreeMap<Comparable<?>, Long>((Comparator<? super Comparable<?>>) comparator);
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: NumberFormat nf = NumberFormat.getPercentInstance();
        // Java: StringBuilder outBuffer = new StringBuilder();
        // Java: outBuffer.append("Value \t Freq. \t Pct. \t Cum Pct. \n");
        // Java: Iterator<Comparable<?>> iter = freqTable.keySet().iterator();
        // Java: while (iter.hasNext())
        {
            // Java: Comparable<?> value = iter.next();
            // Java: outBuffer.append(value);
            // Java: outBuffer.append('\t');
            // Java: outBuffer.append(getCount(value));
            // Java: outBuffer.append('\t');
            // Java: outBuffer.append(nf.format(getPct(value)));
            // Java: outBuffer.append('\t');
            // Java: outBuffer.append(nf.format(getCumPct(value)));
            // Java: outBuffer.append('\n');
        }
        // Java: return outBuffer.toString();
        todo!()
    }

    // Java: void addValue(Comparable<?> v)
    pub fn addValue(&self) {
        // Java: incrementValue(v, 1);
    }

    // Java: void addValue(int v)
    pub fn addValue_2(&self) {
        // Java: addValue(Long.valueOf(v));
    }

    // Java: void addValue(long v)
    pub fn addValue_3(&self) {
        // Java: addValue(Long.valueOf(v));
    }

    // Java: void addValue(char v)
    pub fn addValue_4(&self) {
        // Java: addValue(Character.valueOf(v));
    }

    // Java: void incrementValue(Comparable<?> v, long increment)
    pub fn incrementValue(&self) {
        // Java: Comparable<?> obj = v;
        // Java: if (v instanceof Integer)
        {
            // Java: obj = Long.valueOf(((Integer) v).longValue());
        }
        // Java: try
        {
            // Java: Long count = freqTable.get(obj);
            // Java: if (count == null)
            {
                // Java: freqTable.put(obj, Long.valueOf(increment));
            }
            // Java: else
            {
                // Java: freqTable.put(obj, Long.valueOf(count.longValue() + increment));
            }
        }
        // Java: catch (ClassCastException ex)
        {
            // Java: //TreeMap will throw ClassCastException if v is not comparable
            // Java: throw new MathIllegalArgumentException(
                  // Java: LocalizedFormats.INSTANCES_NOT_COMPARABLE_TO_EXISTING_VALUES,
                  // Java: v.getClass().getName());
        }
    }

    // Java: void incrementValue(int v, long increment)
    pub fn incrementValue_2(&self) {
        // Java: incrementValue(Long.valueOf(v), increment);
    }

    // Java: void incrementValue(long v, long increment)
    pub fn incrementValue_3(&self) {
        // Java: incrementValue(Long.valueOf(v), increment);
    }

    // Java: void incrementValue(char v, long increment)
    pub fn incrementValue_4(&self) {
        // Java: incrementValue(Character.valueOf(v), increment);
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: freqTable.clear();
    }

    // Java: public Iterator<Comparable<?>> valuesIterator()
    pub fn valuesIterator(&self) -> Box<dyn std::any::Any> {
        // Java: return freqTable.keySet().iterator();
        todo!()
    }

    // Java: long getSumFreq()
    pub fn getSumFreq(&self) -> i64 {
        // Java: long result = 0;
        // Java: Iterator<Long> iterator = freqTable.values().iterator();
        // Java: while (iterator.hasNext())
        {
            // Java: result += iterator.next().longValue();
        }
        // Java: return result;
        todo!()
    }

    // Java: long getCount(Comparable<?> v)
    pub fn getCount(&self) -> i64 {
        // Java: if (v instanceof Integer)
        {
            // Java: return getCount(((Integer) v).longValue());
        }
        // Java: long result = 0;
        // Java: try
        {
            // Java: Long count =  freqTable.get(v);
            // Java: if (count != null)
            {
                // Java: result = count.longValue();
            }
        }
            // Java: // ignore and return 0 -- ClassCastException will be thrown if value is not comparable
        // Java: return result;
        todo!()
    }

    // Java: long getCount(int v)
    pub fn getCount_2(&self) -> i64 {
        // Java: return getCount(Long.valueOf(v));
        todo!()
    }

    // Java: long getCount(long v)
    pub fn getCount_3(&self) -> i64 {
        // Java: return getCount(Long.valueOf(v));
        todo!()
    }

    // Java: long getCount(char v)
    pub fn getCount_4(&self) -> i64 {
        // Java: return getCount(Character.valueOf(v));
        todo!()
    }

    // Java: int getUniqueCount()
    pub fn getUniqueCount(&self) -> i32 {
        // Java: return freqTable.keySet().size();
        todo!()
    }

    // Java: double getPct(Comparable<?> v)
    pub fn getPct(&self) -> f64 {
        // Java: final long sumFreq = getSumFreq();
        // Java: if (sumFreq == 0)
        {
            // Java: return Double.NaN;
        }
        // Java: return (double) getCount(v) / (double) sumFreq;
        todo!()
    }

    // Java: double getPct(int v)
    pub fn getPct_2(&self) -> f64 {
        // Java: return getPct(Long.valueOf(v));
        todo!()
    }

    // Java: double getPct(long v)
    pub fn getPct_3(&self) -> f64 {
        // Java: return getPct(Long.valueOf(v));
        todo!()
    }

    // Java: double getPct(char v)
    pub fn getPct_4(&self) -> f64 {
        // Java: return getPct(Character.valueOf(v));
        todo!()
    }

    // Java: long getCumFreq(Comparable<?> v)
    pub fn getCumFreq(&self) -> i64 {
        // Java: if (getSumFreq() == 0)
        {
            // Java: return 0;
        }
        // Java: if (v instanceof Integer)
        {
            // Java: return getCumFreq(((Integer) v).longValue());
        }
        // Java: Comparator<Comparable<?>> c = (Comparator<Comparable<?>>) freqTable.comparator();
        // Java: if (c == null)
        {
            // Java: c = new NaturalComparator();
        }
        // Java: long result = 0;
        // Java: try
        {
            // Java: Long value = freqTable.get(v);
            // Java: if (value != null)
            {
                // Java: result = value.longValue();
            }
        }
        // Java: catch (ClassCastException ex)
        {
            // Java: return result;   // v is not comparable
        }
        // Java: if (c.compare(v, freqTable.firstKey()) < 0)
        {
            // Java: return 0;  // v is comparable, but less than first value
        }
        // Java: if (c.compare(v, freqTable.lastKey()) >= 0)
        {
            // Java: return getSumFreq();    // v is comparable, but greater than the last value
        }
        // Java: Iterator<Comparable<?>> values = valuesIterator();
        // Java: while (values.hasNext())
        {
            // Java: Comparable<?> nextValue = values.next();
            // Java: if (c.compare(v, nextValue) > 0)
            {
                // Java: result += getCount(nextValue);
            }
            // Java: else
            {
                // Java: return result;
            }
        }
        // Java: return result;
        todo!()
    }

    // Java: long getCumFreq(int v)
    pub fn getCumFreq_2(&self) -> i64 {
        // Java: return getCumFreq(Long.valueOf(v));
        todo!()
    }

    // Java: long getCumFreq(long v)
    pub fn getCumFreq_3(&self) -> i64 {
        // Java: return getCumFreq(Long.valueOf(v));
        todo!()
    }

    // Java: long getCumFreq(char v)
    pub fn getCumFreq_4(&self) -> i64 {
        // Java: return getCumFreq(Character.valueOf(v));
        todo!()
    }

    // Java: double getCumPct(Comparable<?> v)
    pub fn getCumPct(&self) -> f64 {
        // Java: final long sumFreq = getSumFreq();
        // Java: if (sumFreq == 0)
        {
            // Java: return Double.NaN;
        }
        // Java: return (double) getCumFreq(v) / (double) sumFreq;
        todo!()
    }

    // Java: double getCumPct(int v)
    pub fn getCumPct_2(&self) -> f64 {
        // Java: return getCumPct(Long.valueOf(v));
        todo!()
    }

    // Java: double getCumPct(long v)
    pub fn getCumPct_3(&self) -> f64 {
        // Java: return getCumPct(Long.valueOf(v));
        todo!()
    }

    // Java: double getCumPct(char v)
    pub fn getCumPct_4(&self) -> f64 {
        // Java: return getCumPct(Character.valueOf(v));
        todo!()
    }

    // Java: public List<Comparable<?>> getMode()
    pub fn getMode(&self) -> Box<dyn std::any::Any> {
        // Java: long mostPopular = 0; // frequencies are always positive
        // Java: // Get the max count first, so we avoid having to recreate the List each time
        // Java: for(Long l : freqTable.values())
        {
            // Java: long frequency = l.longValue();
            // Java: if (frequency > mostPopular)
            {
                // Java: mostPopular = frequency;
            }
        }
        // Java: List<Comparable<?>> modeList = new ArrayList<Comparable<?>>();
        // Java: for (Entry<Comparable<?>, Long> ent : freqTable.entrySet())
        {
            // Java: long frequency = ent.getValue().longValue();
            // Java: if (frequency == mostPopular)
            {
               // Java: modeList.add(ent.getKey());
            }
        }
        // Java: return modeList;
        todo!()
    }

    // Java: void merge(final Frequency other)
    pub fn merge(&self) {
        // Java: MathUtils.checkNotNull(other, LocalizedFormats.NULL_NOT_ALLOWED);
        // Java: final Iterator<Map.Entry<Comparable<?>, Long>> iter = other.entrySetIterator();
        // Java: while (iter.hasNext())
        {
            // Java: final Map.Entry<Comparable<?>, Long> entry = iter.next();
            // Java: incrementValue(entry.getKey(), entry.getValue().longValue());
        }
    }

    // Java: void merge(final Collection<Frequency> others)
    pub fn merge_2(&self) {
        // Java: MathUtils.checkNotNull(others, LocalizedFormats.NULL_NOT_ALLOWED);
        // Java: for (final Frequency freq : others)
        {
            // Java: merge(freq);
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: final int prime = 31;
        // Java: int result = 1;
        // Java: result = prime * result +
                 // Java: ((freqTable == null) ? 0 : freqTable.hashCode());
        // Java: return result;
        todo!()
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self) -> bool {
        // Java: if (this == obj)
        {
            // Java: return true;
        }
        // Java: if (!(obj instanceof Frequency))
        {
            // Java: return false;
        }
        // Java: Frequency other = (Frequency) obj;
        // Java: if (freqTable == null)
        {
            // Java: if (other.freqTable != null)
            {
                // Java: return false;
            }
        }
        // Java: else if (!freqTable.equals(other.freqTable))
        {
            // Java: return false;
        }
        // Java: return true;
        todo!()
    }

}

// Java: public class NaturalComparator
impl NaturalComparator {
    // Java: int compare(Comparable<T> o1, Comparable<T> o2)
    pub fn compare(&self) -> i32 {
            // Java: return o1.compareTo((T) o2);
        todo!()
    }

}

