define fib (i32 %a, i32 %b) {
start:
        br (n < 2), exit, if.else
if.else:
        detach det, cont
det:
        x = fib(n - 1)
        reattach cont
cont:
        y = fib(n - 2)
        sync
        br exit
exit:
        rv = phi([n,entry], [add, join])
        return rv
}


///////////////////////////////
fn fib (_1: i32) -> i32 {
    bb0: {
        _2 = Lt(_1, 2);
        switchInt(_2) -> [false: bb1, otherwise: bb4]
    }
    bb1: { kj_detach -> [det: bb2, continue: bb3]; }
    bb2 : {
        _3 = fib(n - 1);
        kj_reattach -> bb2;
    }
    bb3 : {
        _4 = fib(n - 2);
        kj_sync;
        _0 = _4 + _5;
        goto -> bb5;
    }
    bb4: {
        _0 = _1;
        goto -> bb5;
    }
    bb5: { return; }
}

///////////////////////////////
fn fib(n: i32) -> i32 {
    if n < 2 { n }
    let x = kj_spawn fib(n-1);
    let y = fib(n-2);
    kj_sync;
    x + y
}

//////////////////////////////
fn fib(n: i32) -> i32 {
    if n < 2 { n }
    let x = kj_spawn fib(n - 1);
    let y = fib(n - 2);
    kj_sync;
    x + y
}













