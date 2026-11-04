#include "../src/ksp/product/ProductBridge.h"

#include <cstdint>
#include <fstream>
#include <iostream>
#include <iterator>
#include <vector>

int main(int argc, char** argv) {
    if (argc != 3) return 2;
    std::ifstream input(argv[1], std::ios::binary);
    const std::vector<uint8_t> source{std::istreambuf_iterator<char>(input),
                                      std::istreambuf_iterator<char>()};
    if (source.empty() || ksp_product_abi_version() != 2 ||
        ksp_product_page_count(source.data(), source.size()) != 2) return 3;

    const int32_t extracted[] = {1};
    const int32_t original[] = {0, 1};
    const KspProductPageSelection selections[] = {
        {source.data(), source.size(), extracted, 1},
        {source.data(), source.size(), original, 2},
    };
    KspProductBytes output{};
    char error[256]{};
    const int status = ksp_product_compose_pages(
        selections, 2, &output, error, sizeof(error));
    if (status != 0) {
        std::cerr << "compose failed: " << error << '\n';
        return 4;
    }
    const int count = ksp_product_page_count(output.data, output.size);
    std::ofstream result(argv[2], std::ios::binary);
    result.write(reinterpret_cast<const char*>(output.data), output.size);
    ksp_product_free_bytes(&output);
    if (count != 3 || !result.good()) return 5;
    std::cout << "PRODUCT_ABI_ANDROID PASS pages=3\n";
    return 0;
}
