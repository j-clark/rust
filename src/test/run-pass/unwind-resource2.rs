// xfail-win32
use std;
import task;
import comm;

resource complainer(c: @int) {
}

fn f() {
    task::unsupervise();
    let c <- complainer(@0);
    fail;
}

fn main() {
    task::spawn {|| f(); };
}