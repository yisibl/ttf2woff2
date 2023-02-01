#include "woff2_c.hpp"

extern "C"
{
  size_t MaxWOFF2CompressedSize(const uint8_t *data, size_t length)
  {
    return woff2::MaxWOFF2CompressedSize(data, length);
  }

  size_t ComputeWOFF2FinalSize(const uint8_t *data, size_t length)
  {
    return woff2::ComputeWOFF2FinalSize(data, length);
  }

  bool ConvertTTFToWOFF2(const uint8_t *data, size_t length,
                         uint8_t *result, size_t *result_length)
  {
    return woff2::ConvertTTFToWOFF2(data, length, result, result_length);
  }

  bool ConvertWOFF2ToTTF(
    const uint8_t *data, size_t length, uint8_t **result, size_t *result_length
    )
  {
    // std::allocator<uint8_t> alloc;
    // size_t result_length = woff2::ComputeWOFF2FinalSize(data, length);
    // uint8_t *result = alloc.allocate(result_length);
    // if (result_length == 0)
    // {
    //   return false;
    // }

    std::string buffer(std::min(woff2::ComputeWOFF2FinalSize(data, length), woff2::kDefaultMaxSize), 0);
    woff2::WOFF2StringOut output(&buffer);

    // auto memory_out = new woff2::WOFF2MemoryOut(result, result_length);
    // out->inner = reinterpret_cast<Woff2MemoryOutInner *>(memory_out);
    // out->data = result;
    // out->length = result_length;
    auto ret = woff2::ConvertWOFF2ToTTF(data, length, &output);

    if (ret) {
        *result = static_cast<uint8_t*>(malloc(output.Size()));
        if (!*result) {
          return false;
        }
        memcpy(*result, buffer.data(), output.Size());
        *result_length = output.Size();
    } else {
        *result_length = 0;
    }
    return ret;
  }

  void FreeMemoryOutput(Woff2MemoryOutInner *out)
  {
    delete reinterpret_cast<woff2::WOFF2MemoryOut *>(out);
  }
}
