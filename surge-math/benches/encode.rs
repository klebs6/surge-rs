use surge_math::*;

use std::time::Instant;

fn bench_encode_mid_side(c: &mut Criterion) {
    let nquads = 4096;

    let mut l = Vec::with_capacity(nquads * 4);
    let mut r = Vec::with_capacity(nquads * 4);
    let mut m = Vec::with_capacity(nquads * 4);
    let mut s = Vec::with_capacity(nquads * 4);

    unsafe {
        l.set_len(nquads * 4);
        r.set_len(nquads * 4);
        m.set_len(nquads * 4);
        s.set_len(nquads * 4);
    }

    let mut rng = rand::thread_rng();

    for i in 0..nquads * 4 {
        l[i] = rng.gen_range(-1.0, 1.0);
        r[i] = rng.gen_range(-1.0, 1.0);
    }

    let mut group = c.benchmark_group("mid-side-encode");
    group.throughput(Throughput::Elements(nquads as u64 * 4));
    group.bench_function("encode_mid_side", |b| {
        b.iter(|| unsafe {
            encode_mid_side(l.as_mut_ptr(), r.as_mut_ptr(), m.as_mut_ptr(), s.as_mut_ptr(), nquads)
        })
    });
    group.finish();
}

fn bench_decode_mid_side(c: &mut Criterion) {
    let nquads = 4096;

    let mut m = Vec::with_capacity(nquads * 4);
    let mut s = Vec::with_capacity(nquads * 4);
    let mut l = Vec::with_capacity(nquads * 4);
    let mut r = Vec::with_capacity(nquads * 4);

    unsafe {
        l.set_len(nquads * 4);
        r.set_len(nquads * 4);
        m.set_len(nquads * 4);
        s.set_len(nquads * 4);
    }

    let mut rng = rand::thread_rng();

    for i in 0..nquads * 4 {
        m[i] = rng.gen_range(-1.0, 1.0);
        s[i] = rng.gen_range(-1.0, 1.0);
    }

    let mut group = c.benchmark_group("mid-side-decode");
    group.throughput(Throughput::Elements(nquads as u64 * 4));
    group.bench_function("decode_mid_side", |b| {
        b.iter(|| unsafe {
            decode_mid_side(m.as_mut_ptr(), s.as_mut_ptr(), l.as_mut_ptr(), r.as_mut_ptr(), nquads)
        })
    });
    group.finish();
}
