fn fibonacci() {
    algorithms::fibonacci::bottom_up::example();
    algorithms::fibonacci::bottom_up_space_optimized::example();
    algorithms::fibonacci::recursive::example();
    algorithms::fibonacci::top_down_memoized::example();
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
