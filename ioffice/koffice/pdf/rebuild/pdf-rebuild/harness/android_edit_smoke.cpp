#include "../src/ksp/product/ProductEditBridge.h"

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <cstring>
#include <fstream>
#include <iostream>
#include <iterator>
#include <vector>

std::vector<uint16_t> readDomain(KspProductEditSession* session) {
    const int length = ksp_product_edit_domain_text(session, 0, nullptr, 0);
    if (length < 1) return {};
    std::vector<uint16_t> result(length);
    if (ksp_product_edit_domain_text(session, 0, result.data(), length) != length)
        return {};
    return result;
}

int main(int argc, char** argv) {
    if (argc != 4) return 2;
    if (ksp_product_edit_abi_version() != 6) return 10;
    std::ifstream input(argv[1], std::ios::binary);
    const std::vector<uint8_t> source{std::istreambuf_iterator<char>(input),
                                      std::istreambuf_iterator<char>()};
    if (source.empty()) return 3;
    int error = 0;
    KspProductEditSession* session = ksp_product_edit_open_bytes(
        source.data(), source.size(), &error);
    if (!session) {
        std::cerr << "open error=" << error << '\n';
        return 4;
    }
    std::ifstream linkInput(argv[3], std::ios::binary);
    const std::vector<uint8_t> linkedSource{
        std::istreambuf_iterator<char>(linkInput),
        std::istreambuf_iterator<char>()};
    KspProductEditSession* linked = ksp_product_edit_open_bytes(
        linkedSource.data(), linkedSource.size(), &error);
    KspProductPageLinks pageLinks{};
    const int linkStatus = linked ? ksp_product_edit_read_page_links(
        linked, 0, &pageLinks) : -1;
    const char* expectedUri = "https://example.com/pdf";
    const bool linksValid = linkStatus == 0 && pageLinks.count == 2 &&
        pageLinks.links && pageLinks.uri_data &&
        pageLinks.links[0].kind == 1 &&
        pageLinks.links[0].uri_length == std::strlen(expectedUri) &&
        std::memcmp(pageLinks.uri_data + pageLinks.links[0].uri_offset,
                    expectedUri, std::strlen(expectedUri)) == 0 &&
        pageLinks.links[1].kind == 2 &&
        pageLinks.links[1].page_index == 1;
    ksp_product_edit_free_page_links(&pageLinks);
    const bool linksFreed = !pageLinks.links && !pageLinks.uri_data &&
        pageLinks.count == 0 && pageLinks.uri_size == 0;
    if (linked) ksp_product_edit_close(linked);
    if (!linksValid || !linksFreed) {
        ksp_product_edit_close(session);
        return 12;
    }
    const auto close = [&] { ksp_product_edit_close(session); };
    KspProductPageText pageText{};
    const int textStatus = ksp_product_edit_read_page_text(session, 1,
                                                           &pageText);
    bool textValid = textStatus == 0 && pageText.count > 0 && pageText.chars;
    for (int i = 0; i < pageText.count && textValid; ++i) {
        const auto& ch = pageText.chars[i];
        textValid = std::isfinite(ch.left) && std::isfinite(ch.bottom) &&
            std::isfinite(ch.right) && std::isfinite(ch.top);
    }
    ksp_product_edit_free_page_text(&pageText);
    textValid = textValid && !pageText.chars && pageText.count == 0 &&
        ksp_product_edit_read_page_text(session, 99, &pageText) < 0 &&
        !pageText.chars && pageText.count == 0;
    float pageWidth = 0, pageHeight = 0;
    constexpr int rasterWidth = 320, rasterHeight = 480;
    std::vector<uint8_t> before(rasterWidth * rasterHeight * 4);
    const int sizeStatus = ksp_product_edit_page_size(
        session, 0, &pageWidth, &pageHeight);
    const int beforeStatus = ksp_product_edit_render_page_at(
        session, 0, rasterWidth, rasterHeight, before.data(), before.size());
    constexpr int tileX = 53, tileY = 71, tileWidth = 128, tileHeight = 96;
    std::vector<uint8_t> tile(tileWidth * tileHeight * 4);
    const int tileStatus = ksp_product_edit_render_page_region(
        session, 0, tileX, tileY, tileWidth, tileHeight, rasterWidth,
        rasterHeight, tile.data(), tile.size());
    bool tileMatches = tileStatus == static_cast<int>(tile.size());
    for (int row = 0; row < tileHeight && tileMatches; ++row) {
        const auto* fullRow = before.data() +
            ((tileY + row) * rasterWidth + tileX) * 4;
        const auto* tileRow = tile.data() + row * tileWidth * 4;
        tileMatches = std::equal(fullRow, fullRow + tileWidth * 4, tileRow);
    }
    if (!textValid || sizeStatus != 0 || pageWidth <= 0 || pageHeight <= 0 ||
        beforeStatus != static_cast<int>(before.size()) || !tileMatches ||
        ksp_product_edit_render_page_region(session, 0, 300, 0, 128, 96,
                                              rasterWidth, rasterHeight,
                                              tile.data(), tile.size()) >= 0 ||
        ksp_product_edit_render_page_at(session, 99, rasterWidth,
                                         rasterHeight, before.data(),
                                         before.size()) >= 0) {
        close();
        return 11;
    }
    if (ksp_product_edit_page_count(session) != 2 ||
        ksp_product_edit_load_page(session, 0) < 0 ||
        ksp_product_edit_domain_count(session) < 1) {
        close();
        return 5;
    }
    const auto original = readDomain(session);
    if (original.empty()) {
        close();
        return 6;
    }
    auto changed = original;
    changed.push_back('!');
    const auto replaceStatus = ksp_product_edit_replace_domain(
        session, 0, changed.data(), changed.size());
    const int changedTextStatus = ksp_product_edit_read_page_text(
        session, 0, &pageText);
    bool containsEdit = changedTextStatus == 0 && pageText.chars;
    if (containsEdit) {
        containsEdit = std::any_of(pageText.chars,
                                   pageText.chars + pageText.count,
                                   [](const KspProductTextChar& ch) {
                                       return ch.utf16 == '!';
                                   });
    }
    ksp_product_edit_free_page_text(&pageText);
    std::vector<uint8_t> after(before.size());
    const int afterStatus = ksp_product_edit_render_page_at(
        session, 0, rasterWidth, rasterHeight, after.data(), after.size());
    const auto afterReplace = readDomain(session);
    const auto canUndo = ksp_product_edit_can_undo(session);
    const auto undoStatus = ksp_product_edit_undo(session);
    const auto afterUndo = readDomain(session);
    const auto canRedo = ksp_product_edit_can_redo(session);
    const auto redoStatus = ksp_product_edit_redo(session);
    const auto afterRedo = readDomain(session);
    KspProductBytes snapshot{};
    const auto snapshotStatus = ksp_product_edit_export_bytes(session, &snapshot);
    KspProductEditSession* snapshotSession = snapshotStatus == 0
        ? ksp_product_edit_open_bytes(snapshot.data, snapshot.size, &error)
        : nullptr;
    const bool snapshotOk = snapshotSession &&
        ksp_product_edit_page_count(snapshotSession) == 2 &&
        ksp_product_edit_load_page(snapshotSession, 0) == 0 &&
        readDomain(snapshotSession) == afterRedo;
    if (snapshotSession) ksp_product_edit_close(snapshotSession);
    ksp_product_free_bytes(&snapshot);
    const auto saveStatus = ksp_product_edit_save(session, argv[2]);
    if (replaceStatus < 0 || !containsEdit ||
        afterStatus != static_cast<int>(after.size()) ||
        after == before || afterReplace.empty() ||
        afterReplace.back() != '!' || canUndo != 1 ||
        undoStatus < 0 || afterUndo != original || canRedo != 1 ||
        redoStatus < 0 || afterRedo != afterReplace || !snapshotOk ||
        saveStatus < 0) {
        std::cerr << "edit status replace=" << replaceStatus
                  << " changed=" << (!afterReplace.empty() && afterReplace.back() == '!')
                  << " undo=" << canUndo << '/' << undoStatus
                  << " restored=" << (afterUndo == original)
                  << " redo=" << canRedo << '/' << redoStatus
                  << " redone=" << (afterRedo == afterReplace)
                  << " snapshot=" << snapshotStatus << '/' << snapshotOk
                  << " save=" << saveStatus << '\n';
        const auto printText = [](const char* label,
                                  const std::vector<uint16_t>& value) {
            std::cerr << label << " len=" << value.size() << " text=";
            for (const auto unit : value)
                std::cerr << (unit >= 32 && unit < 127 ? char(unit) : '?');
            std::cerr << '\n';
        };
        printText("original", original);
        printText("changed", changed);
        printText("afterReplace", afterReplace);
        printText("afterRedo", afterRedo);
        close();
        return 8;
    }
    close();
    std::ifstream saved(argv[2], std::ios::binary);
    const std::vector<uint8_t> bytes{std::istreambuf_iterator<char>(saved),
                                     std::istreambuf_iterator<char>()};
    session = ksp_product_edit_open_bytes(bytes.data(), bytes.size(), &error);
    if (!session || ksp_product_edit_page_count(session) != 2 ||
        ksp_product_edit_load_page(session, 0) < 0 ||
        readDomain(session) != afterReplace) {
        if (session) close();
        return 9;
    }
    close();
    std::cout << "PRODUCT_EDIT PASS pages=2 bytes=" << bytes.size() << '\n';
    return 0;
}
