extern crate semiavidpr;
use semiavidpr::{SemiAvidPr};

use ark_ec::{PairingEngine};
use ark_bls12_381::{Bls12_381};
use ark_bn254::{Bn254};

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};


fn bench_disperse_compute_column_commitments<E: PairingEngine + CurveName>(c: &mut Criterion) {
    let mut rng = ark_std::rand::thread_rng();

    let mut group = c.benchmark_group(format!("{}/disperse_compute_column_commitments", E::curve_name()));
    group.sample_size(10);

    let n = 1024;
    #[allow(non_snake_case)]
    for L in vec![256, 512, 1024, 2048] {
        let k = n / 3;
        let scheme = SemiAvidPr::<E>::setup(&mut rng, n, k, L);
        
        group.throughput(Throughput::Bytes(scheme.get_filesize_in_bytes()));
        group.bench_with_input(format!("n={} L={}", n, L), &(n, L), |b, (_n, _L)| {
            let file_uncoded = scheme.generate_random_file(&mut rng);
            b.iter(|| {
                black_box(scheme.disperse_compute_column_commitments(&file_uncoded));
            })
        });
    }

    group.finish();
}


fn bench_disperse_encode_rows<E: PairingEngine + CurveName>(c: &mut Criterion) {
    let mut rng = ark_std::rand::thread_rng();

    let mut group = c.benchmark_group(format!("{}/disperse_encode_rows", E::curve_name()));
    group.sample_size(10);

    for n in vec![256, 512, 1024, 2048] {
        #[allow(non_snake_case)]
        for L in vec![256, 512, 1024, 2048] {
            let k = n / 3;
            let scheme = SemiAvidPr::<E>::setup(&mut rng, n, k, L);
            
            group.throughput(Throughput::Bytes(scheme.get_filesize_in_bytes()));
            group.bench_with_input(format!("n={} L={}", n, L), &(n, L), |b, (_n, _L)| {
                let file_uncoded = scheme.generate_random_file(&mut rng);
                b.iter(|| {
                    black_box(scheme.disperse_encode_rows(&file_uncoded));
                })
            });
        }
    }

    group.finish();
}


pub trait CurveName {
    fn curve_name() -> &'static str;
}

impl CurveName for Bls12_381 {
    fn curve_name() -> &'static str {
        "Bls12_381"
    }
}

impl CurveName for Bn254 {
    fn curve_name() -> &'static str {
        "Bn254"
    }
}


fn bench_bls12_381(c: &mut Criterion) {
    bench_disperse_compute_column_commitments::<Bls12_381>(c);
    bench_disperse_encode_rows::<Bls12_381>(c);
}

fn bench_bn254(c: &mut Criterion) {
    bench_disperse_compute_column_commitments::<Bn254>(c);
    bench_disperse_encode_rows::<Bn254>(c);
}


criterion_group!(benches, bench_bls12_381, bench_bn254);
criterion_main!(benches);
