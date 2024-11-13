I've reimplemented a few algorithms and resolved some problems for fun.

# index

  - [Algorithms](#algorithms)
  - [Problems](#problems)
  - [Make Recipes](#make-recipes)

# Algorithms

[back^](#index)

  - [sorting/quicksort/classic_hoare](/algorithms/src/sorting/quicksort/classic_hoare.rs)
  - [sorting/quicksort/lomuto_clrs](/algorithms/src/sorting/quicksort/lomuto_clrs.rs)
  - [sorting/quicksort/three_way_bentley_mcilroy](/algorithms/src/sorting/quicksort/three_way_bentley_mcilroy.rs)
  - [sorting/quicksort/three_way](/algorithms/src/sorting/quicksort/three_way.rs)

# Problems

[back^](#index)

  - [leetcode/_000151_reverse_words_in_a_string](/problems/src/leetcode/_000151_reverse_words_in_a_string.rs)
  - [leetcode/_001431_kids_with_the_greatest_number_of_candies](/problems/src/leetcode/_001431_kids_with_the_greatest_number_of_candies.rs)
  - [leetcode/_001768_merge_strings_alternately](/problems/src/leetcode/_001768_merge_strings_alternately.rs)

# Make Recipes

[back^](#index)

```
bash-all             Run all bash tests
bash-check           Check format bash code
bash-fmt             Format bash code
bash-lint            Check lint bash code
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
