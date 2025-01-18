I've reimplemented a few algorithms and resolved some problems for fun.

# index

  - [Algorithms](#algorithms)
  - [Problems](#problems)
  - [Make Recipes](#make-recipes)
  - [How to Release](#how-to-release)

# Algorithms

[back^](#index)

  - [numbers/fibonacci/binet_formula](/algorithms/src/numbers/fibonacci/binet_formula.rs)
  - [numbers/fibonacci/bottom_up](/algorithms/src/numbers/fibonacci/bottom_up.rs)
  - [numbers/fibonacci/bottom_up_space_optimized](/algorithms/src/numbers/fibonacci/bottom_up_space_optimized.rs)
  - [numbers/fibonacci/fast_doubling_recursive](/algorithms/src/numbers/fibonacci/fast_doubling_recursive.rs)
  - [numbers/fibonacci/lookup_table](/algorithms/src/numbers/fibonacci/lookup_table.rs)
  - [numbers/fibonacci/matrix_exponentiation_optimized](/algorithms/src/numbers/fibonacci/matrix_exponentiation_optimized.rs)
  - [numbers/fibonacci/matrix_exponentiation_recursive](/algorithms/src/numbers/fibonacci/matrix_exponentiation_recursive.rs)
  - [numbers/fibonacci/recursive](/algorithms/src/numbers/fibonacci/recursive.rs)
  - [numbers/fibonacci/top_down_memoized](/algorithms/src/numbers/fibonacci/top_down_memoized.rs)
  - [sorting/quicksort/classic_hoare](/algorithms/src/sorting/quicksort/classic_hoare.rs)
  - [sorting/quicksort/dual_pivot_kciwegdes](/algorithms/src/sorting/quicksort/dual_pivot_kciwegdes.rs)
  - [sorting/quicksort/dual_pivot_sedgewick](/algorithms/src/sorting/quicksort/dual_pivot_sedgewick.rs)
  - [sorting/quicksort/dual_pivot_yaroslavskiy_bentley_bloch](/algorithms/src/sorting/quicksort/dual_pivot_yaroslavskiy_bentley_bloch.rs)
  - [sorting/quicksort/lomuto_clrs](/algorithms/src/sorting/quicksort/lomuto_clrs.rs)
  - [sorting/quicksort/three_way_bentley_mcilroy](/algorithms/src/sorting/quicksort/three_way_bentley_mcilroy.rs)
  - [sorting/quicksort/three_way](/algorithms/src/sorting/quicksort/three_way.rs)

# Problems

[back^](#index)

  - [codewars/_5667e8f4e3f572a8f2000039_mumbling](/problems/src/codewars/_5667e8f4e3f572a8f2000039_mumbling.rs)
  - [leetcode/_0000136_single_number](/problems/src/leetcode/_0000136_single_number.rs)
  - [leetcode/_0000151_reverse_words_in_a_string](/problems/src/leetcode/_0000151_reverse_words_in_a_string.rs)
  - [leetcode/_0000238_product_of_array_except_self](/problems/src/leetcode/_0000238_product_of_array_except_self.rs)
  - [leetcode/_0000283_move_zeroes](/problems/src/leetcode/_0000283_move_zeroes.rs)
  - [leetcode/_0000334_increasing_triplet_subsequence](/problems/src/leetcode/_0000334_increasing_triplet_subsequence.rs)
  - [leetcode/_0000345_reverse_vowels_of_a_string](/problems/src/leetcode/_0000345_reverse_vowels_of_a_string.rs)
  - [leetcode/_0000392_is_subsequence](/problems/src/leetcode/_0000392_is_subsequence.rs)
  - [leetcode/_0000605_can_place_flowers](/problems/src/leetcode/_0000605_can_place_flowers.rs)
  - [leetcode/_0000724_find_pivot_index](/problems/src/leetcode/_0000724_find_pivot_index.rs)
  - [leetcode/_0001207_unique_number_of_occurrences](/problems/src/leetcode/_0001207_unique_number_of_occurrences.rs)
  - [leetcode/_0001431_kids_with_the_greatest_number_of_candies](/problems/src/leetcode/_0001431_kids_with_the_greatest_number_of_candies.rs)
  - [leetcode/_0001768_merge_strings_alternately](/problems/src/leetcode/_0001768_merge_strings_alternately.rs)
  - [leetcode/_0002215_find_the_difference_of_two_arrays](/problems/src/leetcode/_0002215_find_the_difference_of_two_arrays.rs)
  - [leetcode/_0002390_removing_stars_from_a_string](/problems/src/leetcode/_0002390_removing_stars_from_a_string.rs)

# Make Recipes

[back^](#index)

```
bash-all               Run all bash tests
bash-check             Check format bash code
bash-deps              Install bash dependencies
bash-fmt               Format bash code
bash-lint              Check lint bash code
comments-tidy          Tidy comments within code
doc-changelog          Write CHANGELOG.mode
doc-readme             Write README.md
dprint-check           Dprint check
dprint-fmt             Dprint format
help                   Display this help screen
makefile-descriptions  Check if all Makefile rules have descriptions
rs-audit               Audit Cargo.lock
rs-audit-fix           Update Cargo.toml to fix vulnerable dependency requirement
rs-build               Build binary
rs-cargo-deps          Install cargo dependencies
rs-check               Run check
rs-dev                 Run check in watch mode
rs-doc                 Open app documentation
rs-fix                 Fix rust code
rs-fmt                 Format rust code
rs-fmt-fix             Format fix rust code
rs-install             Install binary
rs-lint                Lint rust code
rs-lint-fix            Fix lint rust code
rs-outdated            Display when dependencies are out of date
rs-run                 Run rust code
rs-tests               Run tests
rs-uninstall           Uninstall binary
rs-update-cargo        Update dependencies
rs-update-rustup       Update rust
typos                  Check typos
typos-fix              Fix typos
```

# How to Release

[back^](#index)

To generate a new version, you need to follow these steps:

1. In the `main` branch, you must bump the version inside the `Cargo.toml` file.
2. Run `make rs-check` so that the version is changed in the `Cargo.lock` file.
3. Run the command `git add -A && git commit -m "release: bump version"`.
4. Run the command `git tag -a <your.new.version> -m "version <your.new.version>"`.
5. Run the command `make doc-changelog && make doc-readme`.
6. Run the command `git add -A && git commit -m "release: <your.new.version>"`.
7. Run `git push` to `main`.
