
#!/bin/bash



for x in `seq -0.01 0.001 0.01`
    do
        for value in `seq 1 1 10`
            do
                python scripts/ptfe/slab_move.py $x
                cargo run --bin ptfe --release parameters.json
        done
done
