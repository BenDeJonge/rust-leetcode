/// https://leetcode.com/problems/water-bottles/description/
/// Easy - [math, simulation]
/// There are numBottles water bottles that are initially full of water.
/// You can exchange numExchange empty water bottles from the market with one full water bottle.
/// The operation of drinking a full water bottle turns it into an empty bottle.
/// Given the two integers numBottles and numExchange, return the maximum number of water bottles you can drink.
pub struct Solution {}

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        Solution::num_water_bottles_recursive(num_bottles, 0, num_exchange, 0)
    }

    pub fn num_water_bottles_recursive(
        num_full: i32,
        num_empty: i32,
        num_exchange: i32,
        total_drank: i32,
    ) -> i32 {
        // We do not have enough to keep exchanging.
        if num_full + num_empty < num_exchange {
            return total_drank + num_full;
        }
        // We drink all the full bottles.
        let new_empty = num_empty + num_full;
        Solution::num_water_bottles_recursive(
            new_empty / num_exchange,
            new_empty % num_exchange,
            num_exchange,
            total_drank + num_full,
        )
    }
}

fn main() {}

#[cfg(test)]

mod tests {
    use super::Solution;

    #[test]
    fn test_1518() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }
}
