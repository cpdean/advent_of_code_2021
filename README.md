organized as a crate per day for now

useful commands:

```

cargo new --bin crates/day03
pbpaste > data/3
cargo test
fd rs | entr cargo test
cargo run --bin day03

```

Download a day's puzzle input:

find your session token by looking for the 'session' cookie in your web browser dev tools, then:

```
export AOC_SESSION_TOKEN=token_you_found

# pull one thing
curl -b session=$AOC_SESSION_TOKEN https://adventofcode.com/2021/day/9/input > data/9

# pull the days you need to catch up on
for day in `seq 10 14`; do
    curl -b session=$AOC_SESSION_TOKEN https://adventofcode.com/2021/day/$day/input > data/$day
done

```

Generate a scaffold of today's puzzle

```
./generate_today_scaffold.sh
```

Generate a scaffold of a given day's puzzle

```
./generate_today_scaffold.sh 9

```


or a range

```
for day in `seq 10 14`; do
    ./generate_today_scaffold.sh $day
done
```
