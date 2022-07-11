#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define SYMBOL_SIZE 4

/**
 * Reed-Solomon Error Correction encoder module.
 */
#define MAX_LENGTH ((1 << SYMBOL_SIZE) - 1)
