class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        for i in range(0, len(nums)):
            for j in range(0, len(nums)):
                if nums[j] == target - nums[i] and i !=j:
                    rt = [i, j]
    return rt                                                                                                                                   
