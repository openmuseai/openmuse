#include "../src/ksp/runtime/HandleRegistry.h"
#include "../src/ksp/runtime/UnicodeText.h"

#include <algorithm>
#include <cstdlib>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

using namespace ksp::runtime;

namespace {
int passed = 0;
int failed = 0;

void check(bool value, const char* name) {
    if (value) {
        ++passed;
    } else {
        ++failed;
        std::cerr << "FAIL " << name << '\n';
    }
}

std::shared_ptr<void> marker(const std::string& name, std::vector<std::string>* destroyed) {
    return std::shared_ptr<void>(new int(1), [name, destroyed](void* value) {
        delete static_cast<int*>(value);
        destroyed->push_back(name);
    });
}
}  // namespace

int main() {
    const std::u16string supplementary = u"A\U0001f600B";
    check(isUnicodeTextBoundary(supplementary, 1), "boundary before surrogate pair");
    check(!isUnicodeTextBoundary(supplementary, 2), "reject middle of surrogate pair");
    check(isUnicodeTextBoundary(supplementary, 3), "boundary after surrogate pair");
    check(previousUnicodeTextBoundary(supplementary, 3) == 1,
          "backspace spans surrogate pair");
    check(nextUnicodeTextBoundary(supplementary, 1) == 3,
          "delete spans surrogate pair");
    check(hasUnsupportedSupplementaryInput(supplementary),
          "match original emoji input rejection");
    const std::u16string combining = u"Ae\u0301B";
    check(!isUnicodeTextBoundary(combining, 2), "reject boundary before combining mark");
    check(previousUnicodeTextBoundary(combining, 3) == 1,
          "backspace spans combining sequence");
    check(nextUnicodeTextBoundary(combining, 1) == 3,
          "delete spans combining sequence");
    check(baseTextDirection(u"hello") == TextDirection::Ltr,
          "latin base direction is LTR");
    check(baseTextDirection(u"\u05e9\u05dc\u05d5\u05dd") == TextDirection::Rtl,
          "hebrew base direction is RTL");
    check(baseTextDirection(u"-12 \u05e2\u05d5\u05dc\u05dd") == TextDirection::Rtl,
          "neutral prefix does not hide RTL base direction");
    check(baseTextDirection(u"123", TextDirection::Rtl) == TextDirection::Rtl,
          "neutral text preserves fallback direction");
    check(restoreLogicalRtlRuns(u"\u05dd\u05d5\u05dc\u05e9") == u"\u05e9\u05dc\u05d5\u05dd",
          "restore pure RTL visual run to logical order");
    check(restoreLogicalRtlRuns(u"\u05dd\u05dc\u05d5\u05e2-12") == u"\u05e2\u05d5\u05dc\u05dd-12",
          "preserve numeric suffix while restoring RTL run");
    check(restoreLogicalRtlRuns(u"\u05dd\u05d5\u05dc\u05e9\r\n\u05dd\u05dc\u05d5\u05e2") ==
              u"\u05e9\u05dc\u05d5\u05dd\r\n\u05e2\u05d5\u05dc\u05dd",
          "do not reorder RTL runs across line boundaries");

    HandleRegistry registry;
    std::vector<std::string> destroyed;
    ErrorCode error = ErrorCode::General;

    Handle doc1 = registry.insert(HandleType::Document, marker("doc1", &destroyed), 0, 0, &error);
    check(doc1 != 0 && error == ErrorCode::Ok, "insert document");
    check(HandleRegistry::hasTokenShape(doc1), "opaque token shape");

    Handle page1 = registry.insert(HandleType::Page, marker("page1", &destroyed), doc1, doc1, &error);
    check(page1 != 0 && error == ErrorCode::Ok, "insert owned page");
    check(registry.borrow(page1, HandleType::Page, doc1).error == ErrorCode::Ok,
          "validate page owner");
    check(registry.borrow(doc1, HandleType::Page).error == ErrorCode::WrongType,
          "reject wrong handle type");
    check(registry.borrow(42, HandleType::Document).error == ErrorCode::InvalidHandle,
          "reject malformed handle");

    Handle text1 = registry.insert(HandleType::TextPage, marker("text1", &destroyed),
                                  doc1, page1, &error);
    check(text1 != 0 && registry.findChild(page1, HandleType::TextPage) == text1,
          "insert and find page child");
    check(registry.borrow(text1, HandleType::TextPage, doc1, page1).error == ErrorCode::Ok,
          "validate document owner and direct parent");
    Handle save1 = registry.insert(HandleType::SaveSession, marker("save1", &destroyed),
                                  doc1, doc1, &error);
    check(save1 != 0 && registry.borrow(save1, HandleType::SaveSession, doc1, doc1),
          "insert document-owned SaveSession");
    Handle optimize1 = registry.insert(HandleType::OptimizeSession,
                                      marker("optimize1", &destroyed),
                                      doc1, doc1, &error);
    check(optimize1 != 0 &&
              registry.borrow(optimize1, HandleType::OptimizeSession, doc1, doc1),
          "insert document-owned OptimizeSession");
    Handle form1 = registry.insert(HandleType::FormFill, marker("form1", &destroyed),
                                  doc1, doc1, &error);
    check(form1 != 0 && registry.borrow(form1, HandleType::FormFill, doc1, doc1),
          "insert document-owned FormFill");
    Handle editor1 = registry.insert(HandleType::TextEditor, marker("editor1", &destroyed),
                                    doc1, form1, &error);
    check(editor1 != 0 && registry.borrow(editor1, HandleType::TextEditor, doc1, form1),
          "insert FormFill-owned TextEditor");
    Handle badEditor = registry.insert(HandleType::TextEditor, std::make_shared<int>(1),
                                      doc1, page1, &error);
    check(badEditor == 0 && error == ErrorCode::OwnerMismatch,
          "reject TextEditor with page parent");

    Handle doc2 = registry.insert(HandleType::Document, marker("doc2", &destroyed), 0, 0, &error);
    check(doc2 != 0, "insert second document");
    check(registry.borrow(page1, HandleType::Page, doc2).error == ErrorCode::OwnerMismatch,
          "reject cross-document page");
    check(registry.liveCount() == 8, "live count before close");

    BorrowedHandle activePage = registry.borrow(page1, HandleType::Page, doc1);
    check(static_cast<bool>(activePage), "active borrow owns payload");

    check(registry.invalidateDocumentTree(doc1) == ErrorCode::Ok, "close document tree");
    check(destroyed.size() == 6 && destroyed[0] == "editor1" &&
              destroyed[1] == "form1" && destroyed[2] == "text1" &&
              destroyed[3] == "save1" && destroyed[4] == "optimize1" &&
              destroyed[5] == "doc1",
          "editor/form/text/save/optimize children close before doc while page borrow delays page");
    check(registry.borrow(text1, HandleType::TextPage).error == ErrorCode::StaleHandle,
          "reject stale text page");
    check(registry.borrow(page1, HandleType::Page).error == ErrorCode::StaleHandle,
          "reject stale page");
    check(registry.borrow(doc1, HandleType::Document).error == ErrorCode::StaleHandle,
          "reject stale document");
    check(registry.invalidateDocumentTree(doc1) == ErrorCode::StaleHandle,
          "double close is predictable");
    activePage.payload.reset();
    check(destroyed.size() == 7 && destroyed[6] == "page1",
          "borrowed payload released after operation");

    Handle doc3 = registry.insert(HandleType::Document, marker("doc3", &destroyed), 0, 0, &error);
    check(doc3 != 0 && doc3 != doc1, "slot reuse changes generation");
    check(registry.borrow(doc1, HandleType::Document).error == ErrorCode::StaleHandle,
          "old generation stays stale");

    Handle badPage = registry.insert(HandleType::Page, marker("bad", &destroyed), page1, page1, &error);
    check(badPage == 0 && (error == ErrorCode::OwnerMismatch ||
                           error == ErrorCode::WrongType || error == ErrorCode::StaleHandle),
          "reject invalid page owner");
    check(registry.invalidate(doc2, HandleType::Page) == ErrorCode::WrongType,
          "reject wrong-type release");
    Handle page2 = registry.insert(HandleType::Page, marker("page2", &destroyed), doc2, doc2, &error);
    check(page2 != 0, "insert second document child");
    Handle text2 = registry.insert(HandleType::TextPage, marker("text2", &destroyed),
                                  doc2, page2, &error);
    check(text2 != 0, "insert second text child");
    Handle annotation2 = registry.insert(HandleType::Annotation, marker("annotation2", &destroyed),
                                        doc2, page2, &error);
    check(annotation2 != 0, "insert page-owned annotation child");
    Handle object2 = registry.insert(HandleType::PageObject, marker("object2", &destroyed),
                                    doc2, page2, &error);
    check(object2 != 0, "insert page-owned object child");
    check(registry.borrow(object2, HandleType::PageObject, doc2, page2).error == ErrorCode::Ok,
          "validate page-object type and owners");
    check(registry.borrow(object2, HandleType::PageObject, doc1, page2).error ==
              ErrorCode::OwnerMismatch,
          "reject cross-document page object");
    check(registry.invalidateChildren(page2, HandleType::Page, HandleType::PageObject) ==
              ErrorCode::Ok,
          "invalidate page-object inventory children");
    check(registry.borrow(object2, HandleType::PageObject).error == ErrorCode::StaleHandle,
          "inventory invalidation advances object generation");
    object2 = registry.insert(HandleType::PageObject, marker("object2b", &destroyed),
                             doc2, page2, &error);
    check(object2 != 0, "recreate page object after inventory invalidation");
    check(registry.invalidatePageTree(page2) == ErrorCode::Ok, "close page subtree");
    check(registry.borrow(text2, HandleType::TextPage).error == ErrorCode::StaleHandle,
          "page close cascades text child");
    const std::size_t tail = destroyed.size();
    const std::vector<std::string> pageChildren(
        destroyed.begin() + static_cast<std::ptrdiff_t>(tail - 4), destroyed.end() - 1);
    check(destroyed[tail - 1] == "page2" &&
              std::find(pageChildren.begin(), pageChildren.end(), "text2") != pageChildren.end() &&
              std::find(pageChildren.begin(), pageChildren.end(), "annotation2") != pageChildren.end() &&
              std::find(pageChildren.begin(), pageChildren.end(), "object2b") != pageChildren.end(),
          "page subtree destruction order");
    page2 = registry.insert(HandleType::Page, marker("page2b", &destroyed), doc2, doc2, &error);
    const std::size_t beforeDoc2Close = destroyed.size();
    check(registry.invalidateDocumentTree(doc2) == ErrorCode::Ok, "close second document");
    check(destroyed.size() == beforeDoc2Close + 2 &&
              destroyed[beforeDoc2Close] == "page2b" &&
              destroyed[beforeDoc2Close + 1] == "doc2",
          "close idle child before document");
    check(registry.invalidateDocumentTree(doc3) == ErrorCode::Ok, "close reused document");
    check(registry.liveCount() == 0, "no live handles after cleanup");

    std::cout << "HANDLE_REGISTRY PASS=" << passed << " FAIL=" << failed << '\n';
    return failed == 0 ? EXIT_SUCCESS : EXIT_FAILURE;
}
