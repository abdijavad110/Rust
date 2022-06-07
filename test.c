void normalize(double *out, double *in, int n) {
    for (int i=0; i<n; i ++)
        out[i] = cilk_spawn in[i] / norm(in, n);
}


void normalize(double *out, double *in, int n) {
    for (int i=0; i<n; i ++)
        int* close[3] = {n, in, out};
        __cilk_rts_for(cilk_for_helper, close, 0, n);
}
void cilk_for_helper (int start, int end) {
    /* reading parameters fron close */
    out[i] = cilk_spawn in[i] / norm(in, n);
}


int fib(int n) {
    if (n < 2) return n;
    int x = cilk_spawn fib(n-1);
    int y = fib(n-2);
    cilk_sync;
    return x + y;
}
