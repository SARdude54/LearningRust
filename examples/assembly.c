#include <stdint.h>

__attribute__((noinline))
uint32_t set_bit(uint32_t value, uint32_t bit)
{
    return value | (1u << bit);
}