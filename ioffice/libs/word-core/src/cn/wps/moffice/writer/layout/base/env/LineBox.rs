#![allow(non_snake_case)]

/// wr-12 I/O: wr-11 ascent/descent + PAPX 220/217/214 → line height and baseline.
///
/// Java: `LineHeightLayouter.calcLineHeightInfoByLineSpacingRule` then
/// `LineLayouter.calcParaBefore` / `calcParaAfter` and
/// `setHeight(mLineHeight + mParaBefore + mParaAfter)`.
/// Do **not** call wr-04 `layout/base/text/LineHeight.rs` (`ceil(pt×20)×spacing`).

/// PAPX 220: `rule` 0=auto, 1=exact, 2=atLeast. Default Java `new LineSpacing(0, 1.0F)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineSpacing {
    pub rule: i32,
    pub spacing: f32,
}

impl LineSpacing {
    pub fn auto(spacing: f32) -> Self {
        LineSpacing { rule: 0, spacing }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineBox {
    pub height: i32,
    pub baseline: i32,
}

impl LineBox {
    fn java_round(x: f32) -> i32 {
        (x + 0.5f32) as i32
    }

    // java float channelHeight = this.mMaxAscent + this.mMaxDescent;
    // java this.mBaseHeight = (int)(this.mMaxAscent + this.mMaxDescent + 0.5F);
    fn channel_and_base(ascent: i32, descent: i32) -> (f32, i32) {
        let channel = ascent as f32 + descent as f32;
        (channel, Self::java_round(channel))
    }

    // java if (1 == spacingRule) { mLineHeight = (int)(Math.abs(lineSpacing) + 0.5F); }
    // java else if (2 == spacingRule) { mLineHeight = (int)(Math.max(getChannelHeight(), lineSpacing) + 0.5F); }
    // java else { extraSpace = mBaseHeight * (lineSpacing - 1); mLineHeight = (int)(getChannelHeight() + extraSpace + 0.5F); }
    fn line_height_from_spacing(ascent: i32, descent: i32, spacing: LineSpacing) -> i32 {
        let (channel, base_height) = Self::channel_and_base(ascent, descent);
        if 1 == spacing.rule {
            Self::java_round(spacing.spacing.abs())
        } else if 2 == spacing.rule {
            Self::java_round(channel.max(spacing.spacing))
        } else {
            let extra_space = (base_height as f32) * (spacing.spacing - 1.0);
            Self::java_round(channel + extra_space)
        }
    }

    // java this.mParaBefore = Math.max(0, cur.paraBefore - pre.paraAfter);
    fn para_before_applied(para_before: i32, prev_para_after: i32, first_of_para: bool) -> i32 {
        if !first_of_para {
            return 0;
        }
        let v = para_before - prev_para_after;
        if v < 0 {
            0
        } else {
            v
        }
    }

    // java if (!mIsCardBegin && !mIsFirstLineOfPage) { mParaAfter = cur.paraAfter; }
    fn para_after_applied(para_after: i32, last_of_para: bool) -> i32 {
        if last_of_para {
            para_after
        } else {
            0
        }
    }

    pub fn measure(
        ascent: i32,
        descent: i32,
        spacing: LineSpacing,
        para_before: i32,
        para_after: i32,
        first_of_para: bool,
        last_of_para: bool,
        prev_para_after: i32,
    ) -> LineBox {
        let line_height = Self::line_height_from_spacing(ascent, descent, spacing);
        let para_before = Self::para_before_applied(para_before, prev_para_after, first_of_para);
        let para_after = Self::para_after_applied(para_after, last_of_para);
        // java line.setHeight(this.mLineHeight + this.mParaBefore + this.mParaAfter);
        // java calcBaseLinePos CHANNEL_ALIGN_TOP: mBaselinePos = (int)(maxAscent + 0.5F);
        LineBox {
            height: line_height + para_before + para_after,
            baseline: para_before + Self::java_round(ascent as f32),
        }
    }
}
