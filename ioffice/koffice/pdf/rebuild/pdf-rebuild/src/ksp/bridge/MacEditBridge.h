#pragma once

#include <cstdint>

extern "C" {

// Historical symbol prefix retained for binary compatibility. This bridge is
// platform-neutral C++ and is built into both macOS and Android product libs.
struct KspMacEditSession;

KspMacEditSession* ksp_mac_edit_open(const char* path, int* error);
KspMacEditSession* ksp_mac_edit_open_bytes(const uint8_t* bytes,
                                         uint64_t size, int* error);
void ksp_mac_edit_close(KspMacEditSession* session);
int ksp_mac_edit_page_count(KspMacEditSession* session);
int ksp_mac_edit_load_page(KspMacEditSession* session, int page_index);
int ksp_mac_edit_domain_count(KspMacEditSession* session);
int ksp_mac_edit_domain_rect(KspMacEditSession* session, int index,
                             float* left, float* bottom, float* right, float* top);
int ksp_mac_edit_domain_text(KspMacEditSession* session, int index,
                             uint16_t* output, int capacity);
int ksp_mac_edit_focus_at(KspMacEditSession* session, float x, float y);
int ksp_mac_edit_focused_text(KspMacEditSession* session,
                              uint16_t* output, int capacity);
int ksp_mac_edit_selection(KspMacEditSession* session, int* start, int* end);
int ksp_mac_edit_set_selection(KspMacEditSession* session, int start, int end);
int ksp_mac_edit_caret(KspMacEditSession* session, float* bottom_x,
                       float* bottom_y, float* top_x, float* top_y);
int ksp_mac_edit_replace_selection(KspMacEditSession* session,
                                   const uint16_t* text, int length);
int ksp_mac_edit_render_page(KspMacEditSession* session, int width, int height,
                             uint8_t* rgba, int capacity);
int ksp_mac_edit_page_size(KspMacEditSession* session, int page_index,
                           float* width, float* height);
int ksp_mac_edit_render_page_at(KspMacEditSession* session, int page_index,
                                int width, int height, uint8_t* rgba,
                                int capacity);
int ksp_mac_edit_replace_domain(KspMacEditSession* session, int index,
                                const uint16_t* text, int length);
int ksp_mac_edit_undo(KspMacEditSession* session);
int ksp_mac_edit_redo(KspMacEditSession* session);
int ksp_mac_edit_can_undo(KspMacEditSession* session);
int ksp_mac_edit_can_redo(KspMacEditSession* session);
int ksp_mac_edit_save(KspMacEditSession* session, const char* path);
// Allocates a complete snapshot with malloc. The product bridge transfers its
// ownership to KspProductBytes; callers free it with ksp_product_free_bytes.
int ksp_mac_edit_export_bytes(KspMacEditSession* session, uint8_t** data,
                              uint64_t* size);

}
