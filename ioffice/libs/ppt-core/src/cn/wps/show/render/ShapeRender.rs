#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code)]

// Java: import android.graphics.Canvas
// Java: import android.graphics.DashPathEffect
// Java: import android.graphics.Matrix
// Java: import android.graphics.Paint
// Java: import android.graphics.Paint.Style
// Java: import cn.wps.base.converter.UnitsConverter
// Java: import cn.wps.drawing.app.text.KmoTextBody
// Java: import cn.wps.graphics.RectF
// Java: import cn.wps.moffice.drawing.Shape
// Java: import cn.wps.moffice.drawing.graphics.RenderModel
// Java: import cn.wps.show.app.selection.KmoSelection
// Java: import cn.wps.show.app.shape.KmoShowShape
// Java: import cn.wps.show.app.text.KmoTextOperator
// Java: import cn.wps.show.drawable.ModularGroupPicPromptDrawable
// Java: import cn.wps.show.drawable.PicPromptDrawable
// Java: import cn.wps.show.graphics.shape3d.Object3DExt
// Java: import cn.wps.show.moffice.drawing.render.DrawingRender
// Java: import cn.wps.show.moffice.drawing.render.ITypoShape
// Java: import cn.wps.show.moffice.drawing.render.ShapeEnv
// Java: import cn.wps.show.moffice.drawing.util.AnyObjPool
// Java: import cn.wps.show.moffice.util.CanvasUtil
// Java: import cn.wps.show.render.util.IShowShapeRender
// Java: import cn.wps.show.render.util.ShowShapeEnv
// Java: import cn.wps.show.util.DrawFlags
use crate::cn::wps::show::util::DrawFlags::DrawFlags;
// Java: import cn.wps.show.util.ShapeUtil
use crate::cn::wps::show::render::ShowTypoShape::ShowTypoShape;

pub struct ShapeEnv {
    pub draw_modular_default_pic: bool,
}

impl ShapeEnv {
    pub fn setDrawModularDefaultPic(&mut self, v: bool) {
        self.draw_modular_default_pic = v;
    }
    pub fn isDrawModularDefaultPic(&self) -> bool {
        self.draw_modular_default_pic
    }
}

pub struct Paint {}
pub struct PicPromptDrawable {}
pub struct ModularGroupPicPromptDrawable {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BeginFilter {
    Pass,
    Skip,
}

// java public class ShapeRender extends IShowShapeRender
pub struct ShapeRender {
    // java protected ShapeEnv mEnv = null;
    mEnv: Option<ShapeEnv>,
    // java protected ShowTypoShape mTypoShape;
    mTypoShape: Option<ShowTypoShape>,
    // java protected int drawFlags;
    drawFlags: i32,
    // java protected Paint mPaint;
    mPaint: Paint,
    // java protected float[] mDash;
    mDash: Vec<f32>,
    // java protected PicPromptDrawable mPicDrawable;
    mPicDrawable: Option<PicPromptDrawable>,
    // java protected ModularGroupPicPromptDrawable mModularPicDrawable;
    mModularPicDrawable: Option<ModularGroupPicPromptDrawable>,
}

impl ShapeRender {
    pub fn begin_filter(typo: Option<ShowTypoShape>, drawFlags: i32) -> BeginFilter {
        let mut env = ShapeEnv {
            draw_modular_default_pic: true,
        };
        let mut sr = Self::ShapeRender(Some(env), typo, drawFlags);
        if sr._begin() {
            BeginFilter::Pass
        } else {
            BeginFilter::Skip
        }
    }

    // java public ShapeRender(ShapeEnv env, ShowTypoShape typoShape, int drawFlags)
    pub fn ShapeRender(
        env: Option<ShapeEnv>,
        typoShape: Option<ShowTypoShape>,
        drawFlags: i32,
    ) -> Self {
        let mut mEnv = env.unwrap_or(ShapeEnv {
            draw_modular_default_pic: true,
        });
        // java if (0 != (drawFlags & 16777216))
        if 0 != (drawFlags & DrawFlags::NOT_DRAW_MODULAR_GROUP_PIC_PLACEHOLDER) {
            // java this.mEnv.setDrawModularDefaultPic(false);
            mEnv.setDrawModularDefaultPic(false);
        } else {
            // java this.mEnv.setDrawModularDefaultPic(true);
            mEnv.setDrawModularDefaultPic(true);
        }
        Self {
            mEnv: Some(mEnv),
            mTypoShape: typoShape,
            drawFlags,
            mPaint: Paint {},
            mDash: Vec::new(),
            mPicDrawable: None,
            mModularPicDrawable: None,
        }
    }

    // java public int getShapeId()
    pub fn getShapeId(&mut self) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.mTypoShape == null ? -1 : this.mTypoShape.getRenderModel().getShapeID();
        */
    }

    // java public boolean _begin()
    pub fn _begin(&self) -> bool {
        let Some(typo) = &self.mTypoShape else {
            return false;
        };
        // java if (this.mTypoShape.isHidden())
        if typo.isHidden() {
            return false;
        }
        // java else if (0 != (this.drawFlags & 1) && this.mTypoShape.isPlaceholder())
        if 0 != (self.drawFlags & DrawFlags::NOT_DRAW_PLACEHOLDER) && typo.isPlaceholder() {
            return false;
        }
        // java else if (0 != (this.drawFlags & 32) && this.mTypoShape.isPlaceholder() && this.isMinerPlaceHolder(...))
        if 0 != (self.drawFlags & DrawFlags::NOT_DRAW_PLACEHOLDER_MINOR)
            && typo.isPlaceholder()
            && self.isMinerPlaceHolder(typo.placeholderType())
        {
            return false;
        }
        // java return this.mEnv.isDrawModularDefaultPic() || !this.mTypoShape.isModularGroupDefaultPic();
        let env = self.mEnv.as_ref().unwrap();
        env.isDrawModularDefaultPic() || !typo.isModularGroupDefaultPic()
    }

    // java private boolean isMinerPlaceHolder(int type)
    pub fn isMinerPlaceHolder(&self, r#type: i32) -> bool {
        // java switch (type) case 4/5/6/7: true; default: false
        matches!(r#type, 4 | 5 | 6 | 7)
    }

    // java private void update3DDrawTag()
    pub fn update3DDrawTag(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java public void _render()
    pub fn _render(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java private void renderPlaceHolderDashFrame(float dash, ShowTypoShape typoShape)
    pub fn renderPlaceHolderDashFrame(&mut self, dash: f32, typoShape: ShowTypoShape) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java private void renderDashBorder(float dash, ShowTypoShape typoShape)
    pub fn renderDashBorder(&mut self, dash: f32, typoShape: ShowTypoShape) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java private void renderLayoutBorder()
    pub fn renderLayoutBorder(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java private void renderPicPhHolder()
    pub fn renderPicPhHolder(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }

    // java private void renderPicPhHolderIcon(float scale)
    pub fn renderPicPhHolderIcon(&mut self, scale: f32) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }
}
