#!/bin/bash
# Purpose of the script is to gather statistics for these random games

ret=0
i=0
games_limit=100

white=Pacifist
black=Agressive
results=/tmp/"$white"vs"$black".txt

while [ $ret -eq 0 ] && [ $i -lt $games_limit ]
do
    # Make sure project is built before running the script!
    ../target/debug/pacifist-chess-simulation -w=$white -b=$black -t=1 >> $results

    echo "run $i"

    ret=$?
    ((i++))
done

final_results=/tmp/final_"$white"_vs_"$black".txt
grep Gamestatus $results > $final_results

echo "$white vs $black"

# Results!
cat $final_results | sort | uniq -c

# Cleanup
rm $results -rf
rm -rf $final_results

exit 0
