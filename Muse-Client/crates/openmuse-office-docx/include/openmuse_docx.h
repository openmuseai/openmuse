#ifndef OPENMUSE_DOCX_H
#define OPENMUSE_DOCX_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define OPENMUSE_DOCX_ABI_VERSION 1u

typedef struct OpenMuseDocxBuffer {
  uint8_t *ptr;
  size_t len;
  size_t capacity;
  int32_t status;
} OpenMuseDocxBuffer;

uint32_t openmuse_docx_abi_version(void);
OpenMuseDocxBuffer openmuse_docx_inspect(const uint8_t *docx, size_t docx_len);
OpenMuseDocxBuffer openmuse_docx_export_simple(
    const uint8_t *docx,
    size_t docx_len,
    const uint8_t *paragraphs_json,
    size_t paragraphs_json_len);
void openmuse_docx_buffer_free(OpenMuseDocxBuffer buffer);

#ifdef __cplusplus
}
#endif

#endif
