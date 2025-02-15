
        message(WARNING "Using `SPIRVTargets.cmake` is deprecated: use `find_package(glslang)` to find glslang CMake targets.")

        if (NOT TARGET glslang::SPIRV)
            include("D:/msys64/home/guyou/dev/ncnn-20241226-full-source/build/install/lib/cmake/glslang/glslang-targets.cmake")
        endif()

        add_library(SPIRV ALIAS glslang::SPIRV)
    