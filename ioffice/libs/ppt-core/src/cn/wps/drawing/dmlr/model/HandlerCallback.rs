#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

// Java: import cn.wps.drawing.app.diagrams.DiagramPair
#[cfg(feature = "full")]
use crate::cn::wps::drawing::app::diagrams::DiagramPair::DiagramPair;
// Java: import cn.wps.drawing.data.blip.Blip
#[cfg(feature = "full")]
use crate::cn::wps::drawing::data::blip::Blip::Blip;
// Java: import cn.wps.drawing.data.color.Color
#[cfg(feature = "full")]
use crate::cn::wps::drawing::data::color::Color::Color;
// Java: import cn.wps.drawing.data.hyperlink.Hyperlink
#[cfg(feature = "full")]
use crate::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
// Java: import cn.wps.drawing.data.media.Media
#[cfg(feature = "full")]
use crate::cn::wps::drawing::data::media::Media::{
    AudioFile, EmbeddedWAVAudioFile, Media, QuickTimeFile, VideoFile,
};
// Java: import cn.wps.drawing.data.model3d.Model3D
#[cfg(feature = "full")]
use crate::cn::wps::drawing::data::model3d::Model3D::Model3D;
// Java: import cn.wps.drawing.data.shape.Shape
#[cfg(feature = "full")]
use crate::cn::wps::drawing::data::shape::Shape::Shape;
// Java: import cn.wps.drawing.data.shape.ShapePr
#[cfg(feature = "full")]
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
// Java: import cn.wps.drawing.data.style.ShapeStyle
#[cfg(feature = "full")]
use crate::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
// Java: import cn.wps.drawing.data.text.TextRun
#[cfg(feature = "full")]
use crate::cn::wps::drawing::data::text::TextRun::TextRun;
// Java: import cn.wps.show.data.custdatalst.CustDataLst
#[cfg(feature = "full")]
use crate::cn::wps::show::data::custdatalst::CustDataLst::TagLst;
// Java: import cn.wps.util.manager.FileManager
#[cfg(feature = "full")]
use crate::cn::wps::util::manager::FileManager::FileManager;
// Java: import java.util.Map
use std::collections::HashMap;

// Java: public class HandlerCallback
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HandlerCallbackState {
    // Java field: int mode
    pub mode: i32,
}

// Java: HandlerCallback()
pub fn HandlerCallback() -> HandlerCallbackState {
    HandlerCallbackState { mode: 0 }
}

// Java: HandlerCallback(int mode)
pub fn HandlerCallback__mode(mode: i32) -> HandlerCallbackState {
    let mut state = HandlerCallbackState { mode: 0 };
    // Java: this.mode = mode;
    state.mode = mode;
    state
}

// Java: public class HandlerCallback
pub trait HandlerCallback {
    type DiagramPair;
    type Blip;
    type Color;
    type Hyperlink;
    type Media;
    type AudioFile;
    type QuickTimeFile;
    type VideoFile;
    type EmbeddedWAVAudioFile;
    type Model3D;
    type Shape;
    type ShapePr;
    type ShapeStyle;
    type TextRun;
    type TagLst;
    type FileManager;

    // Java: boolean isGVml()
    fn isGVml(&self) -> bool {
        // Java: return this.mode == 2;
        self.callback_state().mode == 2
    }

    // Java: boolean isPPTX()
    fn isPPTX(&self) -> bool {
        // Java: return this.mode == 0;
        self.callback_state().mode == 0
    }

    // Java: boolean isPPT14Slides()
    fn isPPT14Slides(&self) -> bool {
        // Java: return this.mode == 1;
        self.callback_state().mode == 1
    }

    // Java: boolean isArtTable()
    fn isArtTable(&self) -> bool {
        // Java: return this.mode == 3;
        self.callback_state().mode == 3
    }

    // Java: void load()
    fn load(&mut self) {}

    // Java: void onTags(String id, CustDataLst.TagLst tagLst)
    fn onTags(&mut self, _id: Option<&str>, _tagLst: Option<&mut Self::TagLst>) {}

    // Java: void onBlipEmbed(String embed, Blip blip)
    fn onBlipEmbed(&mut self, _embed: Option<&str>, _blip: Option<&mut Self::Blip>) {}

    // Java: void onBlipLink(String link, Blip blip)
    fn onBlipLink(&mut self, _link: Option<&str>, _blip: Option<&mut Self::Blip>) {}

    // Java: void onAudioFileLink(String link, Media.AudioFile audioFile)
    fn onAudioFileLink(&mut self, _link: Option<&str>, _audioFile: Option<&mut Self::AudioFile>) {}

    // Java: void onExtMediaFileLink(String link, Media media)
    fn onExtMediaFileLink(&mut self, _link: Option<&str>, _media: Option<&mut Self::Media>) {}

    // Java: void onQuickTimeFileLink(String link, Media.QuickTimeFile quickTimeFile)
    fn onQuickTimeFileLink(
        &mut self,
        _link: Option<&str>,
        _quickTimeFile: Option<&mut Self::QuickTimeFile>,
    ) {
    }

    // Java: void onVideoFileLink(String link, Media.VideoFile videoFile)
    fn onVideoFileLink(&mut self, _link: Option<&str>, _videoFile: Option<&mut Self::VideoFile>) {}

    // Java: void onEmbeddedWAVAudioFileEmbed(String embed, Media.EmbeddedWAVAudioFile embeddedWAVAudioFile)
    fn onEmbeddedWAVAudioFileEmbed(
        &mut self,
        _embed: Option<&str>,
        _embeddedWAVAudioFile: Option<&mut Self::EmbeddedWAVAudioFile>,
    ) {
    }

    // Java: void onHyperlinkRid(String rId, Hyperlink hyperlink)
    fn onHyperlinkRid(&mut self, _rId: Option<&str>, _hyperlink: Option<&mut Self::Hyperlink>) {}

    // Java: void onHyperlinkAction(String action, Hyperlink hyperlink)
    fn onHyperlinkAction(
        &mut self,
        _action: Option<&str>,
        _hyperlink: Option<&mut Self::Hyperlink>,
    ) {
    }

    // Java: void onShapeId(Integer originShapeId, Integer coreShapeId)
    fn onShapeId(&mut self, _originShapeId: Option<i32>, _coreShapeId: Option<i32>) {}

    // Java: void onConnectorShape(Integer connectorShapeId)
    fn onConnectorShape(&mut self, _connectorShapeId: Option<i32>) {}

    // Java: Integer getCoreShapeId(Integer originShapeId)
    fn getCoreShapeId(&self, originShapeId: Option<i32>) -> Option<i32> {
        // Java: return originShapeId;
        originShapeId
    }

    // Java: int onOleObject(String rId)
    fn onOleObject(&mut self, _rId: Option<&str>) -> i32 {
        // Java: return -1;
        -1
    }

    // Java: void onChartRelationShip(String rId, Shape shape)
    fn onChartRelationShip(&mut self, _rId: Option<&str>, _shape: Option<&mut Self::Shape>) {}

    // Java: public Map<String, Shape> getChartMap()
    fn getChartMap(&self) -> Option<&HashMap<String, Self::Shape>> {
        // Java: return null;
        None
    }

    // Java: void addDiagramPair(int smartArtId, String colorsId, String dataId, String layoutId, String quickId)
    fn addDiagramPair(
        &mut self,
        _smartArtId: i32,
        _colorsId: Option<&str>,
        _dataId: Option<&str>,
        _layoutId: Option<&str>,
        _quickId: Option<&str>,
    ) {
    }

    // Java: public Map<Integer, DiagramPair> getDiagramPairs()
    fn getDiagramPairs(&self) -> Option<&HashMap<i32, Self::DiagramPair>> {
        // Java: return null;
        None
    }

    // Java: int onInk(String rId, FileManager fileManager)
    fn onInk(&mut self, _rId: Option<&str>, _fileManager: Option<&mut Self::FileManager>) -> i32 {
        // Java: return -1;
        -1
    }

    // Java: void onModel3D(String rId, Model3D model3D)
    fn onModel3D(&mut self, _rId: Option<&str>, _model3D: Option<&mut Self::Model3D>) {}

    // Java: void onAdjustThemeColor(Color color, int belonger)
    fn onAdjustThemeColor(&mut self, _color: Option<&mut Self::Color>, _belonger: i32) {}

    // Java: void onMergeThemeTypeface(TextRun run)
    fn onMergeThemeTypeface(&mut self, _run: Option<&mut Self::TextRun>) {}

    // Java: void onConvertStyle2ShapeProps(ShapeStyle shapeStyle, ShapePr shapePr)
    fn onConvertStyle2ShapeProps(
        &mut self,
        _shapeStyle: Option<&Self::ShapeStyle>,
        _shapePr: Option<&mut Self::ShapePr>,
    ) {
    }

    fn callback_state(&self) -> &HandlerCallbackState;
}
