I've reimplemented a few algorithms and resolved some problems for fun.

# index

  - [Algorithms](#algorithms)
  - [Problems](#problems)
  - [Make Recipes](#make-recipes)

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
bash-all             Run all bash tests
bash-check           Check format bash code
bash-fmt             Format bash code
bash-lint            Check lint bash code
comments-tidy        Tidy comments within code
doc-changelog        Write CHANGELOG.mode
doc-readme           Write README.md
help                 Display this help screen
rs-audit             Audit Cargo.lock
rs-audit-fix         Update Cargo.toml to fix vulnerable dependency requirement
rs-build             Build binary
rs-cargo-deps        Install cargo dependencies
rs-check             Run check
rs-dev               Run check in watch mode
rs-doc               Open app documentation
rs-fix               Fix rust code
rs-fmt-fix           Format fix rust code
rs-fmt               Format rust code
rs-install           Install binary
rs-lint-fix          Fix lint rust code
rs-lint              Lint rust code
rs-outdated          Display when dependencies are out of date
rs-tests             Run tests
rs-uninstall         Uninstall binary
rs-update-cargo      Update dependencies
toml-fmt             Format toml code
toml-lint            Check toml yaml code
typos                Check typos
typos-fix            Fix typos
yaml-fmt             Format yaml code
yaml-lint            Check lint yaml code
```
