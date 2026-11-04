# OpenMuse UI CJK subset

`OpenMuseUI-CJK.otf` is a 341 character subset of [Noto Sans CJK SC Regular](https://github.com/notofonts/noto-cjk/blob/main/Sans/OTF/SimplifiedChinese/NotoSansCJKsc-Regular.otf), generated with fontTools from the Chinese characters currently used by the Desktop/Web workbench Dart UI. The source font is licensed under the SIL Open Font License 1.1; the license text is preserved in [OFL.txt](OFL.txt).

This small UI font prevents missing glyphs during Flutter Web's initial paint. It is not a document font and must not be used for Office content layout. Office fonts are versioned separately with the future shared Office renderer.
