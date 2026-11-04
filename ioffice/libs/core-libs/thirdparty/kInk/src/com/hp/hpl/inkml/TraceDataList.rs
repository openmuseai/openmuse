use super::Channel::ChannelType;

// Java: public class TraceDataList extends LinkedList implements Cloneable
pub struct TraceDataList {
    // Java field: Channel.ChannelType mXYChannelType
    pub(crate) mXYChannelType: ChannelType,
    // Java field: int mXIndex
    pub(crate) mXIndex: i32,
    // Java field: int mYIndex
    pub(crate) mYIndex: i32,
    // Java field: int mFIndex
    pub(crate) mFIndex: i32,
    // Java field: float mMinX
    mMinX: f32,
    // Java field: float mMaxX
    mMaxX: f32,
    // Java field: float mMinY
    mMinY: f32,
    // Java field: float mMaxY
    mMaxY: f32,
    // Java field: float mMinF
    mMinF: f32,
    // Java field: float mMaxF
    mMaxF: f32,
    // Java field: boolean mIsBoundValid
    mIsBoundValid: bool,
    data: Vec<Vec<f32>>,
}

// Java: public class TraceDataList extends LinkedList implements Cloneable
impl TraceDataList {
    // Java: TraceDataList(Channel.ChannelType xyChannelType, int xIndex, int yIndex, int fIndex)
    pub fn TraceDataList_full(xyChannelType: ChannelType, xIndex: i32, yIndex: i32, fIndex: i32) -> Self {
        // Java: super();
        // Java: this.mXYChannelType = xyChannelType;
        // Java: this.mXIndex = xIndex;
        // Java: this.mYIndex = yIndex;
        // Java: this.mFIndex = fIndex;
        Self {
            mXYChannelType: xyChannelType,
            mXIndex: xIndex,
            mYIndex: yIndex,
            mFIndex: fIndex,
            mMinX: 0.0, mMaxX: 0.0,
            mMinY: 0.0, mMaxY: 0.0,
            mMinF: 0.0, mMaxF: 0.0,
            mIsBoundValid: false,
            data: Vec::new(),
        }
    }

    // Java: TraceDataList()
    pub fn TraceDataList() -> Self {
        // Java: this(Channel.ChannelType.INTEGER, 0, 1, -1);
        Self::TraceDataList_full(ChannelType::INTEGER, 0, 1, -1)
    }

    // Java: boolean add(Object[] fragment)
    pub fn add(&mut self, fragment: Vec<f32>) -> bool {
        // Java: if (0 == fragment.length)
        if 0 == fragment.len() {
            // Java: return false;
            return false;
        // Java: else
        } else {
            // Java: this.applyBounds((Float)fragment[this.mXIndex], (Float)fragment[this.mYIndex]);
            self.applyBounds(fragment[self.mXIndex as usize], fragment[self.mYIndex as usize]);
            // Java: if (this.mFIndex != -1)
            if self.mFIndex != -1 {
                // Java: float f = (Float)fragment[this.mFIndex];
                let f = fragment[self.mFIndex as usize];
                // Java: if (f < this.mMinF)
                if f < self.mMinF {
                    // Java: this.mMinF = f;
                    self.mMinF = f;
                // Java: else if (f > this.mMaxF)
                } else if f > self.mMaxF {
                    // Java: this.mMaxF = f;
                    self.mMaxF = f;
                }
            }
            // Java: return super.add(fragment);
            self.data.push(fragment);
            true
        }
    }

    // Java: Channel.ChannelType getXYChannleType()
    pub fn getXYChannleType(&self) -> ChannelType {
        // Java: return this.mXYChannelType;
        self.mXYChannelType
    }

    // Java: void setXYChannelType(Channel.ChannelType type)
    pub fn setXYChannelType(&mut self, channel_type: ChannelType) {
        // Java: this.mXYChannelType = type;
        self.mXYChannelType = channel_type;
    }

    // Java: RectF getRect()
    pub fn getRect(&self) -> (f32, f32, f32, f32) {
        // Java: return new RectF(this.mMinX, this.mMinY, this.mMaxX, this.mMaxY);
        (self.mMinX, self.mMinY, self.mMaxX, self.mMaxY)
    }

    // Java: private void applyBounds(float x, float y)
    fn applyBounds(&mut self, x: f32, y: f32) {
        // Java: if (this.mIsBoundValid)
        if self.mIsBoundValid {
            // Java: if (x < this.mMinX)
            if x < self.mMinX {
                // Java: this.mMinX = x;
                self.mMinX = x;
            // Java: else if (x > this.mMaxX)
            } else if x > self.mMaxX {
                // Java: this.mMaxX = x;
                self.mMaxX = x;
            }
            // Java: if (y < this.mMinY)
            if y < self.mMinY {
                // Java: this.mMinY = y;
                self.mMinY = y;
            // Java: else if (y > this.mMaxY)
            } else if y > self.mMaxY {
                // Java: this.mMaxY = y;
                self.mMaxY = y;
            }
        // Java: else
        } else {
            // Java: this.mMinX = x;
            self.mMinX = x;
            // Java: this.mMaxX = x;
            self.mMaxX = x;
            // Java: this.mMinY = y;
            self.mMinY = y;
            // Java: this.mMaxY = y;
            self.mMaxY = y;
            // Java: this.mIsBoundValid = true;
            self.mIsBoundValid = true;
        }
    }

    // Java: void expandBounds(com.hp.hpl.inkml.impl.Brush brush)
    pub fn expandBounds(&mut self) {
    }

    // Java: int getXIndex()
    pub fn getXIndex(&self) -> i32 {
        // Java: return this.mXIndex;
        self.mXIndex
    }

    // Java: int getYIndex()
    pub fn getYIndex(&self) -> i32 {
        // Java: return this.mYIndex;
        self.mYIndex
    }

    // Java: int getFIndex()
    pub fn getFIndex(&self) -> i32 {
        // Java: return this.mFIndex;
        self.mFIndex
    }

    // Java: void setFIndex(int fIndex)
    pub fn setFIndex(&mut self, fIndex: i32) {
        // Java: this.mFIndex = fIndex;
        self.mFIndex = fIndex;
    }

    // Java: TraceDataList clone()
    pub fn clone_tdl(&self) -> TraceDataList {
        // Java: TraceDataList ret = new TraceDataList();
        let mut ret = TraceDataList::TraceDataList();
        // Java: ret.mIsBoundValid = this.mIsBoundValid;
        ret.mIsBoundValid = self.mIsBoundValid;
        // Java: ret.mMaxX = this.mMaxX;
        ret.mMaxX = self.mMaxX;
        // Java: ret.mMaxY = this.mMaxY;
        ret.mMaxY = self.mMaxY;
        // Java: ret.mMinX = this.mMinX;
        ret.mMinX = self.mMinX;
        // Java: ret.mMinY = this.mMinY;
        ret.mMinY = self.mMinY;
        // Java: ret.modCount = this.modCount;
        // Java: ret.mXIndex = this.mXIndex;
        ret.mXIndex = self.mXIndex;
        // Java: ret.mXYChannelType = this.mXYChannelType;
        ret.mXYChannelType = self.mXYChannelType;
        // Java: ret.mYIndex = this.mYIndex;
        ret.mYIndex = self.mYIndex;
        // Java: ret.mFIndex = this.mFIndex;
        ret.mFIndex = self.mFIndex;
        // Java: int size = this.size();
        let size = self.data.len();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: Object[] dst = null;
            // Java: switch (this.mXYChannelType)
            let dst = match self.mXYChannelType {
                // Java: case INTEGER:
                // Java: case DECIMAL:
                ChannelType::INTEGER | ChannelType::DECIMAL => {
                    // Java: dst = this.cloneDecimalList(this.get(i));
                    // Java: break;
                    self.cloneDecimalList(&self.data[i])
                }
                // Java: case BOOLEAN:
                ChannelType::BOOLEAN => {
                    // Java: dst = this.cloneBooleanList(this.get(i));
                    self.cloneBooleanList(&self.data[i])
                }
            };
            // Java: if (dst != null)
            {
                // Java: ret.add(dst);
                ret.data.push(dst);
            }
        }
        // Java: return ret;
        ret
    }

    // Java: private Object[] cloneIntegerList(Object[] objects)
    fn cloneIntegerList(&self, objects: &[f32]) -> Vec<f32> {
        // Java: int count = objects.length;
        // Java: Integer[] ret = new Integer[count];
        // Java: for(int i = 0; i < count; ++i)
        // Java: Integer e = (Integer)objects[i];
        // Java: ret[i] = new Integer(e);
        // Java: return ret;
        objects.to_vec()
    }

    // Java: private Object[] cloneDecimalList(Object[] objects)
    fn cloneDecimalList(&self, objects: &[f32]) -> Vec<f32> {
        // Java: int count = objects.length;
        // Java: Float[] ret = new Float[count];
        // Java: for(int i = 0; i < count; ++i)
        // Java: Float e = (Float)objects[i];
        // Java: ret[i] = new Float(e);
        // Java: return ret;
        objects.to_vec()
    }

    // Java: private Object[] cloneBooleanList(Object[] objects)
    fn cloneBooleanList(&self, objects: &[f32]) -> Vec<f32> {
        // Java: int count = objects.length;
        // Java: Boolean[] ret = new Boolean[count];
        // Java: for(int i = 0; i < count; ++i)
        // Java: Boolean e = (Boolean)objects[i];
        // Java: ret[i] = new Boolean(e);
        // Java: return ret;
        objects.to_vec()
    }

    // Java: void scale(float scaleX, float scaleY)
    pub fn scale(&mut self, scaleX: f32, scaleY: f32) {
        // Java: this.mMinX *= scaleX;
        self.mMinX *= scaleX;
        // Java: this.mMaxX *= scaleX;
        self.mMaxX *= scaleX;
        // Java: this.mMinY *= scaleY;
        self.mMinY *= scaleY;
        // Java: this.mMaxY *= scaleY;
        self.mMaxY *= scaleY;
    }

    // Java: void offset(float offsetX, float offsetY)
    pub fn offset(&mut self, offsetX: f32, offsetY: f32) {
        // Java: this.mMinX += offsetX;
        self.mMinX += offsetX;
        // Java: this.mMaxX += offsetX;
        self.mMaxX += offsetX;
        // Java: this.mMinY += offsetY;
        self.mMinY += offsetY;
        // Java: this.mMaxY += offsetY;
        self.mMaxY += offsetY;
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> &Vec<f32> {
        &self.data[index]
    }
}
