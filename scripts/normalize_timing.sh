#!/usr/bin/env bash

split() {
    src="$1"

    head -20000 "$src" | sed '2~2d' > "$src"_odd
    head -20000 "$src" | sed '1~2d' > "$src"_even
}

normalize() {
    src="$1"
    dest="$1"_normalized

    perl -pe 's/.*Time: (\d+\.\d+ \S+).*/\1/g' "$src" > "$dest"
    sed --in-place -E 's/ μs//g; s/(.+ [^μ]s)/units --terse "\1" μs/e' "$dest"
}

for timing in $*; do
    split "$timing"
    normalize "$timing"_odd
    normalize "$timing"_even

    echo "Normalized $src"
done
