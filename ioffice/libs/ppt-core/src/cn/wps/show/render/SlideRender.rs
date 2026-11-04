#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code)]

// Java: import android.graphics.Canvas
// Java: import cn.wps.base.converter.UnitsConverter
use crate::cn::wps::base::converter::UnitsConverter::UnitsConverter;
// Java: import cn.wps.show.app.slide.ISlide
// Java: import cn.wps.show.app.slide.KmoSlide
use crate::cn::wps::show::util::layout_shape::KmoSlide;
// Java: import cn.wps.show.app.slide.KmoSlideLayout
// Java: import cn.wps.show.app.slide.KmoSlideMaster
// Java: import cn.wps.show.render.cache.SharedObjects
// Java: import cn.wps.show.util.RenderLogDumper
// Java: import cn.wps.show.util.TableOpLogDumper
use crate::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use crate::cn::wps::show::render::ShowTypoShape::ShowTypoShape;
use crate::cn::wps::show::util::DrawFlags::DrawFlags;
use crate::cn::wps::show::util::layout_shape::KmoShowShape;

#[derive(Clone)]
pub enum LayerItem {
    WhiteBackground { w: f32, h: f32 },
    Background { w: f32, h: f32, fill: Option<u32> },
    Shape(ShowTypoShape),
}

pub struct Canvas {}
pub struct ISlide {}
pub struct MasterSlideRender {}
pub struct BackSlideRender {}
pub struct SharedObjects {}

pub struct SlideRenderContext {
    pub mChangeIndex: i32,
}

// java public class SlideRender extends SlideRenderBase
pub struct SlideRender {
    // java protected int slideDrawFlags = 0;
    pub slideDrawFlags: i32,
    // java protected int followerDrawFlags = 0;
    pub followerDrawFlags: i32,
    // java protected SlideRenderContext mSlideContext
    mSlideContext: SlideRenderContext,
}

impl Default for SlideRender {
    fn default() -> Self {
        Self {
            slideDrawFlags: 0,
            followerDrawFlags: 0,
            mSlideContext: SlideRenderContext { mChangeIndex: -1 },
        }
    }
}

impl SlideRender {
    pub fn layer_order(
        slide: Option<&KmoSlide>,
        slideDrawFlags: i32,
        followerDrawFlags: i32,
    ) -> Vec<LayerItem> {
        let mut r = Self {
            slideDrawFlags,
            followerDrawFlags,
            mSlideContext: SlideRenderContext { mChangeIndex: -1 },
        };
        r.draw(slide)
    }

    fn film_one(sp: &KmoShowShape, out: &mut Vec<LayerItem>) {
        if sp.isHidden() {
            return;
        }
        // Java GroupRender.drawGroup: type==0 with children → film children, not the empty group AABB.
        if sp.r#type() == 0 {
            for child in &sp.children {
                Self::film_one(child, out);
            }
            return;
        }
        let geom = util::cn::wps::graphics::RectF::RectF::new_with(
            sp.transform2DX as f32,
            sp.transform2DY as f32,
            (sp.transform2DX + sp.transform2DW) as f32,
            (sp.transform2DY + sp.transform2DH) as f32,
        );
        out.push(LayerItem::Shape(ShowTypoShape::typo_shape_snapshot(
            Some(ConvertKmo::from_layout(sp).into_shape()),
            None,
            Some(geom),
        )));
    }

    fn film_shapes(tree: &crate::cn::wps::show::util::layout_shape::KmoShapeTree) -> Vec<LayerItem> {
        let mut out = Vec::new();
        let n = tree.shapeCount();
        for i in 0..n {
            let sp = tree.shapeAt(i);
            Self::film_one(&sp, &mut out);
        }
        out
    }

    // java public static SlideRender getOrAllocThumbSlideRender()
    pub fn getOrAllocThumbSlideRender() -> SlideRender {
        // rust: assign THUMB flags (java literals 29909122 / 548993)
        Self {
            slideDrawFlags: DrawFlags::NORMAL_THUMBNAIL_SLIDE,
            followerDrawFlags: DrawFlags::NORMAL_THUMBNAIL_SLIDE_FOLLOWER,
            mSlideContext: SlideRenderContext { mChangeIndex: -1 },
        }
    }

    // java public static void reuseThumbSlideRender(SlideRender slideRender)
    pub fn reuseThumbSlideRender(_slideRender: SlideRender) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java SharedObjects.sliderRenderPool.returnObject(slideRender);
        */
    }

    // java public static SlideRender getOrAllocReadSlideRender()
    pub fn getOrAllocReadSlideRender() -> SlideRender {
        // rust: assign READ_NORMAL flags (java literals 21505154 / 532609)
        Self {
            slideDrawFlags: DrawFlags::READ_NORMAL_SLIDE,
            followerDrawFlags: DrawFlags::READ_NORMAL_SLIDE_FOLLOWER,
            mSlideContext: SlideRenderContext { mChangeIndex: -1 },
        }
    }

    // java public static void reuseReadSlideRender(SlideRender slideRender)
    pub fn reuseReadSlideRender(_slideRender: SlideRender) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static SlideRender getOrAllocEditSlideRender()
    pub fn getOrAllocEditSlideRender() -> SlideRender {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // rust: slideDrawFlags = 15474692; follower = 794753
        */
    }

    // java public static void reuseEditSlideRender(SlideRender slideRender)
    pub fn reuseEditSlideRender(_slideRender: SlideRender) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static SlideRender getOrAllocMasterSlideRender()
    pub fn getOrAllocMasterSlideRender() -> SlideRender {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // rust: slideDrawFlags = 180; follower = 129
        */
    }

    // java public static void reuseMasterSlideRender(MasterSlideRender slideRender)
    pub fn reuseMasterSlideRender(_slideRender: MasterSlideRender) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static SlideRender getOrAllocBackSlideRender()
    pub fn getOrAllocBackSlideRender() -> SlideRender {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // rust: slideDrawFlags = 450; follower = 385
        */
    }

    // java public static void reuseBackSlideRender(BackSlideRender slideRender)
    pub fn reuseBackSlideRender(_slideRender: BackSlideRender) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public static SlideRender createPrintSlideRender()
    pub fn createPrintSlideRender() -> SlideRender {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // rust: slideDrawFlags = 533122; follower = 533121
        */
    }

    // java public void draw(Canvas canvas, KmoSlide slide)
    pub fn draw(&mut self, slide: Option<&KmoSlide>) -> Vec<LayerItem> {
        // java if (slide != null)
        let Some(slide) = slide else {
            return Vec::new();
        };
        let mut layers = Vec::new();
        // java if (!this.mSlideContext.isPartChange())
        let isPartChange = self.mSlideContext.mChangeIndex != -1 && slide.r#type() == 0;
        if !isPartChange {
            // java this.drawUnderSlide(canvas, slide);
            layers.extend(self.drawUnderSlide(slide));
        }
        // java this.mSlideContext.reset(slide, this.slideDrawFlags);
        // java this.renderTool.drawSlide(canvas, this.mSlideContext);
        layers.extend(Self::film_shapes(slide.spTree()));
        layers
    }

    // java public void drawUnderSlide(Canvas canvas, KmoSlide slide)
    pub fn drawUnderSlide(&mut self, slide: &KmoSlide) -> Vec<LayerItem> {
        let mut layers = Vec::new();
        // java float width = UnitsConverter.render().fromEmuX((float)slide.getPresentation().slideWidth());
        let units = UnitsConverter::render();
        let width = units.fromEmuX(slide.getPresentation().slideWidth() as f32);
        // java float height = UnitsConverter.render().fromEmuY((float)slide.getPresentation().slideHeight());
        let height = units.fromEmuY(slide.getPresentation().slideHeight() as f32);
        // java this.renderTool.drawSlideWhiteBackground(canvas, width, height);
        layers.push(LayerItem::WhiteBackground { w: width, h: height });
        // java this.renderTool.drawBackgroundShape(...)
        // BackgroundShapeRender._begin: hasBackground && (drawFlags & 8)==0
        if slide.background().is_some() && 0 == (self.slideDrawFlags & DrawFlags::NOT_DRAW_BACKGROUND_SHAPE) {
            layers.push(LayerItem::Background {
                w: width,
                h: height,
                fill: slide.bg_fill,
            });
        }
        layers.extend(self.drawShapesOnLayout(slide));
        layers
    }

    // java private void drawShapesOnLayout(Canvas canvas, KmoSlide slide)
    pub fn drawShapesOnLayout(&mut self, slide: &KmoSlide) -> Vec<LayerItem> {
        let mut layers = Vec::new();
        // java KmoSlideLayout layout = slide.layout();
        let layout = slide.layout();
        // java if (null != layout && layout.isShowMasterShape() && slide.isShowMasterShape())
        if let Some(ref layout) = layout {
            if layout.isShowMasterShape() && slide.isShowMasterShape() {
                // java KmoSlideMaster master = layout.master();
                if let Some(master) = layout.master() {
                    layers.extend(Self::film_shapes(&master.spTree()));
                }
            }
        }
        // java if (slide.isShowMasterShape())
        if slide.isShowMasterShape() {
            if let Some(ref layout) = layout {
                if let Some(tree) = layout.spTree() {
                    layers.extend(Self::film_shapes(tree.as_ref()));
                }
            }
        }
        layers
    }

    // java public void draw(Canvas canvas, ISlide slide)
    pub fn draw__islide(&mut self, _slide: Option<&ISlide>) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (slide != null)
        // java this.draw(canvas, (KmoSlide)slide);
        */
    }
}
