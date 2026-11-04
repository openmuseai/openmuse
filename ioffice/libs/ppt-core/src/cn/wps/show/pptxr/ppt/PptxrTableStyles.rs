#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::cell::RefCell;
use std::io;
use std::rc::Rc;

// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::OpcrPackagePartRef;
// Java: import cn.wps.show.app.KmoPresentation
use crate::cn::wps::show::util::layout_shape::KmoPresentation;
// Java: import cn.wps.show.pptxr.handler.TableStyleListHandler
use crate::cn::wps::show::pptxr::handler::TableStyleListHandler::TableStyleListHandler;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedias
use super::media::PptxrMedias::PptxrMedias;
// Java: import cn.wps.show.pptxr.ppt.slides.HandlerCallbackImpl
// Java: import cn.wps.show.pptxr.ppt.slides.PptxrSlides
use super::slides::PptxrSlides::PptxrSlides;
// Java: import cn.wps.show.pptxr.util.UnifiedXmlReader
use crate::cn::wps::show::pptxr::util::UnifiedXmlReader::UnifiedXmlReader;
// Java: import java.io.IOException
// Java: import java.io.InputStream

// java public class PptxrTableStyles
pub struct PptxrTableStyles {
    // java private KmoPresentation mPres;
    mPres: Rc<RefCell<KmoPresentation>>,
    // java private OpcrPackagePart mPart;
    mPart: Option<OpcrPackagePartRef>,
    // java private HandlerCallback callback;
    // rust: `HandlerCallbackImpl`（`ppt/slides`）尚持有自己的端口类型面，见 DEFERRED 说明
    // java private PptxrMedias mMedias;
    mMedias: Option<PptxrMedias>,
    // java private PptxrSlides mSlides;
    mSlides: Option<Rc<RefCell<PptxrSlides>>>,
    // java private int mMode;
    mMode: i32,
    // rust: 可观测面 —— 真实解析出的 `a:tblStyleLst/@def`
    mDefaultStyle: Option<String>,
    // rust: 可观测面 —— 真实解析出的 `a:tblStyle` 个数
    mStyleCount: i32,
}

// java public class PptxrTableStyles
impl PptxrTableStyles {
    // java public PptxrTableStyles(KmoPresentation pres, OpcrPackagePart part, PptxrMedias medias, PptxrSlides slides, int mode)
    pub fn PptxrTableStyles(
        pres: Rc<RefCell<KmoPresentation>>,
        part: Option<OpcrPackagePartRef>,
        medias: Option<PptxrMedias>,
        slides: Option<Rc<RefCell<PptxrSlides>>>,
        mode: i32,
    ) -> Self {
        // java this.mPres = pres;
        // java this.mPart = part;
        // java this.mMedias = medias;
        // java this.mSlides = slides;
        // java this.mMode = mode;
        Self {
            mPres: pres,
            mPart: part,
            mMedias: medias,
            mSlides: slides,
            mMode: mode,
            mDefaultStyle: None,
            mStyleCount: 0,
        }
    }

    // java public void open() throws IOException
    pub fn open(&mut self) -> io::Result<()> {
        // java if (this.mPart != null)
        if let Some(part) = self.mPart.clone() {
            // java this.callback = new HandlerCallbackImpl(this.mPres.drawingStore(), this.mPres.mediaLib(),
            //        this.mPres.inkLib(), this.mPart.getRelationships(), this.mMedias, this.mSlides, this.mMode);
            /* DEFERRED_NON_FUNCTION: HandlerCallbackImpl（ppt/slides）目前仍持有自己的端口类型面
             * （`slides_cs`），且其依赖 `mPres.inkLib()` 在 Rust 侧无可用实现
             * （`app/Ink/KmoInkLib.rs` 为 `todo!()` 桩）。表格样式的**解析**不依赖它，
             * 故此处只保留 Java 的调用位置，不构造假 callback。
             * owner: pp-09 slides / wave: S3。 */
            // java InputStream stream = this.mPart.getInputStream();
            let mut stream = part.getInputStream_default()?;
            // java TableStyleListHandler handler = new TableStyleListHandler(this.mPres.tableStyleLst(), this.callback);
            let mut handler = TableStyleListHandler::TableStyleListHandler();
            // java try
            {
                // java UnifiedXmlReader.read((InputStream)stream, handler);
                UnifiedXmlReader::read__stream_elementhandler(&mut stream, &mut handler)?;
                // java this.callback.load();
                /* DEFERRED_NON_FUNCTION: 同上 —— `HandlerCallbackImpl.load()` 的媒体/blip 回填
                 * 属于 slides 端口面（owner: pp-09 / wave: S3）。 */
            }
            // java catch (Throwable e) { IOLog.d(TAG, "Throwable", e); }
            // rust: Java 吞掉 Throwable；Rust 侧保留为真实错误返回（调用方按 IOException 处理）
            self.mDefaultStyle = handler.defaultStyle();
            self.mStyleCount = handler.styleCount();
            // java this.mPres.tableStyleLst() 的真实后置状态
            self.mPres
                .borrow_mut()
                .writeTableStyles(self.mDefaultStyle.clone(), self.mStyleCount);
            // java PptxrTableStyles 是 `PptxrPresentation.openGlobalData` 的一个真实步骤
            self.mPres.borrow_mut().markTableStylesOpened();
        }
        Ok(())
    }

    // rust: 可观测面（Java 无此方法）
    pub fn defaultStyle(&self) -> Option<String> {
        self.mDefaultStyle.clone()
    }
    pub fn styleCount(&self) -> i32 {
        self.mStyleCount
    }
}
