//! https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
//! Easy - [array, dynamic programming]
//! You are given an array prices where prices[i] is the price of a given stock on the ith day.
//! You want to maximize your profit by choosing a single day to buy one stock and
//!  choosing a different day in the future to sell that stock.
//! Return the maximum profit you can achieve from this transaction.
//! If you cannot achieve any profit, return 0.

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = prices.first().expect("prices is empty");
        let mut profit = 0;
        for price in prices.iter() {
            // This is a better moment to buy.
            if price < buy {
                buy = price
            }
            // This is a better moment to sell, so update the profit.
            else if price - buy > profit {
                profit = price - buy
            }
        }
        profit
    }

    /// This is too complex, as it offers also information on WHEN the maximal profit can be arranged.
    /// It requires 3 passes through the array (gettings the buys, the sells and then the differences) -> O(3n)
    /// It stores the array 3 times (original, buys and sells) -> O(3n)
    pub fn max_profit_naive(prices: Vec<i32>) -> i32 {
        // Get the rolling value of the best (lowest) buy ...
        let mut min_buy = prices.first().expect("prices is empty");
        let buys = prices.iter().map(|p| {
            min_buy = min_buy.min(p);
            *min_buy
        });
        // ... and the best (highest) sell values.
        let mut max_sell = 0;
        let mut sells = vec![max_sell; prices.len()];
        // Build this array from the back because we can only get that sell price when we buy before that date.
        for i in 1..prices.len() {
            let i_rev = prices.len() - i;
            max_sell = max_sell.max(prices[i_rev]);
            sells[i_rev - 1] = max_sell;
        }
        // Get the max with a min of 0.
        buys.zip(sells.iter())
            .map(|(b, s)| s - b)
            .fold(0, |best, curr| best.max(curr))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0121() {
        assert_eq!(Solution::max_profit(vec![1, 2]), 1);
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![5]), 0);
    }
}
