#pragma once

#include "Error.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <mutex>
#include <vector>

namespace ksp::runtime {

using Handle = uint64_t;

enum class HandleType : uint8_t {
    Document = 1,
    Page = 2,
    TextPage = 3,
    Annotation = 4,
    RenderSession = 5,
    SaveSession = 6,
    OptimizeSession = 7,
    PageObject = 8,
    EditUndo = 9,
    FormFill = 10,
    TextEditor = 11,
};

struct BorrowedHandle {
    std::shared_ptr<void> payload;
    HandleType type = HandleType::Document;
    Handle documentOwner = 0;
    Handle parent = 0;
    ErrorCode error = ErrorCode::InvalidHandle;

    explicit operator bool() const { return error == ErrorCode::Ok && payload != nullptr; }

    template <typename T>
    std::shared_ptr<T> as() const {
        return std::static_pointer_cast<T>(payload);
    }
};

class HandleRegistry {
public:
    Handle insert(HandleType type, std::shared_ptr<void> payload,
                  Handle documentOwner = 0, Handle parent = 0,
                  ErrorCode* error = nullptr);

    BorrowedHandle borrow(Handle handle, HandleType expected,
                          Handle expectedDocumentOwner = 0,
                          Handle expectedParent = 0) const;

    Handle findChild(Handle parent, HandleType type) const;

    ErrorCode invalidate(Handle handle, HandleType expected);
    ErrorCode invalidateChildren(Handle parent, HandleType parentType,
                                 HandleType childType);
    ErrorCode invalidatePageTree(Handle page);
    ErrorCode invalidateDocumentChildren(Handle document);
    ErrorCode invalidateDocumentTree(Handle document);

    bool isValid(Handle handle, HandleType expected) const;
    std::size_t liveCount() const;

    static bool hasTokenShape(Handle handle);

private:
    struct Slot {
        std::shared_ptr<void> payload;
        Handle documentOwner = 0;
        Handle parent = 0;
        HandleType type = HandleType::Document;
        uint16_t generation = 1;
        bool live = false;
    };

    struct Decoded {
        uint32_t slot = 0;
        uint16_t generation = 0;
        HandleType type = HandleType::Document;
        bool valid = false;
    };

    static Handle encode(uint32_t slot, uint16_t generation, HandleType type);
    static Decoded decode(Handle handle);
    ErrorCode validateLocked(const Decoded& decoded, HandleType expected,
                             Handle expectedDocumentOwner, Handle expectedParent,
                             const Slot** slot) const;
    // Returns true while the slot can be reused without wrapping its generation.
    static bool retireLocked(Slot& slot);

    mutable std::mutex mutex_;
    std::vector<Slot> slots_;
    std::vector<uint32_t> freeSlots_;
};

HandleRegistry& handles();

}  // namespace ksp::runtime
