impl Solution {
    
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut vect: Vec<i32> = vec![0; 2];
        
        for i in 0..nums.len(){
            for j in 0..nums.len(){
                if (nums[j] == target - nums[i] && i != j){
                    vect[0] = i as i32;
                    vect[1] = j as i32;
                }
            }
        }
    
         return vect;
        
    }
}
