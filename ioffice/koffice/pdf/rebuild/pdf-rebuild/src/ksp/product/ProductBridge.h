#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ABI v2. Input pointers are valid only for the duration of this synchronous call.
// Page indexes are zero-based. Page order and duplicates are preserved.
typedef struct KspProductPageSelection {
    const uint8_t* pdf;
    uint64_t pdf_size;
    const int32_t* page_indices;
    uint32_t page_count;
} KspProductPageSelection;

typedef struct KspProductBytes {
    uint8_t* data;
    uint64_t size;
} KspProductBytes;

uint32_t ksp_product_abi_version(void);

// Returns the page count or a negative error. Uses the same PDFium parser as
// compose_pages and does not retain the source buffer.
int32_t ksp_product_page_count(const uint8_t* pdf, uint64_t pdf_size);

// Returns 0 on success, negative on failure. On failure output is empty and
// error is a NUL-terminated explanation when error_capacity > 0. Call
// ksp_product_free_bytes() before reusing an output that already owns bytes.
int32_t ksp_product_compose_pages(const KspProductPageSelection* sources,
                                  uint32_t source_count,
                                  KspProductBytes* output,
                                  char* error,
                                  uint32_t error_capacity);

void ksp_product_free_bytes(KspProductBytes* bytes);

#ifdef __cplusplus
}
#endif
