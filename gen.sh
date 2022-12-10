#!/usr/bin/env sh

echo "Day number: "
read day

echo "fn main(input: &str) -> impl crate::Results {
    (\"todo\", \"todo\")
}

crate::bp!($day);" >"src/day$day.rs"

touch "src/input/day$day.txt"
touch "src/input/day${day}_test.txt"

sed -i 's/\/\/ gen mod/mod day'$day';\n\/\/ gen mod/g' "src/main.rs"
sed -i 's/\/\/ gen run/day'$day'::run,\n        \/\/ gen run/g' "src/main.rs"
