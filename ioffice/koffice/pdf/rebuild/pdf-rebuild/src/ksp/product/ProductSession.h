#pragma once

#include "../runtime/HandleRegistry.h"

#include <cstddef>
#include <cstdint>
#include <mutex>
#include <vector>

// The historical C ABI uses this opaque name. The session itself belongs to
// the product layer and is shared by the macOS and Android bridges.
struct KspMacEditSession {
    ksp::runtime::Handle document = 0;
    ksp::runtime::Handle page = 0;
    int pageIndex = -1;
    std::vector<ksp::runtime::Handle> history;
    std::size_t historyPosition = 0;
    std::mutex mutex;
};

struct KspProductPageText;
struct KspProductPageLinks;

namespace ksp::product {

KspMacEditSession* openSession(const char* path, int* error);
KspMacEditSession* openSessionBytes(const uint8_t* bytes, uint64_t size,
                                    int* error);
void closeSession(KspMacEditSession* session);
int pageCount(KspMacEditSession* session);
int loadEditPage(KspMacEditSession* session, int pageIndex);
int domainCount(KspMacEditSession* session);
int domainRect(KspMacEditSession* session, int index, float* left,
               float* bottom, float* right, float* top);
int domainText(KspMacEditSession* session, int index, uint16_t* output,
               int capacity);
int focusAt(KspMacEditSession* session, float x, float y);
int focusedText(KspMacEditSession* session, uint16_t* output, int capacity);
int selection(KspMacEditSession* session, int* start, int* end);
int setSelection(KspMacEditSession* session, int start, int end);
int caret(KspMacEditSession* session, float* bottomX, float* bottomY,
          float* topX, float* topY);
int replaceSelection(KspMacEditSession* session, const uint16_t* text,
                     int length);
int replaceDomain(KspMacEditSession* session, int index, const uint16_t* text,
                  int length);
int undo(KspMacEditSession* session);
int redo(KspMacEditSession* session);
int canUndo(KspMacEditSession* session);
int canRedo(KspMacEditSession* session);
int pageSize(KspMacEditSession* session, int pageIndex, float* width,
             float* height);
int renderLoadedPage(KspMacEditSession* session, int width, int height,
                     uint8_t* rgba, int capacity);
int renderPageAt(KspMacEditSession* session, int pageIndex, int width,
                 int height, uint8_t* rgba, int capacity);
int renderPageRegion(KspMacEditSession* session, int pageIndex, int x, int y,
                     int width, int height, int fullWidth, int fullHeight,
                     uint8_t* rgba, int capacity);
int readPageText(KspMacEditSession* session, int pageIndex,
                 KspProductPageText* output);
void freePageText(KspProductPageText* output);
int readPageLinks(KspMacEditSession* session, int pageIndex,
                  KspProductPageLinks* output);
void freePageLinks(KspProductPageLinks* output);
int saveSession(KspMacEditSession* session, const char* path);
int exportSessionBytes(KspMacEditSession* session, uint8_t** data,
                       uint64_t* size);

}  // namespace ksp::product
