#include "HandleRegistry.h"

#include <algorithm>

namespace ksp::runtime {
namespace {
constexpr uint16_t kMagic = 0x4b53;  // "KS"
constexpr uint64_t kSlotMask = 0x00ffffffULL;
constexpr uint32_t kMaxSlots = static_cast<uint32_t>(kSlotMask);

void storeError(ErrorCode code, ErrorCode* out) {
    setLastError(code);
    if (out) *out = code;
}
}  // namespace

Handle HandleRegistry::encode(uint32_t slot, uint16_t generation, HandleType type) {
    const uint64_t slotPlusOne = static_cast<uint64_t>(slot) + 1;
    return (static_cast<uint64_t>(kMagic) << 48) |
           (static_cast<uint64_t>(type) << 40) |
           (static_cast<uint64_t>(generation) << 24) |
           slotPlusOne;
}

HandleRegistry::Decoded HandleRegistry::decode(Handle handle) {
    Decoded out;
    if (static_cast<uint16_t>(handle >> 48) != kMagic) return out;
    const uint64_t slotPlusOne = handle & kSlotMask;
    if (slotPlusOne == 0) return out;
    const uint8_t type = static_cast<uint8_t>((handle >> 40) & 0xff);
    if (type < static_cast<uint8_t>(HandleType::Document) ||
        type > static_cast<uint8_t>(HandleType::TextEditor)) return out;
    out.slot = static_cast<uint32_t>(slotPlusOne - 1);
    out.generation = static_cast<uint16_t>((handle >> 24) & 0xffff);
    out.type = static_cast<HandleType>(type);
    out.valid = out.generation != 0;
    return out;
}

bool HandleRegistry::hasTokenShape(Handle handle) { return decode(handle).valid; }

ErrorCode HandleRegistry::validateLocked(const Decoded& decoded,
                                         HandleType expected,
                                         Handle expectedDocumentOwner,
                                         Handle expectedParent,
                                         const Slot** result) const {
    if (!decoded.valid || decoded.slot >= slots_.size()) return ErrorCode::InvalidHandle;
    if (decoded.type != expected) return ErrorCode::WrongType;
    const Slot& slot = slots_[decoded.slot];
    if (slot.generation != decoded.generation || !slot.live) return ErrorCode::StaleHandle;
    if (slot.type != expected) return ErrorCode::WrongType;
    if (expectedDocumentOwner != 0 && slot.documentOwner != expectedDocumentOwner)
        return ErrorCode::OwnerMismatch;
    if (expectedParent != 0 && slot.parent != expectedParent)
        return ErrorCode::OwnerMismatch;
    if (!slot.payload) return ErrorCode::StaleHandle;
    if (result) *result = &slot;
    return ErrorCode::Ok;
}

Handle HandleRegistry::insert(HandleType type, std::shared_ptr<void> payload,
                              Handle documentOwner, Handle parent, ErrorCode* error) {
    if (!payload) {
        storeError(ErrorCode::InvalidArgument, error);
        return 0;
    }
    std::lock_guard<std::mutex> lock(mutex_);
    if (type == HandleType::Document) {
        if (documentOwner != 0 || parent != 0) {
            storeError(ErrorCode::OwnerMismatch, error);
            return 0;
        }
    } else {
        const Slot* documentSlot = nullptr;
        const ErrorCode ownerError = validateLocked(
            decode(documentOwner), HandleType::Document, 0, 0, &documentSlot);
        if (ownerError != ErrorCode::Ok) {
            storeError(ownerError == ErrorCode::WrongType ? ErrorCode::OwnerMismatch : ownerError,
                       error);
            return 0;
        }
        HandleType requiredParent = HandleType::Page;
        switch (type) {
            case HandleType::Page:
            case HandleType::SaveSession:
            case HandleType::OptimizeSession:
            case HandleType::EditUndo:
            case HandleType::FormFill:
                requiredParent = HandleType::Document;
                break;
            case HandleType::TextEditor:
                requiredParent = HandleType::FormFill;
                break;
            default:
                break;
        }
        const Handle requiredParentHandle = parent == 0 ? documentOwner : parent;
        const Slot* parentSlot = nullptr;
        const ErrorCode parentError = validateLocked(
            decode(requiredParentHandle), requiredParent, 0, 0, &parentSlot);
        if (parentError != ErrorCode::Ok ||
            (requiredParent != HandleType::Document &&
             parentSlot->documentOwner != documentOwner)) {
            storeError(parentError == ErrorCode::Ok || parentError == ErrorCode::WrongType
                           ? ErrorCode::OwnerMismatch
                           : parentError,
                       error);
            return 0;
        }
        parent = requiredParentHandle;
    }

    uint32_t index;
    if (!freeSlots_.empty()) {
        index = freeSlots_.back();
        freeSlots_.pop_back();
    } else {
        if (slots_.size() >= kMaxSlots) {
            storeError(ErrorCode::General, error);
            return 0;
        }
        index = static_cast<uint32_t>(slots_.size());
        slots_.emplace_back();
    }
    Slot& slot = slots_[index];
    slot.payload = std::move(payload);
    slot.documentOwner = documentOwner;
    slot.parent = parent;
    slot.type = type;
    slot.live = true;
    if (slot.generation == 0) slot.generation = 1;
    storeError(ErrorCode::Ok, error);
    return encode(index, slot.generation, type);
}

BorrowedHandle HandleRegistry::borrow(Handle handle, HandleType expected,
                                      Handle expectedDocumentOwner,
                                      Handle expectedParent) const {
    BorrowedHandle result;
    std::lock_guard<std::mutex> lock(mutex_);
    const Slot* slot = nullptr;
    result.error = validateLocked(decode(handle), expected, expectedDocumentOwner,
                                  expectedParent, &slot);
    setLastError(result.error);
    if (result.error == ErrorCode::Ok) {
        result.payload = slot->payload;
        result.type = slot->type;
        result.documentOwner = slot->documentOwner;
        result.parent = slot->parent;
    }
    return result;
}

Handle HandleRegistry::findChild(Handle parent, HandleType type) const {
    std::lock_guard<std::mutex> lock(mutex_);
    for (uint32_t i = 0; i < slots_.size(); ++i) {
        const Slot& slot = slots_[i];
        if (slot.live && slot.type == type && slot.parent == parent && slot.payload) {
            setLastError(ErrorCode::Ok);
            return encode(i, slot.generation, slot.type);
        }
    }
    setLastError(ErrorCode::InvalidHandle);
    return 0;
}

bool HandleRegistry::retireLocked(Slot& slot) {
    slot.live = false;
    slot.documentOwner = 0;
    slot.parent = 0;
    if (slot.generation == UINT16_MAX) {
        // Never wrap to a generation that could make an old token valid again.
        slot.generation = 0;
        return false;
    }
    slot.generation = static_cast<uint16_t>(slot.generation + 1);
    return true;
}

ErrorCode HandleRegistry::invalidate(Handle handle, HandleType expected) {
    std::shared_ptr<void> payload;
    uint32_t index = 0;
    {
        std::lock_guard<std::mutex> lock(mutex_);
        const Decoded decoded = decode(handle);
        const Slot* checked = nullptr;
        const ErrorCode code = validateLocked(decoded, expected, 0, 0, &checked);
        if (code != ErrorCode::Ok) {
            setLastError(code);
            return code;
        }
        index = decoded.slot;
        Slot& slot = slots_[index];
        payload = std::move(slot.payload);
        if (retireLocked(slot)) freeSlots_.push_back(index);
    }
    payload.reset();
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode HandleRegistry::invalidateChildren(Handle parent, HandleType parentType,
                                             HandleType childType) {
    std::vector<std::shared_ptr<void>> payloads;
    {
        std::lock_guard<std::mutex> lock(mutex_);
        const Slot* checked = nullptr;
        const ErrorCode code = validateLocked(decode(parent), parentType, 0, 0, &checked);
        if (code != ErrorCode::Ok) {
            setLastError(code);
            return code;
        }
        for (uint32_t i = 0; i < slots_.size(); ++i) {
            Slot& slot = slots_[i];
            if (!slot.live || slot.parent != parent || slot.type != childType) continue;
            payloads.push_back(std::move(slot.payload));
            if (retireLocked(slot)) freeSlots_.push_back(i);
        }
    }
    for (auto& payload : payloads) payload.reset();
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode HandleRegistry::invalidatePageTree(Handle page) {
    std::vector<std::shared_ptr<void>> payloads;
    {
        std::lock_guard<std::mutex> lock(mutex_);
        const Decoded decoded = decode(page);
        const Slot* checked = nullptr;
        const ErrorCode code = validateLocked(decoded, HandleType::Page, 0, 0, &checked);
        if (code != ErrorCode::Ok) {
            setLastError(code);
            return code;
        }
        for (uint32_t i = 0; i < slots_.size(); ++i) {
            Slot& slot = slots_[i];
            if (!slot.live || slot.parent != page) continue;
            payloads.push_back(std::move(slot.payload));
            if (retireLocked(slot)) freeSlots_.push_back(i);
        }
        Slot& pageSlot = slots_[decoded.slot];
        payloads.push_back(std::move(pageSlot.payload));
        if (retireLocked(pageSlot)) freeSlots_.push_back(decoded.slot);
    }
    for (auto& payload : payloads) payload.reset();
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode HandleRegistry::invalidateDocumentChildren(Handle document) {
    std::vector<std::shared_ptr<void>> children;
    {
        std::lock_guard<std::mutex> lock(mutex_);
        const Decoded decoded = decode(document);
        const Slot* checked = nullptr;
        const ErrorCode code = validateLocked(
            decoded, HandleType::Document, 0, 0, &checked);
        if (code != ErrorCode::Ok) {
            setLastError(code);
            return code;
        }
        for (HandleType type : {HandleType::TextEditor, HandleType::FormFill,
                                HandleType::TextPage, HandleType::Annotation,
                                HandleType::RenderSession, HandleType::SaveSession,
                                HandleType::OptimizeSession, HandleType::PageObject,
                                HandleType::EditUndo, HandleType::Page}) {
            for (uint32_t i = 0; i < slots_.size(); ++i) {
                Slot& slot = slots_[i];
                if (!slot.live || slot.documentOwner != document || slot.type != type)
                    continue;
                children.push_back(std::move(slot.payload));
                if (retireLocked(slot)) freeSlots_.push_back(i);
            }
        }
    }
    for (auto& child : children) child.reset();
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode HandleRegistry::invalidateDocumentTree(Handle document) {
    std::vector<std::shared_ptr<void>> children;
    std::shared_ptr<void> documentPayload;
    {
        std::lock_guard<std::mutex> lock(mutex_);
        const Decoded decoded = decode(document);
        const Slot* checked = nullptr;
        const ErrorCode code = validateLocked(decoded, HandleType::Document, 0, 0, &checked);
        if (code != ErrorCode::Ok) {
            setLastError(code);
            return code;
        }
        // Direct page children depend on their TextPage children. Retire and
        // destroy all deeper objects before pages, then the document itself.
        for (HandleType type : {HandleType::TextEditor, HandleType::FormFill,
                                HandleType::TextPage, HandleType::Annotation,
                                HandleType::RenderSession, HandleType::SaveSession,
                                HandleType::OptimizeSession, HandleType::PageObject,
                                HandleType::EditUndo, HandleType::Page}) {
            for (uint32_t i = 0; i < slots_.size(); ++i) {
                Slot& slot = slots_[i];
                if (!slot.live || slot.documentOwner != document || slot.type != type)
                    continue;
                children.push_back(std::move(slot.payload));
                if (retireLocked(slot)) freeSlots_.push_back(i);
            }
        }
        Slot& docSlot = slots_[decoded.slot];
        documentPayload = std::move(docSlot.payload);
        if (retireLocked(docSlot)) freeSlots_.push_back(decoded.slot);
    }
    // Close child PDFium objects before allowing the document to be destroyed.
    for (auto& child : children) child.reset();
    documentPayload.reset();
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

bool HandleRegistry::isValid(Handle handle, HandleType expected) const {
    return borrow(handle, expected).error == ErrorCode::Ok;
}

std::size_t HandleRegistry::liveCount() const {
    std::lock_guard<std::mutex> lock(mutex_);
    return static_cast<std::size_t>(std::count_if(
        slots_.begin(), slots_.end(), [](const Slot& slot) { return slot.live; }));
}

HandleRegistry& handles() {
    static HandleRegistry registry;
    return registry;
}

}  // namespace ksp::runtime
