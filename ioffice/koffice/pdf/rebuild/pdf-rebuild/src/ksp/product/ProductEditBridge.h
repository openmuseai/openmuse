#pragma once

#include <stdint.h>
#include "ProductBridge.h"

#ifdef __cplusplus
extern "C" {
#endif

// Platform-neutral edit ABI. A session owns its document, loaded page and undo
// history. Pointers returned by open must be closed exactly once. Inputs are
// borrowed for the duration of each call; page indexes are zero-based.
typedef struct KspProductEditSession KspProductEditSession;

uint32_t ksp_product_edit_abi_version(void);
KspProductEditSession* ksp_product_edit_open(const char* path, int* error);
KspProductEditSession* ksp_product_edit_open_bytes(const uint8_t* bytes,
                                                   uint64_t size, int* error);
void ksp_product_edit_close(KspProductEditSession* session);
int ksp_product_edit_page_count(KspProductEditSession* session);
int ksp_product_edit_load_page(KspProductEditSession* session, int page_index);
int ksp_product_edit_domain_count(KspProductEditSession* session);
int ksp_product_edit_domain_rect(KspProductEditSession* session, int index,
                                  float* left, float* bottom, float* right,
                                  float* top);
int ksp_product_edit_domain_text(KspProductEditSession* session, int index,
                                  uint16_t* output, int capacity);
int ksp_product_edit_focus_at(KspProductEditSession* session, float x, float y);
int ksp_product_edit_focused_text(KspProductEditSession* session,
                                   uint16_t* output, int capacity);
int ksp_product_edit_selection(KspProductEditSession* session, int* start,
                                int* end);
int ksp_product_edit_set_selection(KspProductEditSession* session, int start,
                                    int end);
int ksp_product_edit_caret(KspProductEditSession* session, float* bottom_x,
                            float* bottom_y, float* top_x, float* top_y);
int ksp_product_edit_replace_selection(KspProductEditSession* session,
                                        const uint16_t* text, int length);
int ksp_product_edit_render_page(KspProductEditSession* session, int width,
                                  int height, uint8_t* rgba, int capacity);
// ABI v3. Both calls use the same document as editing. render_page_at sees
// pending edits on the active page without saving/reopening the document.
// Raster output is RGBA, four bytes per pixel, and caller-owned.
int ksp_product_edit_page_size(KspProductEditSession* session, int page_index,
                                float* width, float* height);
int ksp_product_edit_render_page_at(KspProductEditSession* session,
                                     int page_index, int width, int height,
                                     uint8_t* rgba, int capacity);
// ABI v4. Renders the rectangle [x, y, width, height] from a virtual full-page
// raster [full_width, full_height]. Coordinates start at the top-left in pixels.
// The output is tightly packed RGBA and is owned by the caller. The rectangle
// must fit inside the virtual page; at most 16 million output pixels are allowed.
int ksp_product_edit_render_page_region(KspProductEditSession* session,
                                         int page_index, int x, int y,
                                         int width, int height, int full_width,
                                         int full_height, uint8_t* rgba,
                                         int capacity);
// ABI v5. Character boxes use PDF page coordinates: origin at bottom-left,
// x increasing right and y increasing up. Each entry is one UTF-16 code unit;
// supplementary Unicode characters have two entries with the same box.
typedef struct KspProductTextChar {
    uint32_t utf16;
    float left;
    float bottom;
    float right;
    float top;
} KspProductTextChar;
typedef struct KspProductPageText {
    KspProductTextChar* chars;
    int32_t count;
} KspProductPageText;
// Pass an empty output; free a previous result before reusing it. This call
// initializes output even on failure. Release successful output with
// ksp_product_edit_free_page_text; no PDFium handle escapes this call.
int ksp_product_edit_read_page_text(KspProductEditSession* session,
                                     int page_index,
                                     KspProductPageText* output);
void ksp_product_edit_free_page_text(KspProductPageText* output);
// ABI v6. Link rectangles use PDF page coordinates. kind: 0 unsupported,
// 1 URI, 2 internal page destination. URI slices refer to uri_data and are
// UTF-8 bytes without a trailing NUL. page_index is zero-based for kind 2.
typedef struct KspProductPageLink {
    float left;
    float bottom;
    float right;
    float top;
    int32_t kind;
    int32_t page_index;
    uint32_t uri_offset;
    uint32_t uri_length;
} KspProductPageLink;
typedef struct KspProductPageLinks {
    KspProductPageLink* links;
    int32_t count;
    uint8_t* uri_data;
    uint32_t uri_size;
} KspProductPageLinks;
// Pass an empty output. The result owns both allocations and must be freed
// before reusing it. Unsupported PDF actions remain present as kind 0.
int ksp_product_edit_read_page_links(KspProductEditSession* session,
                                      int page_index,
                                      KspProductPageLinks* output);
void ksp_product_edit_free_page_links(KspProductPageLinks* output);
int ksp_product_edit_replace_domain(KspProductEditSession* session, int index,
                                     const uint16_t* text, int length);
int ksp_product_edit_undo(KspProductEditSession* session);
int ksp_product_edit_redo(KspProductEditSession* session);
int ksp_product_edit_can_undo(KspProductEditSession* session);
int ksp_product_edit_can_redo(KspProductEditSession* session);
int ksp_product_edit_save(KspProductEditSession* session, const char* path);
// ABI v2. Exports the current edited document directly to memory. On failure
// output is empty. Release successful output with ksp_product_free_bytes.
int ksp_product_edit_export_bytes(KspProductEditSession* session,
                                   KspProductBytes* output);

#ifdef __cplusplus
}
#endif
