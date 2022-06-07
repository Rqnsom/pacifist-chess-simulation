#!/bin/bash
# Purpose of the script is to find rare hidden gemmy bugs!

ret=0
i=0
games_limit=10000

white=Pacifist
black=Aggressive

while [ $ret -eq 0 ] && [ $i -lt $games_limit ]
do
    # Make sure project is built before running the script!
    ../target/debug/pacifist-chess-simulation -w=$white -b=$black -t=1

    echo "run $i"

    ret=$?
    ((i++))
done

exit 0
