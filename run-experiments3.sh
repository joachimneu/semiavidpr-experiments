#! /bin/bash -ve

cargo build
cargo build --release

for RATE in "33"; do
    echo "args_n args_k args_L args_iterations args_curve  net_file_size_bytes  runtime_setup_seconds runtime_file_generation_seconds  runtime_all_column_commitments_seconds runtime_per_column_commitment_seconds runtime_all_row_encodings_seconds runtime_per_row_encoding_seconds runtime_all_chunk_verifications_seconds runtime_per_chunk_verification_seconds  runtime_all_downloaded_chunk_verifications_seconds runtime_per_downloaded_chunk_verification_seconds runtime_all_row_decodings_seconds runtime_per_row_decoding_seconds runtime_prepare_decoding_seconds  num_column_commitments num_row_encodings num_chunk_verifications num_downloaded_chunk_verifications num_row_decodings  size_file_uncoded_bytes size_column_commitments_bytes size_file_coded_bytes  scenario_disperse_runtime_client_seconds scenario_disperse_runtime_node_seconds scenario_disperse_communication_bytes scenario_disperse_storage_bytes  scenario_retrieve_runtime_client_seconds  scenario_sampling_num_openings scenario_sampling_runtime_prover_seconds scenario_sampling_runtime_verifier_seconds scenario_sampling_runtime_proof_size_bytes   ### columns" > data-experiments3-RAW-${RATE}.txt
done


ITERATIONS=5
EXPERIMENTBIN="./target/release/semiavidpr-experiments"
export RAYON_NUM_THREADS=1
export RUST_BACKTRACE=1

for CURVE in "bls12-381" "bn254"; do
    # k = n*rate (rounded to nearest integer)
    # rate: 1/3 = ~33%
    ${EXPERIMENTBIN} 32 11 `echo "2048*32" | bc` --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments3-RAW-33.txt
    ${EXPERIMENTBIN} 64 21 `echo "2048*16" | bc` --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments3-RAW-33.txt
    ${EXPERIMENTBIN} 128 43 `echo "2048*8" | bc` --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments3-RAW-33.txt
    ${EXPERIMENTBIN} 256 85 `echo "2048*4" | bc` --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments3-RAW-33.txt
    ${EXPERIMENTBIN} 512 171 `echo "2048*2" | bc` --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments3-RAW-33.txt
    ${EXPERIMENTBIN} 1024 341 `echo "2048*1" | bc` --iterations ${ITERATIONS} --curve ${CURVE} | tee -a data-experiments3-RAW-33.txt
done

echo "All experiments run!"
