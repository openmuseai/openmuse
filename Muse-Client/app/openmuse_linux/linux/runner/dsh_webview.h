#ifndef OPENMUSE_LINUX_DSH_WEBVIEW_H_
#define OPENMUSE_LINUX_DSH_WEBVIEW_H_

#include <flutter_linux/flutter_linux.h>
#include <gtk/gtk.h>

// The overlay owns the adapter for the lifetime of the Flutter window.
void openmuse_dsh_webview_register(FlView* view, GtkOverlay* overlay);

#endif
