#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: public class MetaFilePainter extends GdiPainter
pub struct MetaFilePainter {
    // Java field: MetaFileHeader mHeader
    mHeader: Option<i32>,
    // Java field: List<MetaFileRecord> mRecords
    mRecords: Option<i32>,
}

impl MetaFilePainter {
    // Java: MetaFilePainter(MetaFileHeader header, List<MetaFileRecord> records)
    pub fn new() -> Self {
        // Java: super(header.getHandles());
        // Java: this.mHeader = header;
        // Java: this.mRecords = records;
        MetaFilePainter {
            mHeader: None,
            mRecords: None,
        }
    }

    // Java: public void paint(Canvas canvas, float width, float height)
    pub fn paint_canvas_wh(&self, _canvas: i32, _width: f32, _height: f32) {
        // Java: this.paint(canvas, this.mRecords.size() - 1, (Path)null, (Paint)null, width, height);
    }

    // Java: public void paint(Canvas canvas)
    pub fn paint_canvas(&self, _canvas: i32) {
        // Java: this.paint(canvas, this.mRecords.size() - 1);
    }

    // Java: public void paint(Canvas canvas, int maxIndex)
    pub fn paint_canvas_max(&self, _canvas: i32, _maxIndex: i32) {
        // Java: this.paint(canvas, maxIndex, (Path)null, (Paint)null, -1.0F, -1.0F);
    }

    // Java: public void paint(Canvas canvas, Path clipPath, Paint paint, float width, float height)
    pub fn paint_canvas_clip(&self, _canvas: i32, _clipPath: i32, _paint: i32, _width: f32, _height: f32) {
        // Java: this.paint(canvas, this.mRecords.size() - 1, clipPath, paint, width, height);
    }

    // Java: private void paint(Canvas canvas, int maxIndex, Path clipPath, Paint paint, float width, float height)
    fn paint_full(&self, _canvas: i32, _maxIndex: i32, _clipPath: i32, _paint: i32, _width: f32, _height: f32) {
        // Java: boolean isEmf = this.mHeader instanceof EMFHeader;
        // Java: this.resetEnv(isEmf);
        // Java: if (isEmf)
        {
            // Java: EMFHeader emfHeader = (EMFHeader)this.mHeader;
            // Java: if (emfHeader.hasOffset())
            {
                // Java: canvas.translate(-emfHeader.getOffsetX(), -emfHeader.getOffsetY());
            }
            // Java: if (emfHeader.isEmfPlus())
            {
                // Java: float rw = (float)emfHeader.getRcFrameWidth();
                // Java: float rh = (float)emfHeader.getRcFrameHeight();
                // Java: float xdpmm = emfHeader.getxdpmm();
                // Java: float ydpmm = emfHeader.getydpmm();
                // Java: float dpiX = (float)emfHeader.getDpiX();
                // Java: float dpiY = (float)emfHeader.getDpiY();
                // Java: if (rw > 0.0F && rh > 0.0F && xdpmm > 0.0F && ydpmm > 0.0F && dpiX > 0.0F && dpiY > 0.0F)
                {
                    // Java: float w1 = (float)Math.ceil((double)(rw * 0.01F * xdpmm));
                    // Java: float h1 = (float)Math.ceil((double)(rh * 0.01F * ydpmm));
                    // Java: float w2 = (float)Math.ceil((double)(dpiX * rw * 0.01F / 25.4F));
                    // Java: float h2 = (float)Math.ceil((double)(dpiY * rh * 0.01F / 25.4F));
                    // Java: float scaleX = w2 / w1;
                    // Java: float scaleY = h2 / h1;
                    // Java: if (scaleX != 1.0F || scaleY != 1.0F)
                    {
                        // Java: canvas.scale(scaleX, scaleY);
                    }
                }
            }
        }
        // Java: else
        {
            // Java: Size size = null;
            // Java: int mapMode = 1;
            // Java: for(int i = 0; i <= maxIndex; ++i)
            {
                // Java: MetaFileRecord record = (MetaFileRecord)this.mRecords.get(i);
                // Java: if (record instanceof Wmf_SetMapMode)
                {
                    // Java: mapMode = ((Wmf_SetMapMode)record).getMapMode();
                }
                // Java: else if (record instanceof WMF_SetWindowExt)
                {
                    // Java: size = ((WMF_SetWindowExt)record).getSize();
                    // Java: if (size.getWidth() > 1.0F && size.getHeight() > 1.0F)
                    {
                        // Java: break;
                    }
                }
                // Java: else if (record instanceof WMF_SetViewportExt)
                {
                    // Java: size = ((WMF_SetViewportExt)record).getSize();
                    // Java: if (size.getWidth() > 1.0F && size.getHeight() > 1.0F)
                    {
                        // Java: break;
                    }
                }
            }
            // Java: if (this.isUseSetWindow(mapMode) && width > 1.0F && height > 1.0F && size != null && size.getWidth() > 1.0F && size.getHeight() > 1.0F)
            {
                // Java: float scaleX = width / size.getWidth();
                // Java: float scaleY = height / size.getHeight();
                // Java: if (scaleX != 1.0F || scaleY != 1.0F)
                {
                    // Java: canvas.scale(scaleX, scaleY);
                }
            }
        }
        // Java: canvas.save();
        // Java: ColorFilter colorFilter = null != paint ? paint.getColorFilter() : null;
        // Java: this.drawer = null != clipPath ? new GraphicsDrawer(this.dc, canvas, clipPath, colorFilter, isEmf) : new GraphicsDrawer(this.dc, canvas, (Path)null, (ColorFilter)null, isEmf);
        // Java: for(int i = 0; i <= maxIndex; ++i)
        {
            // Java: MetaFileRecord record = (MetaFileRecord)this.mRecords.get(i);
            // Java: record.render(this);
        }
        // Java: while(!this.stackDC.isEmpty())
        {
            // Java: this.restoreDC();
        }
        // Java: if (this.dc.path != null)
        {
            // Java: this.dc.getContext().resetTransformation();
            // Java: this.fillPath(this.dc.path);
            // Java: this.drawPath(this.dc.path);
            // Java: this.dc.setPath((GdiPath)null);
        }
        // Java: canvas.restore();
    }

    // Java: protected boolean isUseSetWindow(int mapMode)
    pub fn isUseSetWindow(&self, _mapMode: i32) -> bool {
        // Java: return mapMode == 8 || mapMode == 7 || mapMode == 1;
        false
    }

    // Java: public int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return this.mRecords.size();
        0
    }

    // Java: public MetaFileRecord getRecordAt(int pos)
    pub fn getRecordAt(&self, _pos: i32) -> i32 {
        // Java: return (MetaFileRecord)this.mRecords.get(pos);
        0
    }

    // Java: public MetaFileRecord getNextRecord(MetaFileRecord record)
    pub fn getNextRecord(&self, _record: i32) -> i32 {
        // Java: int size = this.mRecords.size();
        // Java: MetaFileRecord next = null;
        // Java: int i = 0;
        // Java: for(i = 0; i < size; ++i)
        {
            // Java: next = (MetaFileRecord)this.mRecords.get(i);
            // Java: if (record == next)
            {
                // Java: break;
            }
        }
        // Java: ++i;
        // Java: return i < size ? (MetaFileRecord)this.mRecords.get(i) : null;
        0
    }

    // Java: public void initMatrix(Matrix matrix)
    pub fn initMatrix(&self, _matrix: i32) {
        // Java: this.dc.getContext().initSrcTransform(matrix);
    }

    // Java: public void destroy()
    pub fn destroy(&mut self) {
        // Java: if (null != this.mRecords)
        {
            // Java: this.mRecords.clear();
            // Java: this.mRecords = null;
        }
        // Java: this.mHeader = null;
    }

    // Java: public MetaFileHeader getHeader()
    pub fn getHeader(&self) -> i32 {
        // Java: return this.mHeader;
        0
    }

    // Java: public List<MetaFileRecord> getRecords()
    pub fn getRecords(&self) -> i32 {
        // Java: return this.mRecords;
        0
    }

    // Java: public static final MetaFilePainter getMetaFilePainter(PictureInfo info, String path)
    pub fn getMetaFilePainter(_info: i32, _path: &str) -> i32 {
        // Java: InputStream fileStream = new FileInputStream(path);
        // Java: MetaFileParser stream = null;
        // Java: MetaFilePainter painter = null;
        // Java: try
        {
            // Java: if (info.type == 6)
            {
                // Java: stream = new WMFParser(fileStream, info.byteLen);
                // Java: WMFPictureInfo wmfPictureInfo = (WMFPictureInfo)info;
                // Java: if (wmfPictureInfo.getEmbedInfo() == null)
                {
                    // Java: String embedPath = stream.runEmbed();
                    // Java: wmfPictureInfo.setEmbedInfo(info);
                    // Java: if (embedPath != null)
                    {
                        // Java: PictureInfo infoEmbed = PictureParser.getPictureInfo(embedPath);
                        // Java: if (infoEmbed != null && infoEmbed.type == 5)
                        {
                            // Java: wmfPictureInfo.setEmbedInfo(infoEmbed);
                            // Java: fileStream.close();
                            // Java: fileStream = new FileInputStream(embedPath);
                            // Java: stream.close();
                            // Java: stream = new EMFParser(fileStream, infoEmbed.byteLen);
                            // Java: stream.run();
                        }
                    }
                }
                // Java: else
                {
                    // Java: stream.run();
                }
            }
            // Java: else
            {
                // Java: stream = new EMFParser(fileStream, info.byteLen);
                // Java: stream.run();
            }
            // Java: fileStream.close();
            // Java: painter = new MetaFilePainter(stream.getHeader(), stream.getRecords());
            // Java: return painter;
        }
        // Java: catch (Exception e)
        {
            // Java: e.printStackTrace();
        }
        // Java: finally
        {
            // Java: if (stream != null)
            {
                // Java: stream.close();
            }
        }
        // Java: return null;
        0
    }
}
