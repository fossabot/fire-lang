#include <iostream>
#include <cstdint>
#include <string>
#include <vector>
#include <map>

typedef float __fire_f32;
typedef double __fire_f64;
typedef int8_t __fire_i8;
typedef int16_t __fire_i16;
typedef int32_t __fire_i32;
typedef int64_t __fire_i64;
typedef int8_t __fire_u8;
typedef int16_t __fire_u16;
typedef int32_t __fire_u32;
typedef int64_t __fire_u64;
typedef std::string __fire_string;
#define __fire_vec std::vector
#define __fire_map std::map

template<typename T> void __fire_print(T t);
template<typename T, typename... A> void __fire_print(T t, A... a);
template<typename T> void __fire_println(T t);
template<typename T, typename... A> void __fire_println(T t, A... a);
