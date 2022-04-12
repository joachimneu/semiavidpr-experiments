#! /bin/bash -ve

for RATE in "33"; do
    cat data-experiments3-RAW-${RATE}.txt | egrep -v "^#" > data-experiments3-rate${RATE}.txt

    cat data-experiments3-rate${RATE}.txt | grep "### columns" | cut -d"#" -f1 > data-experiments3-rate${RATE}-bn254.txt
    cat data-experiments3-rate${RATE}.txt | grep "bn254" >> data-experiments3-rate${RATE}-bn254.txt

    cat data-experiments3-rate${RATE}.txt | grep "### columns" | cut -d"#" -f1 > data-experiments3-rate${RATE}-bls12-381.txt
    cat data-experiments3-rate${RATE}.txt | grep "bls12-381" >> data-experiments3-rate${RATE}-bls12-381.txt
done

echo "All results formatted!"
