#!/bin/bash



for x in `seq -0.01 0.001 0.01`
    do
        python input/ptfe/move.py $x
        cargo run --bin ptfe setup.json $x

    done
