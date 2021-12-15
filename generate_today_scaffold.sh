#! /usr/bin/env bash

# USAGE: ./generate_today_scaffold.sh [DAY]
# EXAMPLES
#    # generate today's
# ./generate_today_scaffold.sh
#    # generate dec 9th's
# ./generate_today_scaffold.sh 9

generate_crate() {
    CRATE_NAME=crates/day$(printf "%02d" $1)
    cargo new $CRATE_NAME --bin
}

generate_readme() {
    CRATE_NAME=crates/day$(printf "%02d" $1)
    echo "copy contents of https://adventofcode.com/2021/day/$1 , then press enter"
    read -n 1
    pbpaste > $CRATE_NAME/README.md
}

generate_puzzle_input() {
    if [ -z "$AOC_SESSION_TOKEN" ]
    then
        echo export AOC_SESSION_TOKEN="session cookie"
    else
        curl -b session=$AOC_SESSION_TOKEN https://adventofcode.com/2021/day/$1/input > data/$1
    fi
}

TODAY=$(date "+%d")
GENERATE_DATE="${1:-$TODAY}"

generate_crate $GENERATE_DATE
generate_puzzle_input $GENERATE_DATE
generate_readme $GENERATE_DATE
