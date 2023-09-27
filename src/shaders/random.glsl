#include "hash.glsl"

#define RANDOM_MOD 10000

float u_random(int hash) {
    return mod(abs(hash), RANDOM_MOD) / RANDOM_MOD;
}

float i_random(int hash) {
    return mod(hash, RANDOM_MOD) / RANDOM_MOD;
}