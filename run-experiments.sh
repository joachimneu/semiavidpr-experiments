#! /bin/bash -ve

cargo build
cargo build --release

for RATE in "25" "33" "45"; do
    echo "args_n args_k args_L args_iterations args_curve  net_file_size_bytes  runtime_setup_seconds runtime_file_generation_seconds  runtime_all_column_commitments_seconds runtime_per_column_commitment_seconds runtime_all_row_encodings_seconds runtime_per_row_encoding_seconds runtime_all_chunk_verifications_seconds runtime_per_chunk_verification_seconds  runtime_all_downloaded_chunk_verifications_seconds runtime_per_downloaded_chunk_verification_seconds runtime_all_row_decodings_seconds runtime_per_row_decoding_seconds runtime_prepare_decoding_seconds  num_column_commitments num_row_encodings num_chunk_verifications num_downloaded_chunk_verifications num_row_decodings  size_file_uncoded_bytes size_column_commitments_bytes size_file_coded_bytes  scenario_disperse_runtime_client_seconds scenario_disperse_runtime_node_seconds scenario_disperse_communication_bytes scenario_disperse_storage_bytes  scenario_retrieve_runtime_client_seconds  scenario_sampling_num_openings scenario_sampling_runtime_prover_seconds scenario_sampling_runtime_verifier_seconds scenario_sampling_runtime_proof_size_bytes   ### columns" > data-experiments-RAW-${RATE}.txt
done


ITERATIONS=1
EXPERIMENTBIN="./target/release/semiavidpr-experiments"
export RAYON_NUM_THREADS=1
export RUST_BACKTRACE=1

for CURVE in "bls12-381" "bn254"; do
    for L in 512 1024 2048 4096; do
        # k = n*rate (rounded to nearest integer)
        # rate: 1/3 = ~33%
        ${EXPERIMENTBIN} 128 43 ${L} --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments-RAW-33.txt
        ${EXPERIMENTBIN} 256 85 ${L} --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments-RAW-33.txt
        ${EXPERIMENTBIN} 512 171 ${L} --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments-RAW-33.txt
        ${EXPERIMENTBIN} 1024 341 ${L} --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments-RAW-33.txt
        # rate: 25%
        ${EXPERIMENTBIN} 128 32 ${L} --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments-RAW-25.txt
        ${EXPERIMENTBIN} 256 64 ${L} --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments-RAW-25.txt
        ${EXPERIMENTBIN} 512 128 ${L} --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments-RAW-25.txt
        ${EXPERIMENTBIN} 1024 256 ${L} --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments-RAW-25.txt
        # rate: 45%
        ${EXPERIMENTBIN} 128 58 ${L} --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments-RAW-45.txt
        ${EXPERIMENTBIN} 256 115 ${L} --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments-RAW-45.txt
        ${EXPERIMENTBIN} 512 230 ${L} --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments-RAW-45.txt
        ${EXPERIMENTBIN} 1024 461 ${L} --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments-RAW-45.txt
    done
done

echo "All experiments run!"
