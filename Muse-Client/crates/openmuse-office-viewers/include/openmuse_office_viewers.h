#ifndef OPENMUSE_OFFICE_VIEWERS_H
#define OPENMUSE_OFFICE_VIEWERS_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define OPENMUSE_OFFICE_VIEWERS_ABI_VERSION 1u

typedef struct OpenMuseOfficeViewerBuffer {
  uint8_t *ptr;
  size_t len;
  size_t capacity;
  int32_t status;
} OpenMuseOfficeViewerBuffer;

uint32_t openmuse_office_viewers_abi_version(void);
OpenMuseOfficeViewerBuffer openmuse_xlsx_inspect(
    const uint8_t *xlsx,
    size_t xlsx_len);
OpenMuseOfficeViewerBuffer openmuse_pptx_inspect(
    const uint8_t *pptx,
    size_t pptx_len);
OpenMuseOfficeViewerBuffer openmuse_pdf_inspect(
    const uint8_t *pdf,
    size_t pdf_len);
void openmuse_office_viewer_buffer_free(OpenMuseOfficeViewerBuffer buffer);

#ifdef __cplusplus
}
#endif

#endif
