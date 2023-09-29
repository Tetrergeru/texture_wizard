float my_perlin(vec2 pos) {
    float p = perlin(pos);
    p =  clamp((p - 0.5) * 3 + 0.5, 0, 1);
    return p;
}

float fractal_perlin(vec2 pos, int iters) {
    float res = 0;
    float scale = 1;
    float sum = 0;

    for (int i = 0; i < iters; i++) {
        HashSeed = i;
        
        res += my_perlin(pos * scale) / scale;
        sum += 1.0 / scale;

        scale *= 2.0;
    }
    
    res = res / sum;
    return clamp(res, 0, 1);
}
