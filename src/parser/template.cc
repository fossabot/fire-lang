#include <iostream>
#include <cstdint>
#include <string>

template <typename T>
void __fire_print(T t) {
    std::cout << t;
}

template <typename T>
void __fire_println(T t) {
    std::cout << t << std::endl;
}

template<typename T, typename... Args>
void __fire_print(T t, Args... args) {
    std::cout << t << " ";
    __fire_print(args...);
}

template<typename T, typename... Args>
void __fire_println(T t, Args... args) {
    std::cout << t << " ";
    __fire_println(args...);
}

typedef int8_t __fire_i8;
typedef int16_t __fire_i16;
typedef int32_t __fire_i32;
typedef int64_t __fire_i64;
typedef int8_t __fire_u8;
typedef int16_t __fire_u16;
typedef int32_t __fire_u32;
typedef int64_t __fire_u64;
typedef std::string __fire_string;
