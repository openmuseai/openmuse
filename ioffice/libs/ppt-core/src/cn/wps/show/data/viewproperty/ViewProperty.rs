// Rust stub for ViewProperty (auto-generated)
// Original: cn/wps/show/data/viewproperty/ViewProperty.java
use crate::*;

pub struct ViewProperty {
    // TODO: fields
}

impl ViewProperty {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> ViewProperty { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> ViewProperty { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: ViewProperty) { todo!() }
    pub fn saveBin(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: ViewProperty) -> bool { todo!() }
    pub fn equals(&self, o: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn equals_1(&self, o: Box<dyn std::any::Any + Send + Sync>, isStrict: bool) -> bool { todo!() }
    pub fn hashCode(&self) -> i32 { todo!() }
    pub fn dirty(&self) -> bool { todo!() }
    pub fn save(&self) -> Box<dyn KoBin> { todo!() }
    pub fn lastBin(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn discard(&self, zone: KoZone) { todo!() }
    pub fn close(&self) { todo!() }
    pub fn cleanFollow(&self) { todo!() }
    pub fn cleanCache(&self) { todo!() }
    pub fn follow(&self, parent: ViewProperty) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> ViewProperty { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn normalViewProperty(&self) -> NormalViewProperty { todo!() }
    pub fn hasNormalViewProperty(&self) -> bool { todo!() }
    pub fn deleteNormalViewProperty(&self) { todo!() }
    pub fn writeNormalViewProperty(&self, value: NormalViewProperty) { todo!() }
    pub fn forceWriteNormalViewProperty(&self, value: NormalViewProperty) { todo!() }
    pub fn slideViewProperty(&self) -> CommonSlideViewProperty { todo!() }
    pub fn hasSlideViewProperty(&self) -> bool { todo!() }
    pub fn deleteSlideViewProperty(&self) { todo!() }
    pub fn writeSlideViewProperty(&self, value: CommonSlideViewProperty) { todo!() }
    pub fn forceWriteSlideViewProperty(&self, value: CommonSlideViewProperty) { todo!() }
    pub fn outlineViewProperty(&self) -> OutlineViewProperty { todo!() }
    pub fn hasOutlineViewProperty(&self) -> bool { todo!() }
    pub fn deleteOutlineViewProperty(&self) { todo!() }
    pub fn writeOutlineViewProperty(&self, value: OutlineViewProperty) { todo!() }
    pub fn forceWriteOutlineViewProperty(&self, value: OutlineViewProperty) { todo!() }
    pub fn notesTextViewProperty(&self) -> CommonViewProperty { todo!() }
    pub fn hasNotesTextViewProperty(&self) -> bool { todo!() }
    pub fn deleteNotesTextViewProperty(&self) { todo!() }
    pub fn writeNotesTextViewProperty(&self, value: CommonViewProperty) { todo!() }
    pub fn forceWriteNotesTextViewProperty(&self, value: CommonViewProperty) { todo!() }
    pub fn sorterViewProperty(&self) -> SorterViewProperty { todo!() }
    pub fn hasSorterViewProperty(&self) -> bool { todo!() }
    pub fn deleteSorterViewProperty(&self) { todo!() }
    pub fn writeSorterViewProperty(&self, value: SorterViewProperty) { todo!() }
    pub fn forceWriteSorterViewProperty(&self, value: SorterViewProperty) { todo!() }
    pub fn notesViewProperty(&self) -> CommonSlideViewProperty { todo!() }
    pub fn hasNotesViewProperty(&self) -> bool { todo!() }
    pub fn deleteNotesViewProperty(&self) { todo!() }
    pub fn writeNotesViewProperty(&self, value: CommonSlideViewProperty) { todo!() }
    pub fn forceWriteNotesViewProperty(&self, value: CommonSlideViewProperty) { todo!() }
    pub fn gridSpacing(&self) -> PositiveSize2D { todo!() }
    pub fn hasGridSpacing(&self) -> bool { todo!() }
    pub fn deleteGridSpacing(&self) { todo!() }
    pub fn writeGridSpacing(&self, value: PositiveSize2D) { todo!() }
    pub fn forceWriteGridSpacing(&self, value: PositiveSize2D) { todo!() }
    pub fn lastView(&self) -> i32 { todo!() }
    pub fn hasLastView(&self) -> bool { todo!() }
    pub fn deleteLastView(&self) { todo!() }
    pub fn putLastView(&self, value: i32) { todo!() }
    pub fn isShowComments(&self) -> bool { todo!() }
    pub fn hasShowComments(&self) -> bool { todo!() }
    pub fn deleteShowComments(&self) { todo!() }
    pub fn putShowComments(&self, value: bool) { todo!() }
}
