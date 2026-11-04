#include "dsh_webview.h"

#include <json-glib/json-glib.h>
#include <webkit2/webkit2.h>

#include <cmath>
#include <cstring>
#include <string>

namespace {
// Only the authenticated, ephemeral loopback origin can use the Host bridge.
bool valid_loopback(GUri* uri) {
  return uri && g_strcmp0(g_uri_get_scheme(uri), "http") == 0 &&
         g_strcmp0(g_uri_get_host(uri), "127.0.0.1") == 0 &&
         g_uri_get_port(uri) > 0 && !g_uri_get_userinfo(uri);
}

bool same_origin(const std::string& launch, const char* candidate) {
  if (!candidate) return false;
  g_autoptr(GUri) left = g_uri_parse(launch.c_str(), G_URI_FLAGS_NONE, nullptr);
  g_autoptr(GUri) right = g_uri_parse(candidate, G_URI_FLAGS_NONE, nullptr);
  return valid_loopback(left) && valid_loopback(right) &&
         g_uri_get_port(left) == g_uri_get_port(right);
}

std::string json_string(const std::string& text) {
  g_autoptr(JsonNode) node = json_node_new(JSON_NODE_VALUE);
  json_node_set_string(node, text.c_str());
  g_autoptr(JsonGenerator) generator = json_generator_new();
  json_generator_set_root(generator, node);
  g_autofree char* encoded = json_generator_to_data(generator, nullptr);
  return encoded;
}

class DshWebView {
 public:
  DshWebView(FlView* view, GtkOverlay* overlay)
      : flutter_view_(GTK_WIDGET(view)), overlay_(overlay) {
    g_autoptr(FlStandardMethodCodec) codec = fl_standard_method_codec_new();
    channel_ = fl_method_channel_new(
        fl_engine_get_binary_messenger(fl_view_get_engine(view)),
        "com.openmuse.dsh/webview", FL_METHOD_CODEC(codec));
    fl_method_channel_set_method_call_handler(channel_, on_method, this, nullptr);
  }

  ~DshWebView() {
    fl_method_channel_set_method_call_handler(channel_, nullptr, nullptr, nullptr);
    g_clear_object(&channel_);
  }

 private:
  static const char* string_arg(FlValue* args, const char* name) {
    if (!args || fl_value_get_type(args) != FL_VALUE_TYPE_MAP) return nullptr;
    FlValue* value = fl_value_lookup_string(args, name);
    return value && fl_value_get_type(value) == FL_VALUE_TYPE_STRING
               ? fl_value_get_string(value) : nullptr;
  }

  static double number_arg(FlValue* args, const char* name) {
    if (!args || fl_value_get_type(args) != FL_VALUE_TYPE_MAP) return -1;
    FlValue* value = fl_value_lookup_string(args, name);
    if (!value) return -1;
    if (fl_value_get_type(value) == FL_VALUE_TYPE_FLOAT) return fl_value_get_float(value);
    if (fl_value_get_type(value) == FL_VALUE_TYPE_INT) return fl_value_get_int(value);
    return -1;
  }

  static void on_method(FlMethodChannel*, FlMethodCall* call, gpointer data) {
    auto* self = static_cast<DshWebView*>(data);
    const char* method = fl_method_call_get_name(call);
    FlValue* args = fl_method_call_get_args(call);
    if (g_str_equal(method, "show")) {
      const char* url = string_arg(args, "url");
      const double x = number_arg(args, "x"), y = number_arg(args, "y");
      const double width = number_arg(args, "width"), height = number_arg(args, "height");
      g_autoptr(GUri) uri = url ? g_uri_parse(url, G_URI_FLAGS_NONE, nullptr) : nullptr;
      if (!valid_loopback(uri) || !std::isfinite(x) || !std::isfinite(y) ||
          !std::isfinite(width) || !std::isfinite(height) ||
          x < 0 || y < 0 || width < 1 || height < 1 ||
          x > 32768 || y > 32768 || width > 32768 || height > 32768) {
        fl_method_call_respond_error(call, "invalid-arguments", "Invalid DSH origin or bounds", nullptr, nullptr);
        return;
      }
      if(const char* script=string_arg(args,"resourceReferenceScript")) {
        if(strlen(script)<65536)self->resource_reference_script_=script;
      }
      if (self->url_ != url) self->create(url);
      if (const char* mount = string_arg(args, "mount")) self->workspace(mount);
      if (const char* session = string_arg(args, "session")) {
        if (self->session_ != session) {
          self->session_ = session;
          self->push_state();
        }
      }
      gtk_widget_set_margin_start(GTK_WIDGET(self->webview_), static_cast<int>(x));
      gtk_widget_set_margin_top(GTK_WIDGET(self->webview_), static_cast<int>(y));
      gtk_widget_set_size_request(GTK_WIDGET(self->webview_), static_cast<int>(width), static_cast<int>(height));
      gtk_widget_show(GTK_WIDGET(self->webview_));
    } else if (g_str_equal(method, "hide")) {
      if (self->webview_) gtk_widget_hide(GTK_WIDGET(self->webview_));
      // Logout and Flutter dialogs can hide a focused WebKit overlay. GTK
      // otherwise keeps keyboard focus on that hidden child, leaving visible
      // Flutter text fields unable to receive input until the app restarts.
      gtk_widget_grab_focus(self->flutter_view_);
    } else if (g_str_equal(method, "reload")) {
      if (self->webview_) webkit_web_view_reload(self->webview_);
    } else if (g_str_equal(method, "activateWorkspace") || g_str_equal(method, "openSession")) {
      if (fl_value_get_type(args) != FL_VALUE_TYPE_STRING) {
        fl_method_call_respond_error(call, "invalid-arguments", "Expected a string", nullptr, nullptr);
        return;
      }
      const char* value = fl_value_get_string(args);
      if (g_str_equal(method, "activateWorkspace")) self->workspace(value);
      else self->evaluate("window.__OpenMuseDesiredSession=" + json_string(value) +
          ";window.OpenMuseDshWorkspace?.openSession(window.__OpenMuseDesiredSession);");
    } else {
      fl_method_call_respond_not_implemented(call, nullptr);
      return;
    }
    fl_method_call_respond_success(call, nullptr, nullptr);
  }

  void evaluate(const std::string& script) {
    if (webview_) webkit_web_view_evaluate_javascript(
        webview_, script.c_str(), -1, nullptr, nullptr, nullptr, nullptr, nullptr);
  }

  void workspace(const std::string& path) {
    if (path == mount_) return;
    mount_ = path;
    push_state();
  }

  void push_state() {
    evaluate("window.__OpenMuseDesiredWorkspace=" + json_string(mount_) +
        ";window.OpenMuseDshWorkspace?.activate(window.__OpenMuseDesiredWorkspace);");
    if (!session_.empty()) evaluate("window.__OpenMuseDesiredSession=" + json_string(session_) +
        ";window.OpenMuseDshWorkspace?.openSession(window.__OpenMuseDesiredSession);");
  }

  void failed(const char* message) {
    g_autoptr(FlValue) value = fl_value_new_string(message);
    fl_method_channel_invoke_method(channel_, "failed", value, nullptr, nullptr, nullptr);
  }

  static gboolean decide_policy(WebKitWebView*, WebKitPolicyDecision* decision,
                               WebKitPolicyDecisionType type, gpointer data) {
    auto* self = static_cast<DshWebView*>(data);
    if (type == WEBKIT_POLICY_DECISION_TYPE_NAVIGATION_ACTION ||
        type == WEBKIT_POLICY_DECISION_TYPE_NEW_WINDOW_ACTION) {
      auto* navigation = WEBKIT_NAVIGATION_POLICY_DECISION(decision);
      auto* action = webkit_navigation_policy_decision_get_navigation_action(navigation);
      const char* uri = webkit_uri_request_get_uri(webkit_navigation_action_get_request(action));
      if (type == WEBKIT_POLICY_DECISION_TYPE_NEW_WINDOW_ACTION || !same_origin(self->url_, uri)) {
        webkit_policy_decision_ignore(decision);
        return TRUE;
      }
    }
    if (type == WEBKIT_POLICY_DECISION_TYPE_RESPONSE) {
      auto* response = WEBKIT_RESPONSE_POLICY_DECISION(decision);
      if (webkit_response_policy_decision_is_main_frame_main_resource(response) &&
          webkit_uri_response_get_status_code(webkit_response_policy_decision_get_response(response)) >= 400) {
        self->failed("DSH page returned an HTTP error");
        webkit_policy_decision_ignore(decision);
        return TRUE;
      }
    }
    return FALSE;
  }

  static void message(WebKitUserContentManager*, WebKitJavascriptResult* result, gpointer data) {
    auto* self = static_cast<DshWebView*>(data);
    if (!same_origin(self->url_, webkit_web_view_get_uri(self->webview_))) return;
    JSCValue* value = webkit_javascript_result_get_js_value(result);
    if (!jsc_value_is_string(value)) return;
    g_autofree char* raw = jsc_value_to_string(value);
    if (!raw || strlen(raw) > 16384) return;
    g_autoptr(JsonParser) parser = json_parser_new();
    if (!json_parser_load_from_data(parser, raw, -1, nullptr)) return;
    JsonNode* root = json_parser_get_root(parser);
    if (!JSON_NODE_HOLDS_OBJECT(root)) return;
    JsonObject* object = json_node_get_object(root);
    JsonNode* type = json_object_get_member(object, "type");
    if (!type || !JSON_NODE_HOLDS_VALUE(type) || json_node_get_value_type(type) != G_TYPE_STRING) return;
    const char* kind = json_node_get_string(type);
    if (g_str_equal(kind, "clipboard.write")) {
      JsonNode* text = json_object_get_member(object, "text");
      if (text && JSON_NODE_HOLDS_VALUE(text) && json_node_get_value_type(text) == G_TYPE_STRING)
        gtk_clipboard_set_text(gtk_clipboard_get(GDK_SELECTION_CLIPBOARD), json_node_get_string(text), -1);
      return;
    }
    const char* method = (g_str_equal(kind, "resource.open") || g_str_equal(kind,"resource.reference.open")) ? "resourceOpen" :
                         g_str_equal(kind, "workspace.activate") ? "workspaceActivate" : nullptr;
    if (!method) return;
    g_autoptr(FlValue) payload = fl_value_new_string(raw);
    fl_method_channel_invoke_method(self->channel_, method, payload, nullptr, nullptr, nullptr);
  }

  void create(const std::string& url) {
    if (webview_) gtk_widget_destroy(GTK_WIDGET(webview_));
    url_ = url;
    g_autoptr(WebKitWebContext) context = webkit_web_context_new_ephemeral();
    g_autoptr(WebKitUserContentManager) manager = webkit_user_content_manager_new();
    g_signal_connect(manager, "script-message-received::openmuse", G_CALLBACK(message), this);
    webkit_user_content_manager_register_script_message_handler(manager, "openmuse");
    const std::string script = R"JS(
      for (const name of ['MuseHostResource','MuseHostWorkspace','MuseHostClipboard']) {
        window[name] = {postMessage: raw => window.webkit.messageHandlers.openmuse.postMessage(String(raw))};
      }
    )JS" + std::string("window.__OpenMuseDesiredWorkspace=") + json_string(mount_) + ";";
    WebKitUserScript* injected = webkit_user_script_new(script.c_str(),
        WEBKIT_USER_CONTENT_INJECT_TOP_FRAME, WEBKIT_USER_SCRIPT_INJECT_AT_DOCUMENT_START, nullptr, nullptr);
    webkit_user_content_manager_add_script(manager, injected);
    webkit_user_script_unref(injected);
    if(!resource_reference_script_.empty()) {
      WebKitUserScript* resource_script=webkit_user_script_new(resource_reference_script_.c_str(),
        WEBKIT_USER_CONTENT_INJECT_TOP_FRAME,WEBKIT_USER_SCRIPT_INJECT_AT_DOCUMENT_END,nullptr,nullptr);
      webkit_user_content_manager_add_script(manager,resource_script);webkit_user_script_unref(resource_script);
    }
    webview_ = WEBKIT_WEB_VIEW(g_object_new(WEBKIT_TYPE_WEB_VIEW,
        "web-context", context, "user-content-manager", manager, nullptr));
    gtk_widget_set_halign(GTK_WIDGET(webview_), GTK_ALIGN_START);
    gtk_widget_set_valign(GTK_WIDGET(webview_), GTK_ALIGN_START);
    gtk_overlay_add_overlay(overlay_, GTK_WIDGET(webview_));
    g_signal_connect(webview_, "decide-policy", G_CALLBACK(decide_policy), this);
    g_signal_connect(webview_, "load-changed", G_CALLBACK(+[](WebKitWebView*, WebKitLoadEvent event, gpointer data) {
      if (event == WEBKIT_LOAD_FINISHED) {
        static_cast<DshWebView*>(data)->push_state();
        g_message("OpenMuse DSH WebKit page loaded");
      }
    }), this);
    g_signal_connect(webview_, "load-failed", G_CALLBACK(+[](WebKitWebView*, WebKitLoadEvent, const char*, GError* error, gpointer data) -> gboolean {
      if (!g_error_matches(error, WEBKIT_NETWORK_ERROR, WEBKIT_NETWORK_ERROR_CANCELLED))
        static_cast<DshWebView*>(data)->failed("Unable to load DSH page");
      return TRUE;
    }), this);
    g_signal_connect(webview_, "web-process-terminated", G_CALLBACK(+[](WebKitWebView*, WebKitWebProcessTerminationReason, gpointer data) {
      static_cast<DshWebView*>(data)->failed("DSH web process exited; reload the panel");
    }), this);
    // WebKit handles the same-origin token exchange and HttpOnly cookie redirect.
    webkit_web_view_load_uri(webview_, url.c_str());
  }

  GtkWidget* flutter_view_;  // Borrowed; owned by the same overlay as this adapter.
  GtkOverlay* overlay_;
  FlMethodChannel* channel_ = nullptr;
  WebKitWebView* webview_ = nullptr;
  std::string url_;
  std::string resource_reference_script_;
  std::string mount_;
  std::string session_;
};
}  // namespace

void openmuse_dsh_webview_register(FlView* view, GtkOverlay* overlay) {
  g_object_set_data_full(G_OBJECT(overlay), "openmuse-dsh",
      new DshWebView(view, overlay), [](gpointer data) { delete static_cast<DshWebView*>(data); });
}
