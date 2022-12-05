#!/usr/bin/env sh

echo "Day number: "
read day

echo "fn main(input: &str) -> impl crate::Results {
    (0, 0)
}

crate::bp!($day);" > "src/day$day.rs"

touch "src/day$day.txt"
touch "src/day${day}_test.txt"

sed -i 's/\/\/ gen mod/mod day'$day';\n\/\/ gen mod/g' "src/main.rs"
sed -i 's/\/\/ gen run/day'$day'::run();\n    \/\/ gen run/g' "src/main.rs"
