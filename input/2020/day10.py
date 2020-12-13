import numpy as np
from math import comb, factorial, perm


with open("day10.txt") as f:
    joltages = f.readlines()


joltages = [int(jolt) for jolt in joltages]
# add the wall joltage
joltages.insert(0, 0)
chain = np.sort(joltages)
phone_rating = chain[-1] + 3
full_chain = np.append(chain, phone_rating)
diff_dict = {1: 0, 2:0, 3: 0}


# Info
print("\nProblem Info:")
print("============")
print(f"Adapater Chain: {full_chain}")
print(f"Length of Chain: {len(full_chain)}")


for i, adapter in enumerate(chain):
    diff = full_chain[i + 1] - adapter
    if diff > 3 or diff < 0:
        print("Not possible!!")
    else:
        diff_dict[diff] += 1


# Ex 1
print("\nExercise 1:")
print("===========")
print(f"\nDifference Counts: {diff_dict}")
print(f"1-diff x 3-diff: {diff_dict[1] * diff_dict[3]}\n")


# Ex 2
print("\nExercise 2:")
print("===========")

def make_chain(array):
    from functools import lru_cache
    
    @lru_cache
    def combinations_from(index):
        # Base Case: if at end of list, end recursion
        if index == len(array):
            return 1
        count = combinations_from(index + 1)
        if index + 2 < len(array) and array[index + 2] - array[index] <= 3:
            count += combinations_from(index + 2)
        if index + 3 < len(array) and array[index + 3] - array[index] <= 3:
            count += combinations_from(index + 3)
        return count
        
    return combinations_from(0)


print(f"\nNumber of Possible Combinations for chain is: {make_chain(full_chain)}")
