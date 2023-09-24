
int hash(int v) {
    v ^= HashSeed * 216091 >> 2;
    v ^= v * 524287 >> 5;
    v ^= v * 131071 >> 2;
    return v;
}