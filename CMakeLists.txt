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

add_custom_command(
  OUTPUT "${lib}" "${h}"
  COMMAND go build -buildmode=c-shared -o "${lib}" ./bind
  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}")
if(WIN32)
  add_custom_command(
    OUTPUT "${import_lib}"
    COMMAND gendef - "${lib}" > "${CMAKE_CURRENT_BINARY_DIR}/mcserverkit.def"
    COMMAND
      dlltool -d "${CMAKE_CURRENT_BINARY_DIR}/mcserverkit.def" -l
      "${import_lib}"
    DEPENDS "${lib}")
  add_custom_target(bin ALL DEPENDS "${lib}" "${h}" "${import_lib}")
else()
  add_custom_target(bin ALL DEPENDS "${lib}" "${h}")
endif()

add_library(mcserverkit INTERFACE)

add_dependencies(mcserverkit bin)

target_include_directories(mcserverkit INTERFACE "${CMAKE_BINARY_DIR}")

if(WIN32)
  target_link_libraries(mcserverkit INTERFACE "${import_lib}")
else()
  target_link_libraries(mcserverkit INTERFACE "${lib}")
endif()
