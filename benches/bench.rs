#[macro_use]
extern crate bencher;
use bencher::{Bencher, black_box};

#[path = "../src/gb/cpu/reg/union.rs"]
mod union_unsafe;
use union_unsafe::SafeU16Union as UnionUnsafe;
#[path = "../src/gb/cpu/reg/union_safe.rs"]
mod union_safe;
use union_safe::SafeU16Union as UnionSafe;

macro_rules! bench {
    ($u: expr) => {
        for _ in 0..100000000u64 {
            $u.set_inner_a(0x32);
            $u.set_inner_b(0x64);
            $u.set_union_value(0x5634);
            black_box($u.get_union_value());
            black_box($u.get_inner_a() + $u.get_inner_b());
        }
    };
}
fn bench_unsafe(bench: &mut Bencher) {
    let mut u = UnionUnsafe::new(0x1234);
    bench.iter(|| {
        bench!(u);
    });
}

fn bench_safe(bench: &mut Bencher) {
    let mut u = UnionSafe::new(0x1234);
    bench.iter(|| {
        bench!(u);
    });
}

benchmark_group!(benches, bench_unsafe, bench_safe);
benchmark_main!(benches);