#!/bin/bash

declare TRACE
[[ "${TRACE}" == 1 ]] && set -o xtrace
set -o errexit
set -o nounset
set -o pipefail
set -o noclobber
shopt -s inherit_errexit

index() {
  paste -d "" \
    <(
      cat dev/doc-readme.sh |
        grep -E '^#{1,} [A-Z]' |
        sed 's/^ {1,}//g' |
        sed -E 's/(^#{1,}) (.+)/\1\[\2]/g' |
        sed 's/#/  /g' |
        sed -E 's/\[/- [/g'
    ) \
    <(
      cat dev/doc-readme.sh |
        grep -E '^#{1,} [A-Z]' |
        sed 's/#//g' |
        sed -E 's/^ {1,}//g' |
        # https://www.gnu.org/software/grep/manual/html_node/Character-Classes-and-Bracket-Expressions.html
        sed -E "s1[][!#$%&'()*+,./:;<=>?@\\^_\`{|}~]11g" |
        sed -E 's/"//g' |
        sed 's/[A-Z]/\L&/g' |
        sed 's/ /-/g' |
        sed -E 's@(.+)@(#\1)@g'
    )
}

backlink() {
  sed -i -E '/^#{1,} [A-Z]/a\\n\[back^\](#index)' README.md
}

algorithms_list() {
  find algorithms -name '*.rs' |
    grep mod -v |
    grep utils -v |
    grep main -v |
    grep my_benchmark -v |
    grep lib -v |
    sort |
    sed 's@algorithms/src/@@g' |
    sed -E 's/\.rs$//g' |
    sed -E 's@(.+)@  - [\1](/algorithms/src/\1.rs)@g'
}

problems_list() {
  find problems -name '*.rs' |
    grep mod -v |
    grep utils -v |
    grep main -v |
    grep my_benchmark -v |
    grep lib -v |
    sort |
    sed 's@problems/src/@@g' |
    sed -E 's/\.rs$//g' |
    sed -E 's@(.+)@  - [\1](/problems/src/\1.rs)@g'
}

readme() {
  cat <<EOF >|README.md
I've reimplemented a few algorithms and resolved some problems for fun.

# index

$(index)

# Algorithms

$(algorithms_list)

# Problems

$(problems_list)

# Make Recipes

\`\`\`
$(make help)
\`\`\`

# How to Release

$(cat HOW_TO_RELEASE.md)
EOF

  sed -i -E '/^make\[[0-9]/d' README.md
  backlink
  dprint fmt README.md CHANGELOG.md
}

trap readme EXIT
