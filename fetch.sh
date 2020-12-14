if [ -z "$1" ]; then
    echo "Please provide a day number."
    echo "Usage: $0 DAY"
    exit 1
fi

curl "https://adventofcode.com/2020/day/$1/input" --cookie "session=$(cat AOC_SESSION)" -s | tee "input/day$1"
