cmake_minimum_required(VERSION 3.16)
project(mcserverkit LANGUAGES C)

if(CMAKE_SYSTEM_PROCESSOR MATCHES "arm64|aarch64")
  set(architecture "arm64")
else()
  set(architecture "x86_64")
endif()

if(WIN32)
  set(lib "${CMAKE_BINARY_DIR}/mcserverkit.dll")
  set(import_lib "${CMAKE_BINARY_DIR}/libmcserverkit.a")
  set(os "windows")
elseif(APPLE)
  set(lib "${CMAKE_BINARY_DIR}/mcserverkit.dylib")
  set(os "macos")
else()
  set(lib "${CMAKE_BINARY_DIR}/mcserverkit.so")
  set(os "linux")
endif()

set(h "${CMAKE_BINARY_DIR}/mcserverkit.h")

set(asset "mcserverkit.${os}-${architecture}.zip")
set(url
    "https://github.com/mcserverkit/mcserverkit/releases/latest/download/${asset}")

# https://cmake.org/cmake/help/latest/command/file.html

set(zip "${CMAKE_BINARY_DIR}/${asset}")
file(DOWNLOAD "${url}" "${zip}")
file(ARCHIVE_EXTRACT INPUT "${zip}" DESTINATION "${CMAKE_BINARY_DIR}")

add_library(mcserverkit INTERFACE)

target_include_directories(mcserverkit INTERFACE "${CMAKE_BINARY_DIR}")

if(WIN32)
  target_link_libraries(mcserverkit INTERFACE "${import_lib}")
else()
  target_link_libraries(mcserverkit INTERFACE "${lib}")
endif()
