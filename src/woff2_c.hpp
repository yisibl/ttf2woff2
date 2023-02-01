#ifndef WOFF2_CAPI_H
#define WOFF2_CAPI_H

#include <stddef.h>
#include <inttypes.h>

#include <woff2/encode.h>
#include <woff2/decode.h>

typedef struct Woff2MemoryOutInner Woff2MemoryOutInner;

struct Woff2MemoryOut
{
  u_int8_t *data;
  size_t length;
  Woff2MemoryOutInner *inner;
};

extern "C"
{
  size_t MaxWOFF2CompressedSize(const uint8_t *data, size_t length);
  size_t ComputeWOFF2FinalSize(const uint8_t *data, size_t length);

  bool ConvertTTFToWOFF2(const uint8_t *data, size_t length,
                         uint8_t *result, size_t *result_length);

  bool ConvertWOFF2ToTTF(const uint8_t *data, size_t length,
                         uint8_t **result, size_t *result_length);

  void FreeMemoryOutput(Woff2MemoryOutInner *out);
}

#endif // WOFF2_CAPI_H
