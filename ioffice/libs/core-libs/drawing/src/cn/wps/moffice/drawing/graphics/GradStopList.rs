#![allow(non_snake_case)]

// Java: import java.io.Externalizable
// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput
// Java: import java.util.Collections
// Java: import java.util.Comparator
// Java: import java.util.Vector
use std::io::{Read, Write, Result};
use super::GradStop::GradStop;

pub trait Cloneable {}
pub trait Externalizable {}

struct Collections;
impl Collections {
    fn sort<T>(list: &mut Vec<T>, comparator: impl Fn(&T, &T) -> std::cmp::Ordering) {
        list.sort_by(comparator);
    }
}

// Java: public class GradStopList implements Cloneable, Externalizable
#[derive(Clone)]
pub struct GradStopList {
    // Java field: Vector<GradStop> gradStops
    pub gradStops: Vec<GradStop>,
}

impl Cloneable for GradStopList {}
impl Externalizable for GradStopList {}

// Java: public class MyCompare
pub struct MyCompare;

// Java: public class GradStopList implements Cloneable, Externalizable
impl GradStopList {
    // Java: GradStopList()
    pub fn new() -> Self {
        // Java: super();
        GradStopList {
            gradStops: Vec::new(),
        }
    }

    // Java: GradStopList clone()
    pub fn clone_list(&self) -> Option<GradStopList> {
        // Java: try
        let result: std::result::Result<GradStopList, String> = (|| -> std::result::Result<GradStopList, String> {
            // Java: return (GradStopList)super.clone();
            Ok(self.clone())
        })();
        match result {
            Ok(cloned) => Some(cloned),
            // Java: catch (CloneNotSupportedException e)
            Err(e) => {
                // Java: e.printStackTrace();
                eprintln!("{}", e);
                // Java: return null;
                None
            }
        }
    }

    // Java: boolean equals(Object o)
    pub fn equals_obj(&self, o: Option<&GradStopList>) -> bool {
        // Java: if (o != null && o instanceof GradStopList)
        if o.is_some() && o.unwrap().gradStops.len() >= 0 {
            let o = o.unwrap();
            // Java: if (o == this)
            if std::ptr::eq(o, self) {
                // Java: return true;
                return true;
            // Java: else
            } else {
                // Java: int size = this.size();
                let size = self.size();
                // Java: GradStopList obj = (GradStopList)o;
                let obj = o;
                // Java: if (obj.size() != size)
                if obj.size() != size {
                    // Java: return false;
                    return false;
                // Java: else
                } else {
                    // Java: for(int i = 0; i < size; ++i)
                    for i in 0..size as usize {
                        // Java: if (!((GradStop)this.gradStops.elementAt(i)).equals(obj.gradStops.elementAt(i)))
                        if self.gradStops[i].getPostion() != obj.gradStops[i].getPostion()
                            || self.gradStops[i].getColor() != obj.gradStops[i].getColor() {
                            // Java: return false;
                            return false;
                        }
                    }
                    // Java: return true;
                    return true;
                }
            }
        // Java: else
        } else {
            // Java: return false;
            return false;
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.gradStops.size();
        self.gradStops.len() as i32
    }

    // Java: void remove(int n)
    pub fn remove(&mut self, n: i32) {
        // Java: if (n >= 0 && n < this.size())
        if n >= 0 && n < self.size() {
            // Java: this.gradStops.remove(n);
            self.gradStops.remove(n as usize);
        }
    }

    // Java: GradStop at(int n)
    pub fn at(&self, n: i32) -> Option<&GradStop> {
        // Java: return n >= 0 && n < this.size() ? (GradStop)this.gradStops.elementAt(n) : null;
        if n >= 0 && n < self.size() {
            Some(&self.gradStops[n as usize])
        } else {
            None
        }
    }

    // Java: void add(GradStop gradStop)
    pub fn add(&mut self, gradStop: GradStop) {
        // Java: this.gradStops.add(gradStop);
        self.gradStops.push(gradStop);
    }

    // Java: void sort()
    pub fn sort(&mut self) {
        // Java: Collections.sort(this.gradStops, new MyCompare());
        Collections::sort(&mut self.gradStops, |a, b| MyCompare::compare_stops(a, b));
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &mut dyn Read) -> Result<()> {
        let mut buf = [0u8; 4];
        // Java: int size = input.readInt();
        input.read_exact(&mut buf)?;
        let size = i32::from_be_bytes(buf);
        // Java: for(int i = 0; i < size; ++i)
        for _i in 0..size {
            // Java: GradStop gradStop = (GradStop)input.readObject();
            let mut pos_buf = [0u8; 4];
            input.read_exact(&mut pos_buf)?;
            let mut gradStop = GradStop::new();
            gradStop.setPostion(f32::from_be_bytes(pos_buf));
            // Java: this.add(gradStop);
            self.add(gradStop);
        }
        Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut dyn Write) -> Result<()> {
        // Java: int size = this.size();
        let size = self.size();
        // Java: output.writeInt(size);
        output.write_all(&size.to_be_bytes())?;
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: output.writeObject(this.at(i));
            if let Some(stop) = self.at(i) {
                output.write_all(&stop.getPostion().to_be_bytes())?;
            }
        }
        Ok(())
    }
}

// Java: public class MyCompare
impl MyCompare {
    // Java: MyCompare()
    fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: int compare(GradStop o1, GradStop o2)
    pub fn compare_stops(o1: &GradStop, o2: &GradStop) -> std::cmp::Ordering {
        // Java: if (o1.getPostion() > o2.getPostion())
        if o1.getPostion() > o2.getPostion() {
            // Java: return 1;
            std::cmp::Ordering::Greater
        // Java: else
        } else {
            // Java: return o1.getPostion() < o2.getPostion() ? -1 : 0;
            if o1.getPostion() < o2.getPostion() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
