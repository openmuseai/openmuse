cmake_minimum_required(VERSION 3.14)
set(PROJECT_NAME "openmuse_dsh_plugin")
project(${PROJECT_NAME} LANGUAGES CXX)

set(PLUGIN_NAME "openmuse_dsh_plugin_plugin")

# Flutter configures plugins through ephemeral/.plugin_symlinks. A fixed
# ../../.. from that link is the ephemeral directory, not this repository.
function(openmuse_repo_root result)
  set(candidates "${CMAKE_CURRENT_SOURCE_DIR}")
  get_filename_component(resolved "${CMAKE_CURRENT_SOURCE_DIR}" REALPATH)
  list(APPEND candidates "${resolved}")
  foreach(start IN LISTS candidates)
    set(dir "${start}")
    foreach(_step RANGE 0 12)
      if(EXISTS "${dir}/scripts/ci/stage-webview2.ps1")
        set(${result} "${dir}" PARENT_SCOPE)
        return()
      endif()
      get_filename_component(parent "${dir}" DIRECTORY)
      if(parent STREQUAL dir)
        break()
      endif()
      set(dir "${parent}")
    endforeach()
  endforeach()
  set(${result} "" PARENT_SCOPE)
endfunction()

openmuse_repo_root(REPO_ROOT)
set(WEBVIEW2_HEADER "${REPO_ROOT}/target/webview2/include/WebView2.h")
set(WEBVIEW2_LIB "${REPO_ROOT}/target/webview2/lib/WebView2LoaderStatic.lib")

if(NOT EXISTS "${WEBVIEW2_HEADER}" OR NOT EXISTS "${WEBVIEW2_LIB}")
  execute_process(
    COMMAND powershell -NoProfile -ExecutionPolicy Bypass -File
            "${REPO_ROOT}/scripts/ci/stage-webview2.ps1"
    RESULT_VARIABLE WEBVIEW2_STAGE_RESULT
  )
  if(NOT WEBVIEW2_STAGE_RESULT EQUAL 0 OR NOT EXISTS "${WEBVIEW2_HEADER}")
    message(FATAL_ERROR "WebView2 SDK is missing. Run scripts/ci/stage-webview2.ps1")
  endif()
endif()

add_library(${PLUGIN_NAME} SHARED
  "openmuse_dsh_plugin.cpp"
  "openmuse_dsh_plugin_c_api.cpp"
)

# Exceptions stay on for Microsoft::WRL. apply_standard_settings defines
# _HAS_EXCEPTIONS=0, which breaks that header, so the warning bar is repeated
# here instead of calling it.
target_compile_features(${PLUGIN_NAME} PUBLIC cxx_std_17)
target_compile_options(${PLUGIN_NAME} PRIVATE /W4 /WX /wd"4100" /wd"4265" /EHsc /utf-8)
target_compile_definitions(${PLUGIN_NAME} PRIVATE FLUTTER_PLUGIN_IMPL NOMINMAX "_HAS_EXCEPTIONS=1")
target_compile_definitions(${PLUGIN_NAME} PRIVATE "$<$<CONFIG:Debug>:_DEBUG>")
set_target_properties(${PLUGIN_NAME} PROPERTIES CXX_VISIBILITY_PRESET hidden)
target_include_directories(${PLUGIN_NAME} INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_include_directories(${PLUGIN_NAME} PRIVATE "${REPO_ROOT}/target/webview2/include")
target_link_libraries(${PLUGIN_NAME} PRIVATE flutter flutter_wrapper_plugin
  "${WEBVIEW2_LIB}" version ole32 shell32 user32)

set(openmuse_dsh_plugin_bundled_libraries "" PARENT_SCOPE)
