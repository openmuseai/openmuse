#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.ShareView.ShareViewRunPropFilter
use util::moffice::define::FontName::FontName;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::graph::IRunPropFilter::IRunPropFilter;

// java public class ShareViewRunPropFilter implements IRunPropFilter
pub struct ShareViewRunPropFilter {
    share_view_property: Option<PropertySet>,
    view: *const ViewEnv,
}

impl ShareViewRunPropFilter {
    // java ShareViewRunPropFilter(ViewEnv viewEnv)
    pub fn new(view: &ViewEnv) -> Self {
        Self {
            share_view_property: view.getShareViewProperty(),
            view: view as *const ViewEnv,
        }
    }
}

impl IRunPropFilter for ShareViewRunPropFilter {
    // java public PropertySet doFilter(PropertySet prop, IChpxTable.Entry run, boolean revisionFinal, LayoutState layoutState)
    fn doFilter(&self, prop: &PropertySet, _run: &dyn ChpxEntry, _revision_final: bool, layout: &mut LayoutState) -> PropertySet {
        let Some(share) = self.share_view_property.as_ref() else {
            return prop.clone();
        };
        // java MutablePropertySet mutableProp = new MutablePropertySet(prop);
        let mut mutable = MutablePropertySet::new_from_ps(prop);
        // java String oldFont = prop.getString(3, "Times New Roman");
        let old_font = prop.getString__int_string(3, "Times New Roman");
        // java mutableProp.add(this.shareViewPropert);
        mutable.add(share);
        // java if (2 == layoutState.curPage.getType())
        if layout.curPage.as_ref().map(|page| page.getType()).unwrap_or(0) == 2 {
            let font_size = unsafe { (*self.view).getShareViewFontsize() };
            mutable.setFloat(10, font_size);
            mutable.setFloat(37, font_size);
        }
        // java if (FontName.isSpecialFontName(oldFont)) mutableProp.setString(3, oldFont);
        if FontName::isSpecialFontName(&old_font) {
            mutable.setString(3, old_font);
        }
        mutable.freeze()
    }
}
