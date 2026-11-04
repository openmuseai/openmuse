// Java: public interface ExtendedPackageProperties
pub trait ExtendedPackageProperties {
    // Java: abstract String getApplicationProperty()
    fn getApplicationProperty(&self) -> Option<String>;

    // Java: abstract void setApplicationProperty(String var1)
    fn setApplicationProperty(&mut self, var1: &str);

    // Java: abstract String getAppVersionProperty()
    fn getAppVersionProperty(&self) -> Option<String>;

    // Java: abstract void setAppVersionProperty(String var1)
    fn setAppVersionProperty(&mut self, var1: &str);

    // Java: abstract Integer getCharactersProperty()
    fn getCharactersProperty(&self) -> Option<i32>;

    // Java: abstract void setCharactersProperty(int var1)
    fn setCharactersProperty(&mut self, var1: i32);

    // Java: abstract String getCompanyProperty()
    fn getCompanyProperty(&self) -> Option<String>;

    // Java: abstract void setCompanyProperty(String var1)
    fn setCompanyProperty(&mut self, var1: &str);

    // Java: abstract String getDigSigProperty()
    fn getDigSigProperty(&self) -> Option<String>;

    // Java: abstract void setDigSigProperty(String var1)
    fn setDigSigProperty(&mut self, var1: &str);

    // Java: abstract Integer getDocSecurityProperty()
    fn getDocSecurityProperty(&self) -> Option<i32>;

    // Java: abstract void setDocSecurityProperty(int var1)
    fn setDocSecurityProperty(&mut self, var1: i32);

    // Java: abstract String getHeadingPairsProperty()
    fn getHeadingPairsProperty(&self) -> Option<String>;

    // Java: abstract void setHeadingPairsProperty(String var1)
    fn setHeadingPairsProperty(&mut self, var1: &str);

    // Java: abstract Integer getHiddenSlidesProperty()
    fn getHiddenSlidesProperty(&self) -> Option<i32>;

    // Java: abstract void setHiddenSlidesProperty(int var1)
    fn setHiddenSlidesProperty(&mut self, var1: i32);

    // Java: abstract String getHLinksProperty()
    fn getHLinksProperty(&self) -> Option<String>;

    // Java: abstract void setHLinksProperty(String var1)
    fn setHLinksProperty(&mut self, var1: &str);

    // Java: abstract String getHyperlinkBaseProperty()
    fn getHyperlinkBaseProperty(&self) -> Option<String>;

    // Java: abstract void setHyperlinkBaseProperty(String var1)
    fn setHyperlinkBaseProperty(&mut self, var1: &str);

    // Java: abstract Boolean getHyperlinksChangedProperty()
    fn getHyperlinksChangedProperty(&self) -> Option<bool>;

    // Java: abstract void setHyperlinksChangedProperty(boolean var1)
    fn setHyperlinksChangedProperty(&mut self, var1: bool);

    // Java: abstract Integer getLinesProperty()
    fn getLinesProperty(&self) -> Option<i32>;

    // Java: abstract void setLinesProperty(int var1)
    fn setLinesProperty(&mut self, var1: i32);

    // Java: abstract Boolean getLinksUpToDateProperty()
    fn getLinksUpToDateProperty(&self) -> Option<bool>;

    // Java: abstract void setLinksUpToDateProperty(boolean var1)
    fn setLinksUpToDateProperty(&mut self, var1: bool);

    // Java: abstract String getManagerProperty()
    fn getManagerProperty(&self) -> Option<String>;

    // Java: abstract void setManagerProperty(String var1)
    fn setManagerProperty(&mut self, var1: &str);

    // Java: abstract Integer getMMClipsProperty()
    fn getMMClipsProperty(&self) -> Option<i32>;

    // Java: abstract void setMMClipsProperty(int var1)
    fn setMMClipsProperty(&mut self, var1: i32);

    // Java: abstract Integer getNotesProperty()
    fn getNotesProperty(&self) -> Option<i32>;

    // Java: abstract void setNotesProperty(int var1)
    fn setNotesProperty(&mut self, var1: i32);

    // Java: abstract Integer getPagesProperty()
    fn getPagesProperty(&self) -> Option<i32>;

    // Java: abstract void setPagesProperty(int var1)
    fn setPagesProperty(&mut self, var1: i32);

    // Java: abstract Integer getParagraphsProperty()
    fn getParagraphsProperty(&self) -> Option<i32>;

    // Java: abstract void setParagraphsProperty(int var1)
    fn setParagraphsProperty(&mut self, var1: i32);

    // Java: abstract String getPresentationFormatProperty()
    fn getPresentationFormatProperty(&self) -> Option<String>;

    // Java: abstract void setPresentationFormatProperty(String var1)
    fn setPresentationFormatProperty(&mut self, var1: &str);

    // Java: abstract Boolean getScaleCropProperty()
    fn getScaleCropProperty(&self) -> Option<bool>;

    // Java: abstract void setScaleCropProperty(boolean var1)
    fn setScaleCropProperty(&mut self, var1: bool);

    // Java: abstract Boolean getSharedDocProperty()
    fn getSharedDocProperty(&self) -> Option<bool>;

    // Java: abstract void setSharedDocProperty(boolean var1)
    fn setSharedDocProperty(&mut self, var1: bool);

    // Java: abstract Integer getSlidesProperty()
    fn getSlidesProperty(&self) -> Option<i32>;

    // Java: abstract void setSlidesProperty(int var1)
    fn setSlidesProperty(&mut self, var1: i32);

    // Java: abstract String getTemplateProperty()
    fn getTemplateProperty(&self) -> Option<String>;

    // Java: abstract void setTemplateProperty(String var1)
    fn setTemplateProperty(&mut self, var1: &str);

    // Java: abstract String getTitlesOfPartsProperty()
    fn getTitlesOfPartsProperty(&self) -> Option<String>;

    // Java: abstract void setTitlesOfPartsProperty(String var1)
    fn setTitlesOfPartsProperty(&mut self, var1: &str);

    // Java: abstract Integer getTotalTimeProperty()
    fn getTotalTimeProperty(&self) -> Option<i32>;

    // Java: abstract void setTotalTimeProperty(int var1)
    fn setTotalTimeProperty(&mut self, var1: i32);

    // Java: abstract Integer getWordCountProperty()
    fn getWordCountProperty(&self) -> Option<i32>;

    // Java: abstract void setWordCountProperty(int var1)
    fn setWordCountProperty(&mut self, var1: i32);

    // Java: abstract Integer getNumberCharactersProperty()
    fn getNumberCharactersProperty(&self) -> Option<i32>;

    // Java: abstract void setCharactersWithSpacesProperty(int var1)
    fn setCharactersWithSpacesProperty(&mut self, var1: i32);

}
