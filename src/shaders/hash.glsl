int HashSeed = 0;

int hash_1(int v) {
    v ^= HashSeed * 216091 >> 2;
    v ^= v * 524287 >> 5;
    v ^= v * 131071 >> 2;
    return v;
}

int hash_2(int x, int y) {
    return hash_1(x ^ hash_1(y));
}

int hash_3(int x, int y, int s) {
    return hash_1(s ^ hash_2(x, y));
}

int hash_4(int x, int y, int z, int s) {
    return hash_1(s ^ hash_3(x, y, z));
}
