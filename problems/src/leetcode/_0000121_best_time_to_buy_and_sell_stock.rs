// 0000121. Best Time to Buy and Sell Stock
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/
// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
//
// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.
//
// Example 1:
//
// Input: prices = [7,1,5,3,6,4]
// Output: 5
// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
// Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
//
// Example 2:
//
// Input: prices = [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transactions are done and the max profit = 0.
//
// Constraints:
//
//     1 <= prices.length <= 105
//     0 <= prices[i] <= 104
// ---
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = prices[0];
    let mut max_profit = 0;

    for p in prices.iter().skip(1) {
        max_profit = std::cmp::max(max_profit, p - min_price);
        min_price = std::cmp::min(min_price, *p);
    }
    max_profit
}
// ---

pub fn testcase() {
    let res = max_profit(vec![7, 1, 5, 3, 6, 4]);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let res = max_profit(vec![7, 1, 5, 3, 6, 4]);
        let expected = 5;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_2() {
        let res = max_profit(vec![7, 6, 4, 3, 1]);
        let expected = 0;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_3() {
        let res = max_profit(vec![4, 1, 5, 2, 7]);
        let expected = 6;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_4() {
        let res = max_profit(vec![2, 1, 2, 1, 0, 0, 1]);
        let expected = 1;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_5() {
        let res = max_profit(vec![6, 1, 3, 2, 4, 7]);
        let expected = 6;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_6() {
        let res = max_profit(vec![1, 2]);
        let expected = 1;
        assert_eq!(res, expected);
    }
}
