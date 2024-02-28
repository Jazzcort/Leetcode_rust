mod util;

// mod p455_assign_cookie;
// #[allow(non_snake_case)]
// mod p2610_convert_an_array_into_a_2D_array_with_conditions;
// mod p215_kth_largest_element_in_an_array;
// mod p2125_number_of_laser_beams_in_a_bank;
// mod p2870_minimum_number_of_operations_to_make_array_empty;
// mod p300_longest_increasing_subsequence;
// mod p446_arithmetic_slices_ii_subsequence;
// mod p303_range_sum_query_immutable;
// mod p938_range_sum_of_bst;
// mod p872_leaf_similar_trees;
// mod p2385_amount_of_time_for_binary_tree_to_be_infected;
// mod p1026_maximum_difference_between_node_and_ancestor;
// mod p1704_determine_if_string_halves_are_alike;
// mod p2225_find_players_with_zero_or_one_losses;
// mod p380_insert_delete_get_random;
// mod p913_minimum_falling_path_sum;
// mod p128_longest_consecutive_sequence;
// mod p907_sum_of_subarray_minimums;
// mod p198_house_robber;
// mod p1143_longest_common_subsequence;
// mod p576_out_of_boundary_paths;
// mod p1074_number_of_submatrices_that_sum_to_target;
// mod p232_implement_queue_using_stacks;
// mod p739_daily_temperatures;
// mod p2966_divide_array_into_arrays_with_max_difference;
// mod p49_group_anagrams;
// mod p451_sort_characters_by_frequency;
// mod p2108_find_first_palindromic_string_in_the_array;
// mod p2149_rearrange_array_elements_by_sign;
// mod p2971_find_polygon_with_the_largest_perimeter;
// mod p1642_furthest_building_you_can_reach;
// mod p787_cheapest_flights_within_k_stops;
mod p513_find_bottom_left_tree_values;

// use crate::p455_assign_cookie::solution as p455;
// use crate::p2610_convert_an_array_into_a_2D_array_with_conditions::solution as p2610;
// use crate::p215_kth_largest_element_in_an_array::solution as p215;
// use crate::p2125_number_of_laser_beams_in_a_bank::solution as p2125;
// use p2870_minimum_number_of_operations_to_make_array_empty::solution as p2870;
// use p300_longest_increasing_subsequence::solution as p300;
// use p446_arithmetic_slices_ii_subsequence::solution as p446;
// use p303_range_sum_query_immutable::solution as p303;
// use p938_range_sum_of_bst::solution as p938;
// use p872_leaf_similar_trees::solution as p872;
// use p2385_amount_of_time_for_binary_tree_to_be_infected::solution as p2385;
// use p1026_maximum_difference_between_node_and_ancestor::solution as p1026;
// use p1704_determine_if_string_halves_are_alike::solution as p1704;
// use p2225_find_players_with_zero_or_one_losses::solution as p2225;
// use p380_insert_delete_get_random::solution::RandomizedSet;
// use p913_minimum_falling_path_sum::solution as p913;
// use p128_longest_consecutive_sequence::solution as p128;
// use p907_sum_of_subarray_minimums::solution as p907;
// use p198_house_robber::solution as p198;
// use p1143_longest_common_subsequence::solution as p1143;
// use p576_out_of_boundary_paths::solution as p576;
// use p1074_number_of_submatrices_that_sum_to_target::solution as p1074;
// use p232_implement_queue_using_stacks::solution as p232;
// use p739_daily_temperatures::solution as p739;
// use p2966_divide_array_into_arrays_with_max_difference::solution as p2966;
// use p49_group_anagrams::solution as p49;
// use p451_sort_characters_by_frequency::solution as p451;
// use p2108_find_first_palindromic_string_in_the_array::solution as p2108;
// use p2149_rearrange_array_elements_by_sign::solution as p2149;
// use p2971_find_polygon_with_the_largest_perimeter::solution as p2971;
// use p1642_furthest_building_you_can_reach::solution as p1642;
// use p787_cheapest_flights_within_k_stops::solution as p787;
use p513_find_bottom_left_tree_values::solution as p513;

use crate::util::tree_node::Tree;

fn main() {
    // dbg!(p455::Solution::find_content_children(vec![1,2,3], vec![1,1]));
    // dbg!(p2610::Solution::find_matrix(vec![1,3,4,1,2,3,1]));
    // dbg!(p215::Solution::find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4));
    // dbg!(p2125::Solution::number_of_beams(vec!["0110".to_string(), "0100".to_string()]));
    // dbg!(p2870::Solution::min_operations(vec![1,1,1,2,2,3,3,3,3,4,4]));
    // dbg!(p300::Solution::length_of_lis(vec![0,1,0,3,2,3]));
    // dbg!(p446::Solution::number_of_arithmetic_slices(vec![0,2000000000,-294967296]));
    // let a = p303::NumArray::new(vec![2,3,4,5]);
    // let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // dbg!(p938::Solution::range_sum_bst(root, 2, 5));
    // let tree1 = util::tree_node::Tree::from(vec![Some(1),Some(5),Some(3),None,Some(4),Some(10),Some(6),Some(9),Some(2)]);
    // dbg!(p2385::Solution::amount_of_time(tree1.get_head(), 3));

    // dbg!(p1026::Solution::max_ancestor_diff(tree1.get_head()));

    // dbg!(p1704::Solution::halves_are_alike("apples".to_string()));

    // dbg!(p2225::Solution::find_winners(vec!(vec![1,3], vec![2,3], vec![3,6], vec![5,6], vec![5,7], vec![4,5], vec![4,8], vec![4,9], vec![10,4], vec![10,9])));
    // let mut tmp = RandomizedSet::new();
    // tmp.insert(1);
    // tmp.insert(2);
    // tmp.insert(24);
    // tmp.insert(8);
    // tmp.insert(90);
    // dbg!(tmp.get_random());
    // dbg!(p913::Solution::min_falling_path_sum(vec![vec![2,1,3],vec![6,5,4],vec![7,8,9]]));
    // dbg!(p128::Solution::longest_consecutive(vec![100,4,200,1,3,2]));
    // dbg!(p907::Solution::sum_subarray_mins(vec![3,1,2,4]));
    // dbg!(p198::Solution::rob(vec![2,7,9,3,1]));
    // dbg!(p1143::Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()));
    // dbg!(p576::Solution::find_paths(2, 2, 2, 0, 0));
    // dbg!(p1074::Solution::num_submatrix_sum_target(vec![vec![0,1,0],vec![1,1,1],vec![0,1,0]], 1));
    // let mut obj = p232::MyQueue::new();
    // obj.push(1);
    // obj.push(2);
    // dbg!(obj.peek());
    // dbg!(obj.pop());
    // dbg!(obj.empty());
    // dbg!(p739::Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73]));
    // dbg!(p2966::Solution::divide_array(vec![1,3,4,8,7,9,3,5,1], 2));
    // dbg!(p49::Solution::group_anagrams(vec!["tea".to_string(),"and".to_string(),"ace".to_string(),"ad".to_string(),"eat".to_string(),"dans".to_string()]));
    // dbg!(p451::Solution::frequency_sort("tree".to_string()));
    // dbg!(p2108::Solution::first_palindrome(vec!["acb".to_string(), "aabbaa".to_string()]));
    // dbg!(p2149::Solution::rearrange_array(vec![3,1,-2,-5,2,-4]));
    // dbg!(p2971::Solution::largest_perimeter(vec![1,12,1,2,5,50,3]));
    // dbg!(p1642::Solution::furthest_building(vec![4,12,2,7,3,18,20,3,19], 10, 2));
    // dbg!(p787::Solution::find_cheapest_price(
    //     4,
    //     vec![
    //         vec![0, 1, 100],
    //         vec![1, 2, 100],
    //         vec![2, 0, 100],
    //         vec![1, 3, 600],
    //         vec![2, 3, 200]
    //     ],
    //     0,
    //     3,
    //     1
    // ));
    // dbg!(p787::Solution::find_cheapest_price(
    //     3,
    //     vec![
    //         vec![0, 1, 100],
    //         vec![1, 2, 100],
    //         vec![0, 2, 500]
    //     ],
    //     0,
    //     2,
    //     1
    // ));
    // dbg!(p787::Solution::find_cheapest_price(
    //     4,
    //     vec![
    //         vec![0, 1, 1],
    //         vec![0, 2, 5],
    //         vec![1, 2, 1],
    //         vec![2, 3, 1]
    //     ],
    //     0,
    //     3,
    //     1
    // ));

    let tree = Tree::from(vec![Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6), None, None, Some(7)]);

    dbg!(p513::Solution::find_bottom_left_value(tree.get_head()));
}
