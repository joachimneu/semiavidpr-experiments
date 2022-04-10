extern crate semiavidpr;
use semiavidpr::{SemiAvidPr};

use ark_ec::{PairingEngine};
use ark_bls12_381::{Bls12_381};
use ark_bn254::{Bn254};

use rand::{Rng};
use std::time::{Instant};
use clap::{Parser, IntoApp, ErrorKind, ArgEnum};


/// Run Semi-AVID-PR experiments: https://arxiv.org/abs/2111.12323
#[allow(non_snake_case)]
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of coded chunks
    n: usize,

    /// Number of uncoded chunks
    k: usize,

    /// Length of chunk
    L: usize,

    /// Iterations of the dispersal experiment
    #[clap(short, long, default_value_t = 1)]
    iterations: usize,

    /// Pairing-friendly curve to use for experiments
    #[clap(arg_enum, short, long, default_value_t = CurveArg::Bls12_381)]
    curve: CurveArg,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum CurveArg {
    Bls12_381,
    Bn254,
}

impl std::fmt::Display for CurveArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurveArg::Bls12_381 => write!(f, "bls12-381"),
            CurveArg::Bn254 => write!(f, "bn254"),
        }
    }
}


#[derive(Clone, Copy, Debug, Default)]
struct Measurements {
    num_measurements: usize,

    net_file_size_bytes: usize,

    runtime_setup_seconds: f64,
    runtime_file_generation_seconds: f64,

    runtime_all_column_commitments_seconds: f64,
    runtime_per_column_commitment_seconds: f64,
    runtime_all_row_encodings_seconds: f64,
    runtime_per_row_encoding_seconds: f64,
    runtime_all_chunk_verifications_seconds: f64,
    runtime_per_chunk_verification_seconds: f64,

    runtime_all_downloaded_chunk_verifications_seconds: f64,
    runtime_per_downloaded_chunk_verification_seconds: f64,
    runtime_all_row_decodings_seconds: f64,
    runtime_per_row_decoding_seconds: f64,
    runtime_prepare_decoding_seconds: f64,

    num_column_commitments: usize,
    num_row_encodings: usize,
    num_chunk_verifications: usize,
    num_downloaded_chunk_verifications: usize,
    num_row_decodings: usize,

    size_file_uncoded_bytes: usize,
    size_column_commitments_bytes: usize,
    size_file_coded_bytes: usize,

    scenario_disperse_runtime_client_seconds: f64,
    scenario_disperse_runtime_node_seconds: f64,
    scenario_disperse_communication_bytes: usize,
    scenario_disperse_storage_bytes: usize,

    scenario_retrieve_runtime_client_seconds: f64,

    scenario_sampling_num_openings: usize,
    scenario_sampling_runtime_prover_seconds: f64,
    scenario_sampling_runtime_verifier_seconds: f64,
    scenario_sampling_runtime_proof_size_bytes: usize,
}

impl core::ops::Add for Measurements {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            num_measurements: self.num_measurements + other.num_measurements,

            net_file_size_bytes: self.net_file_size_bytes + other.net_file_size_bytes,

            runtime_setup_seconds: self.runtime_setup_seconds + other.runtime_setup_seconds,
            runtime_file_generation_seconds: self.runtime_file_generation_seconds + other.runtime_file_generation_seconds,
        
            runtime_all_column_commitments_seconds: self.runtime_all_column_commitments_seconds + other.runtime_all_column_commitments_seconds,
            runtime_per_column_commitment_seconds: self.runtime_per_column_commitment_seconds + other.runtime_per_column_commitment_seconds,
            runtime_all_row_encodings_seconds: self.runtime_all_row_encodings_seconds + other.runtime_all_row_encodings_seconds,
            runtime_per_row_encoding_seconds: self.runtime_per_row_encoding_seconds + other.runtime_per_row_encoding_seconds,
            runtime_all_chunk_verifications_seconds: self.runtime_all_chunk_verifications_seconds + other.runtime_all_chunk_verifications_seconds,
            runtime_per_chunk_verification_seconds: self.runtime_per_chunk_verification_seconds + other.runtime_per_chunk_verification_seconds,
            
            runtime_all_downloaded_chunk_verifications_seconds: self.runtime_all_downloaded_chunk_verifications_seconds + other.runtime_all_downloaded_chunk_verifications_seconds,
            runtime_per_downloaded_chunk_verification_seconds: self.runtime_per_downloaded_chunk_verification_seconds + other.runtime_per_downloaded_chunk_verification_seconds,
            runtime_all_row_decodings_seconds: self.runtime_all_row_decodings_seconds + other.runtime_all_row_decodings_seconds,
            runtime_per_row_decoding_seconds: self.runtime_per_row_decoding_seconds + other.runtime_per_row_decoding_seconds,
            runtime_prepare_decoding_seconds: self.runtime_prepare_decoding_seconds + other.runtime_prepare_decoding_seconds,

            num_column_commitments: self.num_column_commitments + other.num_column_commitments,
            num_row_encodings: self.num_row_encodings + other.num_row_encodings,
            num_chunk_verifications: self.num_chunk_verifications + other.num_chunk_verifications,
            num_downloaded_chunk_verifications: self.num_downloaded_chunk_verifications + other.num_downloaded_chunk_verifications,
            num_row_decodings: self.num_row_decodings + other.num_row_decodings,

            size_file_uncoded_bytes: self.size_file_uncoded_bytes + other.size_file_uncoded_bytes,
            size_column_commitments_bytes: self.size_column_commitments_bytes + other.size_column_commitments_bytes,
            size_file_coded_bytes: self.size_file_coded_bytes + other.size_file_coded_bytes,
        
            scenario_disperse_runtime_client_seconds: self.scenario_disperse_runtime_client_seconds + other.scenario_disperse_runtime_client_seconds,
            scenario_disperse_runtime_node_seconds: self.scenario_disperse_runtime_node_seconds + other.scenario_disperse_runtime_node_seconds,
            scenario_disperse_communication_bytes: self.scenario_disperse_communication_bytes + other.scenario_disperse_communication_bytes,
            scenario_disperse_storage_bytes: self.scenario_disperse_storage_bytes + other.scenario_disperse_storage_bytes,

            scenario_retrieve_runtime_client_seconds: self.scenario_retrieve_runtime_client_seconds + other.scenario_retrieve_runtime_client_seconds,

            scenario_sampling_num_openings: self.scenario_sampling_num_openings + other.scenario_sampling_num_openings,
            scenario_sampling_runtime_prover_seconds: self.scenario_sampling_runtime_prover_seconds + other.scenario_sampling_runtime_prover_seconds,
            scenario_sampling_runtime_verifier_seconds: self.scenario_sampling_runtime_verifier_seconds + other.scenario_sampling_runtime_verifier_seconds,
            scenario_sampling_runtime_proof_size_bytes: self.scenario_sampling_runtime_proof_size_bytes + other.scenario_sampling_runtime_proof_size_bytes,
        }
    }
}

impl core::ops::Div<usize> for Measurements {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            num_measurements: self.num_measurements / rhs,

            net_file_size_bytes: self.net_file_size_bytes / rhs,

            runtime_setup_seconds: self.runtime_setup_seconds / (rhs as f64),
            runtime_file_generation_seconds: self.runtime_file_generation_seconds / (rhs as f64),
        
            runtime_all_column_commitments_seconds: self.runtime_all_column_commitments_seconds / (rhs as f64),
            runtime_per_column_commitment_seconds: self.runtime_per_column_commitment_seconds / (rhs as f64),
            runtime_all_row_encodings_seconds: self.runtime_all_row_encodings_seconds / (rhs as f64),
            runtime_per_row_encoding_seconds: self.runtime_per_row_encoding_seconds / (rhs as f64),
            runtime_all_chunk_verifications_seconds: self.runtime_all_chunk_verifications_seconds / (rhs as f64),
            runtime_per_chunk_verification_seconds: self.runtime_per_chunk_verification_seconds / (rhs as f64),

            runtime_all_downloaded_chunk_verifications_seconds: self.runtime_all_downloaded_chunk_verifications_seconds / (rhs as f64),
            runtime_per_downloaded_chunk_verification_seconds: self.runtime_per_downloaded_chunk_verification_seconds / (rhs as f64),
            runtime_all_row_decodings_seconds: self.runtime_all_row_decodings_seconds / (rhs as f64),
            runtime_per_row_decoding_seconds: self.runtime_per_row_decoding_seconds / (rhs as f64),
            runtime_prepare_decoding_seconds: self.runtime_prepare_decoding_seconds / (rhs as f64),

            num_column_commitments: self.num_column_commitments / rhs,
            num_row_encodings: self.num_row_encodings / rhs,
            num_chunk_verifications: self.num_chunk_verifications / rhs,
            num_downloaded_chunk_verifications: self.num_downloaded_chunk_verifications / rhs,
            num_row_decodings: self.num_row_decodings / rhs,

            size_file_uncoded_bytes: self.size_file_uncoded_bytes / rhs,
            size_column_commitments_bytes: self.size_column_commitments_bytes / rhs,
            size_file_coded_bytes: self.size_file_coded_bytes / rhs,
        
            scenario_disperse_runtime_client_seconds: self.scenario_disperse_runtime_client_seconds / (rhs as f64),
            scenario_disperse_runtime_node_seconds: self.scenario_disperse_runtime_node_seconds / (rhs as f64),
            scenario_disperse_communication_bytes: self.scenario_disperse_communication_bytes / rhs,
            scenario_disperse_storage_bytes: self.scenario_disperse_storage_bytes / rhs,

            scenario_retrieve_runtime_client_seconds: self.scenario_retrieve_runtime_client_seconds / (rhs as f64),

            scenario_sampling_num_openings: self.scenario_sampling_num_openings / rhs,
            scenario_sampling_runtime_prover_seconds: self.scenario_sampling_runtime_prover_seconds / (rhs as f64),
            scenario_sampling_runtime_verifier_seconds: self.scenario_sampling_runtime_verifier_seconds / (rhs as f64),
            scenario_sampling_runtime_proof_size_bytes: self.scenario_sampling_runtime_proof_size_bytes / rhs,
        }
    }
}


// // https://stackoverflow.com/a/42893060
// pub fn black_box<T>(dummy: T) -> T {
//     unsafe { asm!("" : : "r"(&dummy)) }
//     dummy
// }

#[allow(non_snake_case)]
fn run_dispersal_experiment<R: Rng + ?Sized, E: PairingEngine>(mut rng: &mut R, n: usize, k: usize, L: usize) -> Measurements {
    let mut measurements = Measurements::default();
    measurements.num_measurements = 1;


    // DISPERSAL (OPENING CHUNKS)

    // setup

    let timer_begin = Instant::now();
    let scheme = SemiAvidPr::<E>::setup(&mut rng, n, k, L);
    measurements.runtime_setup_seconds = timer_begin.elapsed().as_secs_f64();

    measurements.net_file_size_bytes = scheme.get_filesize_in_bytes() as usize;
    measurements.num_column_commitments = scheme.get_num_column_commitments();
    measurements.num_row_encodings = scheme.get_num_row_encodings();
    measurements.num_chunk_verifications = scheme.get_num_chunk_verifications();
    measurements.num_downloaded_chunk_verifications = scheme.get_num_downloaded_chunk_verifications();
    measurements.num_row_decodings = scheme.get_num_row_decodings();

    // random file generation

    let timer_begin = Instant::now();
    let file_uncoded = scheme.generate_random_file(&mut rng);
    measurements.runtime_file_generation_seconds = timer_begin.elapsed().as_secs_f64();

    // commit to file

    let timer_begin = Instant::now();
    let column_commitments = scheme.disperse_compute_column_commitments(&file_uncoded);
    measurements.runtime_all_column_commitments_seconds = timer_begin.elapsed().as_secs_f64();
    measurements.runtime_per_column_commitment_seconds = measurements.runtime_all_column_commitments_seconds / (measurements.num_column_commitments as f64);

    // encode rows

    let timer_begin = Instant::now();
    let file_coded = scheme.disperse_encode_rows(&file_uncoded);
    measurements.runtime_all_row_encodings_seconds = timer_begin.elapsed().as_secs_f64();
    measurements.runtime_per_row_encoding_seconds = measurements.runtime_all_row_encodings_seconds / (measurements.num_row_encodings as f64);

    // verify chunks

    let timer_begin = Instant::now();
    if !scheme.disperse_verify_chunks(&column_commitments, &file_coded) {
        panic!("Verification of chunks should not fail!");
    }
    measurements.runtime_all_chunk_verifications_seconds = timer_begin.elapsed().as_secs_f64();
    measurements.runtime_per_chunk_verification_seconds = measurements.runtime_all_chunk_verifications_seconds / (measurements.num_chunk_verifications as f64);

    // black_box(file_uncoded);
    // black_box(column_commitments);
    // black_box(file_coded);


    // BOOKKEEPING

    measurements.size_file_uncoded_bytes = std::mem::size_of::<E::Fr>() * k * L;
    measurements.size_column_commitments_bytes = std::mem::size_of::<E::G1Affine>() * k;
    measurements.size_file_coded_bytes = std::mem::size_of::<E::Fr>() * n * L;

    measurements.scenario_disperse_runtime_client_seconds = measurements.runtime_all_column_commitments_seconds + measurements.runtime_all_row_encodings_seconds;
    measurements.scenario_disperse_runtime_node_seconds = measurements.runtime_per_chunk_verification_seconds;
    measurements.scenario_disperse_communication_bytes = n * measurements.size_column_commitments_bytes + measurements.size_file_coded_bytes;
    measurements.scenario_disperse_storage_bytes = n * measurements.size_column_commitments_bytes + measurements.size_file_coded_bytes;


    // RETRIEVAL

    // retrieve chunks

    let idxs_download_nodes: Vec<usize> = (k..2*k).collect();   // an arbitrary set of storage nodes to download from
    let file_coded_downloaded = scheme.retrieve_download_chunks(&file_coded, &idxs_download_nodes);

    // verify chunks

    let timer_begin = Instant::now();
    if !scheme.retrieve_verify_chunks(&column_commitments, &file_coded_downloaded, &idxs_download_nodes) {
        panic!("Verification of chunks should not fail!");
    }
    measurements.runtime_all_downloaded_chunk_verifications_seconds = timer_begin.elapsed().as_secs_f64();
    measurements.runtime_per_downloaded_chunk_verification_seconds = measurements.runtime_all_downloaded_chunk_verifications_seconds / (measurements.num_downloaded_chunk_verifications as f64);

    // decode preparations

    let timer_begin = Instant::now();
    let decoder_aux = scheme.retrieve_prepare_decoding(&idxs_download_nodes);
    measurements.runtime_prepare_decoding_seconds = timer_begin.elapsed().as_secs_f64();

    // decode chunks

    let timer_begin = Instant::now();
    let file_uncoded_downloaded = scheme.retrieve_decode_rows(&file_coded_downloaded, &decoder_aux);
    measurements.runtime_all_row_decodings_seconds = timer_begin.elapsed().as_secs_f64();
    measurements.runtime_per_row_decoding_seconds = measurements.runtime_all_row_decodings_seconds / (measurements.num_row_decodings as f64);

    // black_box(file_uncoded_downloaded);

    for row in 0..L {
        for col in 0..k {
            assert!(file_uncoded[row][col] == file_uncoded_downloaded[row][col]);
        }
    }


    // BOOKKEEPING

    measurements.scenario_retrieve_runtime_client_seconds = measurements.runtime_all_downloaded_chunk_verifications_seconds + measurements.runtime_prepare_decoding_seconds + measurements.runtime_all_row_decodings_seconds;


    // SAMPLING (OPENING ENTRIES)

    measurements.scenario_sampling_num_openings = std::cmp::min(k, L);
    for idx in 0..measurements.scenario_sampling_num_openings {
        let timer_begin = Instant::now();
        let opening = scheme.sampling_open_entry(&column_commitments, &file_uncoded, idx, idx);
        measurements.scenario_sampling_runtime_prover_seconds += timer_begin.elapsed().as_secs_f64();

        measurements.scenario_sampling_runtime_proof_size_bytes = 0;
        measurements.scenario_sampling_runtime_proof_size_bytes += std::mem::size_of::<E::Fr>(); // value
        measurements.scenario_sampling_runtime_proof_size_bytes += std::mem::size_of::<usize>(); // row
        measurements.scenario_sampling_runtime_proof_size_bytes += std::mem::size_of::<usize>(); // col
        measurements.scenario_sampling_runtime_proof_size_bytes += measurements.size_column_commitments_bytes; // column_commitments
        measurements.scenario_sampling_runtime_proof_size_bytes += std::mem::size_of::<E::G1Affine>(); // KZG proof

        let timer_begin = Instant::now();
        if !scheme.sampling_verify_entry(opening) {
            panic!("Verification of openings should not fail!");
        }
        measurements.scenario_sampling_runtime_verifier_seconds += timer_begin.elapsed().as_secs_f64();
    }
    measurements.scenario_sampling_runtime_prover_seconds /= measurements.scenario_sampling_num_openings as f64;
    measurements.scenario_sampling_runtime_verifier_seconds /= measurements.scenario_sampling_num_openings as f64;

    // black_box(openings);


    measurements
}


fn main() {
    let mut rng = ark_std::rand::thread_rng();

    let args = Args::parse();
    println!("# {:?}", args);

    if !(args.n.is_power_of_two()) || !(args.L.is_power_of_two()) {
        let mut app = Args::into_app();
        app.error(
            ErrorKind::InvalidValue,
            "N and L have to be a power of 2",
        )
        .exit();
    }

    assert!(args.n.is_power_of_two());
    assert!(args.L.is_power_of_two());

    let mut measurement = Measurements::default();
    for _iter in 0..args.iterations {
        measurement = measurement + match args.curve {
            CurveArg::Bls12_381 => run_dispersal_experiment::<_, Bls12_381>(&mut rng, args.n, args.k, args.L),
            CurveArg::Bn254 => run_dispersal_experiment::<_, Bn254>(&mut rng, args.n, args.k, args.L),
        };
    }

    assert_eq!(args.iterations, measurement.num_measurements);
    measurement = measurement / measurement.num_measurements;
    println!("# {:?}", measurement);

    println!("{} {} {} {} {}  \
        {}  \
        {:.6} {:.6}  \
        {:.6} {:.6} {:.6} {:.6} {:.6} {:.6}  \
        {:.6} {:.6} {:.6} {:.6} {:.6}  \
        {} {} {} {} {}  \
        {} {} {}  \
        {:.6} {:.6} {} {}  \
        {:.6}  \
        {} {:.6} {:.6} {}",
        args.n, args.k, args.L, args.iterations, args.curve,

        measurement.net_file_size_bytes,

        measurement.runtime_setup_seconds,
        measurement.runtime_file_generation_seconds,

        measurement.runtime_all_column_commitments_seconds,
        measurement.runtime_per_column_commitment_seconds,
        measurement.runtime_all_row_encodings_seconds,
        measurement.runtime_per_row_encoding_seconds,
        measurement.runtime_all_chunk_verifications_seconds,
        measurement.runtime_per_chunk_verification_seconds,

        measurement.runtime_all_downloaded_chunk_verifications_seconds,
        measurement.runtime_per_downloaded_chunk_verification_seconds,
        measurement.runtime_all_row_decodings_seconds,
        measurement.runtime_per_row_decoding_seconds,
        measurement.runtime_prepare_decoding_seconds,

        measurement.num_column_commitments,
        measurement.num_row_encodings,
        measurement.num_chunk_verifications,
        measurement.num_downloaded_chunk_verifications,
        measurement.num_row_decodings,

        measurement.size_file_uncoded_bytes,
        measurement.size_column_commitments_bytes,
        measurement.size_file_coded_bytes,
    
        measurement.scenario_disperse_runtime_client_seconds,
        measurement.scenario_disperse_runtime_node_seconds,
        measurement.scenario_disperse_communication_bytes,
        measurement.scenario_disperse_storage_bytes,

        measurement.scenario_retrieve_runtime_client_seconds,

        measurement.scenario_sampling_num_openings,
        measurement.scenario_sampling_runtime_prover_seconds,
        measurement.scenario_sampling_runtime_verifier_seconds,
        measurement.scenario_sampling_runtime_proof_size_bytes,
    );
}
