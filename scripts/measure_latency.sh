#!/usr/bin/env bash

sigrok-cli -d fx2lafw --config samplerate=24m --time 10s --channels D0=A5,D2=A6 -o measurement.sr
