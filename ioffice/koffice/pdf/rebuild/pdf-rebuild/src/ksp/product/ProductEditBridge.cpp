#include "ProductEditBridge.h"

#include "ProductSession.h"
#include "../runtime/PdfiumLifecycle.h"

#include <mutex>

namespace {
// Edit sessions and page tools share one PDFium instance.
#define KSP_PRODUCT_LOCK std::lock_guard<std::mutex> productCallGuard(ksp::runtime::productPdfiumMutex())
KspMacEditSession* sessionImpl(KspProductEditSession* session) {
    return reinterpret_cast<KspMacEditSession*>(session);
}
KspProductEditSession* productHandle(KspMacEditSession* session) {
    return reinterpret_cast<KspProductEditSession*>(session);
}
}  // namespace

extern "C" {

uint32_t ksp_product_edit_abi_version(void) { return 6; }
KspProductEditSession* ksp_product_edit_open(const char* path, int* error) {
    KSP_PRODUCT_LOCK;
    return productHandle(ksp::product::openSession(path, error));
}
KspProductEditSession* ksp_product_edit_open_bytes(const uint8_t* bytes,
                                                   uint64_t size, int* error) {
    KSP_PRODUCT_LOCK;
    return productHandle(ksp::product::openSessionBytes(bytes, size, error));
}
void ksp_product_edit_close(KspProductEditSession* session) {
    KSP_PRODUCT_LOCK;
    ksp::product::closeSession(sessionImpl(session));
}
int ksp_product_edit_page_count(KspProductEditSession* session) {
    KSP_PRODUCT_LOCK;
    return ksp::product::pageCount(sessionImpl(session));
}
int ksp_product_edit_load_page(KspProductEditSession* session, int index) {
    KSP_PRODUCT_LOCK;
    return ksp::product::loadEditPage(sessionImpl(session), index);
}
int ksp_product_edit_domain_count(KspProductEditSession* session) {
    KSP_PRODUCT_LOCK;
    return ksp::product::domainCount(sessionImpl(session));
}
int ksp_product_edit_domain_rect(KspProductEditSession* session, int index,
                                  float* left, float* bottom, float* right,
                                  float* top) {
    KSP_PRODUCT_LOCK;
    return ksp::product::domainRect(sessionImpl(session), index, left, bottom,
                                     right, top);
}
int ksp_product_edit_domain_text(KspProductEditSession* session, int index,
                                  uint16_t* output, int capacity) {
    KSP_PRODUCT_LOCK;
    return ksp::product::domainText(sessionImpl(session), index, output, capacity);
}
int ksp_product_edit_focus_at(KspProductEditSession* session, float x, float y) {
    KSP_PRODUCT_LOCK;
    return ksp::product::focusAt(sessionImpl(session), x, y);
}
int ksp_product_edit_focused_text(KspProductEditSession* session,
                                   uint16_t* output, int capacity) {
    KSP_PRODUCT_LOCK;
    return ksp::product::focusedText(sessionImpl(session), output, capacity);
}
int ksp_product_edit_selection(KspProductEditSession* session, int* start,
                                int* end) {
    KSP_PRODUCT_LOCK;
    return ksp::product::selection(sessionImpl(session), start, end);
}
int ksp_product_edit_set_selection(KspProductEditSession* session, int start,
                                    int end) {
    KSP_PRODUCT_LOCK;
    return ksp::product::setSelection(sessionImpl(session), start, end);
}
int ksp_product_edit_caret(KspProductEditSession* session, float* bottom_x,
                            float* bottom_y, float* top_x, float* top_y) {
    KSP_PRODUCT_LOCK;
    return ksp::product::caret(sessionImpl(session), bottom_x, bottom_y, top_x, top_y);
}
int ksp_product_edit_replace_selection(KspProductEditSession* session,
                                        const uint16_t* text, int length) {
    KSP_PRODUCT_LOCK;
    return ksp::product::replaceSelection(sessionImpl(session), text, length);
}
int ksp_product_edit_render_page(KspProductEditSession* session, int width,
                                  int height, uint8_t* rgba, int capacity) {
    KSP_PRODUCT_LOCK;
    return ksp::product::renderLoadedPage(sessionImpl(session), width, height, rgba,
                                     capacity);
}
int ksp_product_edit_page_size(KspProductEditSession* session, int pageIndex,
                                float* width, float* height) {
    KSP_PRODUCT_LOCK;
    return ksp::product::pageSize(sessionImpl(session), pageIndex, width, height);
}
int ksp_product_edit_render_page_at(KspProductEditSession* session,
                                     int pageIndex, int width, int height,
                                     uint8_t* rgba, int capacity) {
    KSP_PRODUCT_LOCK;
    return ksp::product::renderPageAt(sessionImpl(session), pageIndex, width,
                                        height, rgba, capacity);
}
int ksp_product_edit_render_page_region(KspProductEditSession* session,
                                         int pageIndex, int x, int y,
                                         int width, int height, int fullWidth,
                                         int fullHeight, uint8_t* rgba,
                                         int capacity) {
    KSP_PRODUCT_LOCK;
    return ksp::product::renderPageRegion(sessionImpl(session), pageIndex, x,
                                           y, width, height, fullWidth,
                                           fullHeight, rgba, capacity);
}
int ksp_product_edit_read_page_text(KspProductEditSession* session,
                                     int pageIndex,
                                     KspProductPageText* output) {
    KSP_PRODUCT_LOCK;
    return ksp::product::readPageText(sessionImpl(session), pageIndex, output);
}
void ksp_product_edit_free_page_text(KspProductPageText* output) {
    ksp::product::freePageText(output);
}
int ksp_product_edit_read_page_links(KspProductEditSession* session,
                                     int pageIndex,
                                     KspProductPageLinks* output) {
    KSP_PRODUCT_LOCK;
    return ksp::product::readPageLinks(sessionImpl(session), pageIndex, output);
}
void ksp_product_edit_free_page_links(KspProductPageLinks* output) {
    ksp::product::freePageLinks(output);
}
int ksp_product_edit_replace_domain(KspProductEditSession* session, int index,
                                     const uint16_t* text, int length) {
    KSP_PRODUCT_LOCK;
    return ksp::product::replaceDomain(sessionImpl(session), index, text, length);
}
int ksp_product_edit_undo(KspProductEditSession* session) {
    KSP_PRODUCT_LOCK;
    return ksp::product::undo(sessionImpl(session));
}
int ksp_product_edit_redo(KspProductEditSession* session) {
    KSP_PRODUCT_LOCK;
    return ksp::product::redo(sessionImpl(session));
}
int ksp_product_edit_can_undo(KspProductEditSession* session) {
    KSP_PRODUCT_LOCK;
    return ksp::product::canUndo(sessionImpl(session));
}
int ksp_product_edit_can_redo(KspProductEditSession* session) {
    KSP_PRODUCT_LOCK;
    return ksp::product::canRedo(sessionImpl(session));
}
int ksp_product_edit_save(KspProductEditSession* session, const char* path) {
    KSP_PRODUCT_LOCK;
    return ksp::product::saveSession(sessionImpl(session), path);
}
int ksp_product_edit_export_bytes(KspProductEditSession* session,
                                   KspProductBytes* output) {
    KSP_PRODUCT_LOCK;
    if (!output) return -1001;
    output->data = nullptr;
    output->size = 0;
    return ksp::product::exportSessionBytes(sessionImpl(session), &output->data,
                                     &output->size);
}

}  // extern "C"
