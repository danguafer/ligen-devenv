CMAKE_MINIMUM_REQUIRED(VERSION 3.0)
PROJECT("geom-cpp")

AUX_SOURCE_DIRECTORY(src SOURCES)
AUX_SOURCE_DIRECTORY(../target/debug/ligen/cpp/src SOURCES)

ADD_EXECUTABLE(${PROJECT_NAME} ${SOURCES})
TARGET_INCLUDE_DIRECTORIES(${PROJECT_NAME} PUBLIC ../target/debug/ligen/c/include)
TARGET_INCLUDE_DIRECTORIES(${PROJECT_NAME} PUBLIC ../target/debug/ligen/cpp/include)
#LINK_DIRECTORIES(../../target/debug/)
TARGET_LINK_LIBRARIES(${PROJECT_NAME} wsock32 ws2_32 userenv)
TARGET_LINK_LIBRARIES(${PROJECT_NAME} ../../target/debug/geom)
TARGET_LINK_LIBRARIES(${PROJECT_NAME} ../../target/debug/gpu dwmapi opengl32)
#TARGET_COMPILE_OPTIONS(${PROJECT_NAME} PUBLIC -std=c++11)
