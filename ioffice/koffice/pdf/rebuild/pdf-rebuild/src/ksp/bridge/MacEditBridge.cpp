#include "MacEditBridge.h"

#include "../product/ProductSession.h"

extern "C" {

KspMacEditSession* ksp_mac_edit_open(const char* path, int* error) {
    return ksp::product::openSession(path, error);
}

KspMacEditSession* ksp_mac_edit_open_bytes(const uint8_t* bytes,
                                         uint64_t size, int* error) {
    return ksp::product::openSessionBytes(bytes, size, error);
}

void ksp_mac_edit_close(KspMacEditSession* session) {
    ksp::product::closeSession(session);
}

int ksp_mac_edit_page_count(KspMacEditSession* session) {
    return ksp::product::pageCount(session);
}

int ksp_mac_edit_load_page(KspMacEditSession* session, int pageIndex) {
    return ksp::product::loadEditPage(session, pageIndex);
}

int ksp_mac_edit_domain_count(KspMacEditSession* session) {
    return ksp::product::domainCount(session);
}

int ksp_mac_edit_domain_rect(KspMacEditSession* session, int index,
                             float* left, float* bottom, float* right, float* top) {
    return ksp::product::domainRect(session, index, left, bottom, right, top);
}

int ksp_mac_edit_domain_text(KspMacEditSession* session, int index,
                             uint16_t* output, int capacity) {
    return ksp::product::domainText(session, index, output, capacity);
}

int ksp_mac_edit_focus_at(KspMacEditSession* session, float x, float y) {
    return ksp::product::focusAt(session, x, y);
}

int ksp_mac_edit_focused_text(KspMacEditSession* session,
                              uint16_t* output, int capacity) {
    return ksp::product::focusedText(session, output, capacity);
}

int ksp_mac_edit_selection(KspMacEditSession* session, int* start, int* end) {
    return ksp::product::selection(session, start, end);
}

int ksp_mac_edit_set_selection(KspMacEditSession* session, int start, int end) {
    return ksp::product::setSelection(session, start, end);
}

int ksp_mac_edit_caret(KspMacEditSession* session, float* bottomX,
                       float* bottomY, float* topX, float* topY) {
    return ksp::product::caret(session, bottomX, bottomY, topX, topY);
}

int ksp_mac_edit_replace_selection(KspMacEditSession* session,
                                   const uint16_t* text, int length) {
    return ksp::product::replaceSelection(session, text, length);
}

int ksp_mac_edit_render_page(KspMacEditSession* session, int width, int height,
                             uint8_t* rgba, int capacity) {
    return ksp::product::renderLoadedPage(session, width, height, rgba, capacity);
}

int ksp_mac_edit_page_size(KspMacEditSession* session, int pageIndex,
                           float* width, float* height) {
    return ksp::product::pageSize(session, pageIndex, width, height);
}

int ksp_mac_edit_render_page_at(KspMacEditSession* session, int pageIndex,
                                int width, int height, uint8_t* rgba,
                                int capacity) {
    return ksp::product::renderPageAt(session, pageIndex, width, height, rgba,
                                       capacity);
}

int ksp_mac_edit_replace_domain(KspMacEditSession* session, int index,
                                const uint16_t* text, int length) {
    return ksp::product::replaceDomain(session, index, text, length);
}

int ksp_mac_edit_undo(KspMacEditSession* session) {
    return ksp::product::undo(session);
}

int ksp_mac_edit_redo(KspMacEditSession* session) {
    return ksp::product::redo(session);
}

int ksp_mac_edit_can_undo(KspMacEditSession* session) {
    return ksp::product::canUndo(session);
}

int ksp_mac_edit_can_redo(KspMacEditSession* session) {
    return ksp::product::canRedo(session);
}

int ksp_mac_edit_save(KspMacEditSession* session, const char* path) {
    return ksp::product::saveSession(session, path);
}

int ksp_mac_edit_export_bytes(KspMacEditSession* session, uint8_t** data,
                              uint64_t* size) {
    return ksp::product::exportSessionBytes(session, data, size);
}

}  // extern "C"
