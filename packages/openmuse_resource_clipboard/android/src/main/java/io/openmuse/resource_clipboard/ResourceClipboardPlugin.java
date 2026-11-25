package io.openmuse.resource_clipboard;

import android.content.ClipData;
import android.content.ClipDescription;
import android.content.ClipboardManager;
import android.content.Context;
import android.os.PersistableBundle;
import io.flutter.embedding.engine.plugins.FlutterPlugin;
import io.flutter.plugin.common.MethodCall;
import io.flutter.plugin.common.MethodChannel;

public final class ResourceClipboardPlugin implements FlutterPlugin, MethodChannel.MethodCallHandler {
  private MethodChannel channel;
  private ClipboardManager clipboard;
  private static final String MIME="application/vnd.openmuse.resource-reference+json";
  @Override public void onAttachedToEngine(FlutterPluginBinding binding) {
    clipboard=(ClipboardManager)binding.getApplicationContext().getSystemService(Context.CLIPBOARD_SERVICE);
    channel=new MethodChannel(binding.getBinaryMessenger(),"io.openmuse/resource_clipboard");channel.setMethodCallHandler(this);
  }
  @Override public void onDetachedFromEngine(FlutterPluginBinding binding) {channel.setMethodCallHandler(null);clipboard=null;}
  @Override public void onMethodCall(MethodCall call,MethodChannel.Result result) {
    if(clipboard==null){result.error("UNAVAILABLE","Clipboard unavailable",null);return;}
    if(call.method.equals("write")) {
      String text=call.argument("text"),reference=call.argument("reference");
      if(text==null || reference==null || reference.length()>8192){result.error("INVALID_REFERENCE","Invalid reference",null);return;}
      ClipDescription description=new ClipDescription("OpenMuse resource",new String[]{"text/plain",MIME});
      PersistableBundle extras=new PersistableBundle();extras.putString(MIME,reference);description.setExtras(extras);
      clipboard.setPrimaryClip(new ClipData(description,new ClipData.Item(text)));result.success(null);
    } else if(call.method.equals("read")) {
      ClipData clip=clipboard.getPrimaryClip();
      PersistableBundle extras=clip==null?null:clip.getDescription().getExtras();
      result.success(extras==null?null:extras.getString(MIME));
    } else result.notImplemented();
  }
}
