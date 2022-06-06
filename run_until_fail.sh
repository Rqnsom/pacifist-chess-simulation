#!/bin/bash

ret=0
i=0
games_limit=3

while [ $ret -eq 0 ] && [ $i -lt $games_limit ]
do
    ./target/debug/pacifist-chess-simulation -w=Random -b=Random -t=1

    ret=$?
    ((i++))
done

exit 0
