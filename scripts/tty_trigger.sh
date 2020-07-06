#!/usr/bin/env bash

# Generates pulses at the TX pin of ttyUSB0

while true; do
    echo UUUU > /dev/ttyUSB0
    sleep 0.015625
done
