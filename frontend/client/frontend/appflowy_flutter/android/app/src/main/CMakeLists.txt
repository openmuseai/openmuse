cmake_minimum_required(VERSION 3.10.0)

project(AppFlowy)

# Resolve the active ABI through the NDK toolchain. Recent NDKs keep this
# library in the LLVM sysroot, not the removed sources/cxx-stl directory.
find_library(APPFLOWY_CXX_SHARED c++_shared)
if(NOT APPFLOWY_CXX_SHARED)
    message(FATAL_ERROR "libc++_shared.so missing from Android NDK for ${ANDROID_ABI}")
endif()
file(COPY "${APPFLOWY_CXX_SHARED}"
    DESTINATION "${CMAKE_CURRENT_SOURCE_DIR}/jniLibs/${ANDROID_ABI}")
