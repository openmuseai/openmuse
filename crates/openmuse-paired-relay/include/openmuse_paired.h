#ifndef OPENMUSE_PAIRED_H
#define OPENMUSE_PAIRED_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define OPENMUSE_PAIRED_ABI_VERSION 1u
#define OPENMUSE_PAIRED_DEVICE_PUBLIC_BYTES 64u

typedef struct OpenMusePairedBuffer {
  uint8_t *ptr;
  size_t len;
  size_t capacity;
  int32_t status;
} OpenMusePairedBuffer;

uint32_t openmuse_paired_abi_version(void);
int32_t openmuse_paired_device_public(
    const uint8_t *seed,
    size_t seed_len,
    uint8_t *output,
    size_t output_len);
OpenMusePairedBuffer openmuse_paired_issue_offer(
    const uint8_t *seed,
    size_t seed_len,
    const uint8_t *account_ref,
    size_t account_ref_len,
    const uint8_t *device_ref,
    size_t device_ref_len,
    const uint8_t *nonce,
    size_t nonce_len,
    uint64_t registration_generation);
void openmuse_paired_buffer_free(OpenMusePairedBuffer buffer);

#ifdef __cplusplus
}
#endif

#endif
