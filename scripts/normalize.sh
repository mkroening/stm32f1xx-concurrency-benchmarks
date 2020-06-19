#!/usr/bin/env bash

normalize() {
    src="$1"
    dest="$1"_normalized

    head -10000 "$src" > "$dest"
    perl -pe 's/.*Time: (\d+\.\d+ \S+).*/\1/g' "$dest" | sponge "$dest"
    sed --in-place -E 's/ μs//g; s/(.+ [^μ]s)/units --terse "\1" μs/e' "$dest"

    echo "Normalized $src as $dest"
}

for timing in $*; do
    normalize "$timing"
done
