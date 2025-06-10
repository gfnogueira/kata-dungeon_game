struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();

        let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];
        dp[m][n - 1] = 1;
        dp[m - 1][n] = 1;

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let min_health_on_exit = dp[i + 1][j].min(dp[i][j + 1]);
                dp[i][j] = (min_health_on_exit - dungeon[i][j]).max(1);
            }
        }

        dp[0][0]
    }
}

fn main() {
    let dungeon = vec![
        vec![-2, -3, 3],
        vec![-5, -10, 1],
        vec![10, 30, -5],
    ];
    let result = Solution::calculate_minimum_hp(dungeon);
    println!("Minimum initial health required: {}", result);
}