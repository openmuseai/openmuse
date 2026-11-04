// Rust stub for Model3D (auto-generated)
// Original: cn/wps/drawing/data/model3d/Model3D.java
use crate::*;

pub struct Model3D {
    // TODO: fields
}

impl Model3D {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> Model3D { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> Model3D { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone::defaultZone()) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: Model3D) { todo!() }
    pub fn saveBin(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: Model3D) -> bool { todo!() }
    pub fn equals(&self, o: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn equals_1(&self, o: Box<dyn std::any::Any + Send + Sync>, isStrict: bool) -> bool { todo!() }
    pub fn hashCode(&self) -> i32 { todo!() }
    pub fn dirty(&self) -> bool { todo!() }
    pub fn save(&self) -> Box<dyn KoBin> { todo!() }
    pub fn lastBin(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn discard(&self, zone: KoZone::defaultZone()) { todo!() }
    pub fn close(&self) { todo!() }
    pub fn cleanFollow(&self) { todo!() }
    pub fn cleanCache(&self) { todo!() }
    pub fn follow(&self, parent: Model3D) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> Model3D { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn embedMediaID(&self) -> i32 { todo!() }
    pub fn hasEmbedMediaID(&self) -> bool { todo!() }
    pub fn deleteEmbedMediaID(&self) { todo!() }
    pub fn putEmbedMediaID(&self, value: i32) { todo!() }
    pub fn spPr(&self) -> ShapePr { todo!() }
    pub fn hasSpPr(&self) -> bool { todo!() }
    pub fn deleteSpPr(&self) { todo!() }
    pub fn writeSpPr(&self, value: ShapePr) { todo!() }
    pub fn forceWriteSpPr(&self, value: ShapePr) { todo!() }
    pub fn camera(&self) -> Model3DCamera { todo!() }
    pub fn hasCamera(&self) -> bool { todo!() }
    pub fn deleteCamera(&self) { todo!() }
    pub fn writeCamera(&self, value: Model3DCamera) { todo!() }
    pub fn forceWriteCamera(&self, value: Model3DCamera) { todo!() }
    pub fn transform(&self) -> Model3DTransform { todo!() }
    pub fn hasTransform(&self) -> bool { todo!() }
    pub fn deleteTransform(&self) { todo!() }
    pub fn writeTransform(&self, value: Model3DTransform) { todo!() }
    pub fn forceWriteTransform(&self, value: Model3DTransform) { todo!() }
    pub fn raster(&self) -> Model3DRaster { todo!() }
    pub fn hasRaster(&self) -> bool { todo!() }
    pub fn deleteRaster(&self) { todo!() }
    pub fn writeRaster(&self, value: Model3DRaster) { todo!() }
    pub fn forceWriteRaster(&self, value: Model3DRaster) { todo!() }
    pub fn objViewport(&self) -> ObjViewport { todo!() }
    pub fn hasObjViewport(&self) -> bool { todo!() }
    pub fn deleteObjViewport(&self) { todo!() }
    pub fn writeObjViewport(&self, value: ObjViewport) { todo!() }
    pub fn forceWriteObjViewport(&self, value: ObjViewport) { todo!() }
    pub fn winViewport(&self) -> WinViewport { todo!() }
    pub fn hasWinViewport(&self) -> bool { todo!() }
    pub fn deleteWinViewport(&self) { todo!() }
    pub fn writeWinViewport(&self, value: WinViewport) { todo!() }
    pub fn forceWriteWinViewport(&self, value: WinViewport) { todo!() }
    pub fn ambientLight(&self) -> AmbientLight { todo!() }
    pub fn hasAmbientLight(&self) -> bool { todo!() }
    pub fn deleteAmbientLight(&self) { todo!() }
    pub fn writeAmbientLight(&self, value: AmbientLight) { todo!() }
    pub fn forceWriteAmbientLight(&self, value: AmbientLight) { todo!() }
    pub fn spotLight(&self) -> SpotLight { todo!() }
    pub fn hasSpotLight(&self) -> bool { todo!() }
    pub fn deleteSpotLight(&self) { todo!() }
    pub fn writeSpotLight(&self, value: SpotLight) { todo!() }
    pub fn forceWriteSpotLight(&self, value: SpotLight) { todo!() }
    pub fn dirLight(&self) -> DirectionalLight { todo!() }
    pub fn hasDirLight(&self) -> bool { todo!() }
    pub fn deleteDirLight(&self) { todo!() }
    pub fn writeDirLight(&self, value: DirectionalLight) { todo!() }
    pub fn forceWriteDirLight(&self, value: DirectionalLight) { todo!() }
    pub fn ptLights(&self) -> PointLightList { todo!() }
    pub fn hasPtLights(&self) -> bool { todo!() }
    pub fn deletePtLights(&self) { todo!() }
    pub fn writePtLights(&self, value: PointLightList) { todo!() }
    pub fn forceWritePtLights(&self, value: PointLightList) { todo!() }
    pub fn extLst(&self) -> A3DAnimList { todo!() }
    pub fn hasExtLst(&self) -> bool { todo!() }
    pub fn deleteExtLst(&self) { todo!() }
    pub fn writeExtLst(&self, value: A3DAnimList) { todo!() }
    pub fn forceWriteExtLst(&self, value: A3DAnimList) { todo!() }
    pub fn loadBin_1(&self, bin: Box<dyn KoBin>, zone: KoZone::defaultZone()) { todo!() }
    pub fn load_2(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_3(&self, value: PointLightList) { todo!() }
    pub fn saveBin_1(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax_1(&self, o: PointLightList) -> bool { todo!() }
    pub fn equals_2(&self, o: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn equals_3(&self, o: Box<dyn std::any::Any + Send + Sync>, isStrict: bool) -> bool { todo!() }
    pub fn hashCode_1(&self) -> i32 { todo!() }
    pub fn dirty_1(&self) -> bool { todo!() }
    pub fn save_1(&self) -> Box<dyn KoBin> { todo!() }
    pub fn discard_2(&self, zone: KoZone::defaultZone()) { todo!() }
    pub fn lastBin_1(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn close_1(&self) { todo!() }
    pub fn initialize_1(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create_1() -> PointLightList { todo!() }
    pub fn open_1(bin: Box<dyn KoBin>) -> PointLightList { todo!() }
    pub fn read(&self, valueLst: Collection) { todo!() }
    pub fn write(&self, valueLst: Iterable) { todo!() }
    pub fn forceWrite(&self, valueLst: Iterable) { todo!() }
    pub fn loadBin_2(&self, bin: Box<dyn KoBin>, zone: KoZone::defaultZone()) { todo!() }
    pub fn load_4(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_5(&self, value: A3DAnimList) { todo!() }
    pub fn saveBin_2(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax_2(&self, o: A3DAnimList) -> bool { todo!() }
    pub fn equals_4(&self, o: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn equals_5(&self, o: Box<dyn std::any::Any + Send + Sync>, isStrict: bool) -> bool { todo!() }
    pub fn hashCode_2(&self) -> i32 { todo!() }
    pub fn dirty_2(&self) -> bool { todo!() }
    pub fn save_2(&self) -> Box<dyn KoBin> { todo!() }
    pub fn discard_3(&self, zone: KoZone::defaultZone()) { todo!() }
    pub fn lastBin_2(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn close_2(&self) { todo!() }
    pub fn initialize_2(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create_2() -> A3DAnimList { todo!() }
    pub fn open_2(bin: Box<dyn KoBin>) -> A3DAnimList { todo!() }
    pub fn read_1(&self, valueLst: Collection) { todo!() }
    pub fn write_1(&self, valueLst: Iterable) { todo!() }
    pub fn forceWrite_1(&self, valueLst: Iterable) { todo!() }
    pub fn initialize_3(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create_3() -> Model3DRaster { todo!() }
    pub fn open_3(bin: Box<dyn KoBin>) -> Model3DRaster { todo!() }
    pub fn loadBin_3(&self, bin: Box<dyn KoBin>, zone: KoZone::defaultZone()) { todo!() }
    pub fn load_6(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_7(&self, value: Model3DRaster) { todo!() }
    pub fn saveBin_3(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax_3(&self, o: Model3DRaster) -> bool { todo!() }
    pub fn equals_6(&self, o: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn equals_7(&self, o: Box<dyn std::any::Any + Send + Sync>, isStrict: bool) -> bool { todo!() }
    pub fn hashCode_3(&self) -> i32 { todo!() }
    pub fn dirty_3(&self) -> bool { todo!() }
    pub fn save_3(&self) -> Box<dyn KoBin> { todo!() }
    pub fn lastBin_3(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn discard_4(&self, zone: KoZone::defaultZone()) { todo!() }
    pub fn close_3(&self) { todo!() }
    pub fn cleanFollow_1(&self) { todo!() }
    pub fn cleanCache_1(&self) { todo!() }
    pub fn follow_1(&self, parent: Model3DRaster) { todo!() }
    pub fn unFollow_1(&self) { todo!() }
    pub fn hasParent_1(&self) -> bool { todo!() }
    pub fn parent_1(&self) -> Model3DRaster { todo!() }
    pub fn discard_5(&self) { todo!() }
    pub fn cleanParent_1(&self) { todo!() }
    pub fn rName(&self) -> String { todo!() }
    pub fn hasRName(&self) -> bool { todo!() }
    pub fn deleteRName(&self) { todo!() }
    pub fn putRName(&self, value: String) { todo!() }
    pub fn rVer(&self) -> String { todo!() }
    pub fn hasRVer(&self) -> bool { todo!() }
    pub fn deleteRVer(&self) { todo!() }
    pub fn putRVer(&self, value: String) { todo!() }
    pub fn blip(&self) -> Blip { todo!() }
    pub fn hasBlip(&self) -> bool { todo!() }
    pub fn deleteBlip(&self) { todo!() }
    pub fn writeBlip(&self, value: Blip) { todo!() }
    pub fn forceWriteBlip(&self, value: Blip) { todo!() }
    pub fn initialize_4(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create_4() -> ObjViewport { todo!() }
    pub fn open_4(bin: Box<dyn KoBin>) -> ObjViewport { todo!() }
    pub fn loadBin_4(&self, bin: Box<dyn KoBin>, zone: KoZone::defaultZone()) { todo!() }
    pub fn load_8(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_9(&self, value: ObjViewport) { todo!() }
    pub fn saveBin_4(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax_4(&self, o: ObjViewport) -> bool { todo!() }
    pub fn equals_8(&self, o: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn equals_9(&self, o: Box<dyn std::any::Any + Send + Sync>, isStrict: bool) -> bool { todo!() }
    pub fn hashCode_4(&self) -> i32 { todo!() }
    pub fn dirty_4(&self) -> bool { todo!() }
    pub fn save_4(&self) -> Box<dyn KoBin> { todo!() }
    pub fn lastBin_4(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn discard_6(&self, zone: KoZone::defaultZone()) { todo!() }
    pub fn close_4(&self) { todo!() }
    pub fn cleanFollow_2(&self) { todo!() }
    pub fn cleanCache_2(&self) { todo!() }
    pub fn follow_2(&self, parent: ObjViewport) { todo!() }
    pub fn unFollow_2(&self) { todo!() }
    pub fn hasParent_2(&self) -> bool { todo!() }
    pub fn parent_2(&self) -> ObjViewport { todo!() }
    pub fn discard_7(&self) { todo!() }
    pub fn cleanParent_2(&self) { todo!() }
    pub fn viewportSz(&self) -> i32 { todo!() }
    pub fn hasViewportSz(&self) -> bool { todo!() }
    pub fn deleteViewportSz(&self) { todo!() }
    pub fn putViewportSz(&self, value: i32) { todo!() }
    pub fn initialize_5(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create_5() -> WinViewport { todo!() }
    pub fn open_5(bin: Box<dyn KoBin>) -> WinViewport { todo!() }
    pub fn loadBin_5(&self, bin: Box<dyn KoBin>, zone: KoZone::defaultZone()) { todo!() }
    pub fn load_10(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_11(&self, value: WinViewport) { todo!() }
    pub fn saveBin_5(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax_5(&self, o: WinViewport) -> bool { todo!() }
    pub fn equals_10(&self, o: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn equals_11(&self, o: Box<dyn std::any::Any + Send + Sync>, isStrict: bool) -> bool { todo!() }
    pub fn hashCode_5(&self) -> i32 { todo!() }
    pub fn dirty_5(&self) -> bool { todo!() }
    pub fn save_5(&self) -> Box<dyn KoBin> { todo!() }
    pub fn lastBin_5(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn discard_8(&self, zone: KoZone::defaultZone()) { todo!() }
    pub fn close_5(&self) { todo!() }
    pub fn cleanFollow_3(&self) { todo!() }
    pub fn cleanCache_3(&self) { todo!() }
    pub fn follow_3(&self, parent: WinViewport) { todo!() }
    pub fn unFollow_3(&self) { todo!() }
    pub fn hasParent_3(&self) -> bool { todo!() }
    pub fn parent_3(&self) -> WinViewport { todo!() }
    pub fn discard_9(&self) { todo!() }
    pub fn cleanParent_3(&self) { todo!() }
    pub fn isPlaceHolder(&self) -> bool { todo!() }
    pub fn hasPlaceHolder(&self) -> bool { todo!() }
    pub fn deletePlaceHolder(&self) { todo!() }
    pub fn putPlaceHolder(&self, value: bool) { todo!() }
}
