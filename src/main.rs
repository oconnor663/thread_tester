extern crate blake2b_simd;
extern crate crossbeam;

const TOTAL_SIZE: usize = 4_000_000_000;
const THREADS: usize = 8;

fn hash_update(inputs: &[&[u8]]) {
    crossbeam::scope(|scope| {
        for i in 0..THREADS {
            scope.spawn(move || {
                blake2b_simd::blake2b(inputs[4 * i + 0]);
                blake2b_simd::blake2b(inputs[4 * i + 1]);
                blake2b_simd::blake2b(inputs[4 * i + 2]);
                blake2b_simd::blake2b(inputs[4 * i + 3]);
            });
        }
    });
}

fn hash_update4(inputs: &[&[u8]]) {
    crossbeam::scope(|scope| {
        for i in 0..THREADS {
            scope.spawn(move || {
                let mut state0 = blake2b_simd::State::new();
                let mut state1 = blake2b_simd::State::new();
                let mut state2 = blake2b_simd::State::new();
                let mut state3 = blake2b_simd::State::new();
                blake2b_simd::update4(
                    &mut state0,
                    &mut state1,
                    &mut state2,
                    &mut state3,
                    inputs[4 * i + 0],
                    inputs[4 * i + 1],
                    inputs[4 * i + 2],
                    inputs[4 * i + 3],
                );
                blake2b_simd::finalize4(&mut state0, &mut state1, &mut state2, &mut state3);
            });
        }
    });
}

fn millis(d: std::time::Duration) -> f32 {
    (d.as_secs() * 1000 + d.subsec_millis() as u64) as f32
}

fn main() {
    let mut inputs = Vec::new();
    for _ in 0..THREADS * 4 {
        // 1 not 0 to force intializing the pages.
        inputs.push(vec![1u8; TOTAL_SIZE / (THREADS * 4)]);
    }
    let refs: Vec<&[u8]> = inputs.iter().map(|v| &**v).collect();
    println!(
        "total size {} GB",
        refs.len() * refs[0].len() / 1_000_000_000
    );

    let start = std::time::Instant::now();
    hash_update(&refs);
    let dur = std::time::Instant::now() - start;
    println!("hash_update  {:?}", dur);

    let start = std::time::Instant::now();
    hash_update4(&refs);
    let dur2 = std::time::Instant::now() - start;
    println!("hash_update4 {:?}", dur2);

    println!("ratio {:?}", millis(dur) / millis(dur2));
}
