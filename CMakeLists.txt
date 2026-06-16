cmake_minimum_required(VERSION 3.16)
project(mcserverkit LANGUAGES C)

if (WIN32)
    set(output "mcserverkit.dll")
elseif(APPLE)
    set(output "mcserverkit.dylib")
else()
    set(output "mcserverkit.so")
endif()

set(lib "${CMAKE_CURRENT_BINARY_DIR}/${output}")
set(h "${CMAKE_CURRENT_BINARY_DIR}/mcserverkit.h")

add_custom_command(OUTPUT "${lib}" "${h}" COMMAND go build -buildmode=c-shared -o "${lib}" ./bind WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}")
add_custom_target(bin ALL DEPENDS "${lib}" "${h}")
add_library(mcserverkit SHARED IMPORTED GLOBAL)
add_dependencies(mcserverkit bin)

set_target_properties(mcserverkit PROPERTIES IMPORTED_LOCATION "${lib}" INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_BINARY_DIR}")