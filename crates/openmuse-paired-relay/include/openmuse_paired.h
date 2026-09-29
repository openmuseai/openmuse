#ifndef OPENMUSE_PAIRED_H
#define OPENMUSE_PAIRED_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define OPENMUSE_PAIRED_ABI_VERSION 1u
#define OPENMUSE_PAIRED_DEVICE_PUBLIC_BYTES 64u

uint32_t openmuse_paired_abi_version(void);
int32_t openmuse_paired_device_public(
    const uint8_t *seed,
    size_t seed_len,
    uint8_t *output,
    size_t output_len);

#ifdef __cplusplus
}
#endif

#endif
