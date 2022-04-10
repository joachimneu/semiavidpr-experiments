#! /bin/bash -ve

for RATE in "25" "33" "45"; do
    cat data-experiments-RAW-${RATE}.txt | egrep -v "^#" > data-experiments-rate${RATE}.txt

    cat data-experiments-rate${RATE}.txt | grep "### columns" | cut -d"#" -f1 > data-experiments-rate${RATE}-bn254.txt
    cat data-experiments-rate${RATE}.txt | grep "bn254" >> data-experiments-rate${RATE}-bn254.txt

    cat data-experiments-rate${RATE}.txt | grep "### columns" | cut -d"#" -f1 > data-experiments-rate${RATE}-bls12-381.txt
    cat data-experiments-rate${RATE}.txt | grep "bls12-381" >> data-experiments-rate${RATE}-bls12-381.txt
done

echo "All results formatted!"
