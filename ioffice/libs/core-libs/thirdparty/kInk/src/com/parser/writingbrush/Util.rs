// Java: import android.content.Context
// Java: import android.content.res.AssetManager
// Java: import android.graphics.Bitmap
// Java: import android.graphics.BitmapFactory
// Java: import java.io.IOException
// Java: import java.io.InputStream

pub struct Context;
pub struct Bitmap;

// Java: public class Util
pub struct Util {
}

// Java: public class Util
impl Util {
    // Java: static Bitmap loadBitmpFromRes(Context context, String name)
    pub fn loadBitmpFromRes(_context: &Context, _name: &str) -> Option<Bitmap> {
      // Java: Bitmap bitmap = null;
      let bitmap: Option<Bitmap> = None;
      // Java: AssetManager am = context.getResources().getAssets();
      // Java: try
      {
         // Java: InputStream is = am.open(name);
         // Java: bitmap = BitmapFactory.decodeStream(is);
         // Java: is.close();
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
      }
      // Java: return bitmap;
      bitmap
    }

}
