fn fibonacci() {
    algorithms::numbers::fibonacci::binet_formula::example();
    algorithms::numbers::fibonacci::bottom_up::example();
    algorithms::numbers::fibonacci::bottom_up_space_optimized::example();
    algorithms::numbers::fibonacci::fast_doubling_recursive::example();
    algorithms::numbers::fibonacci::lookup_table::example();
    algorithms::numbers::fibonacci::matrix_exponentiation_optimized::example();
    algorithms::numbers::fibonacci::matrix_exponentiation_recursive::example();
    algorithms::numbers::fibonacci::recursive::example();
    algorithms::numbers::fibonacci::top_down_memoized::example();
}

fn sorting() {
    algorithms::sorting::quicksort::classic_hoare::example();
    algorithms::sorting::quicksort::dual_pivot_kciwegdes::example();
    algorithms::sorting::quicksort::dual_pivot_sedgewick::example();
    algorithms::sorting::quicksort::dual_pivot_yaroslavskiy_bentley_bloch::example();
    algorithms::sorting::quicksort::lomuto_clrs::example();
    algorithms::sorting::quicksort::three_way::example();
    algorithms::sorting::quicksort::three_way_bentley_mcilroy::example();
}

fn main() {
    fibonacci();
    sorting();
}
