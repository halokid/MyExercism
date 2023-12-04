def f(n):
  if (n == 0) or (n == 1) or (n == 2):
    return 1
  else:
    return 3 + f(n - 3)


# -------------------------------------------------------
def kth_smallest_bubble_sort(arr, k):
  n = len(arr)

  if k < 1 or k > n:
    return "Invalid k value"

  for i in range(k):  # 进行 k 次冒泡
    print('i -->>>', i)
    print('(n - i - 1) -->>>', (n - i -1))
    for j in range(n - i - 1):
      if arr[j] > arr[j + 1]:
        arr[j], arr[j + 1] = arr[j + 1], arr[j]

  return arr[k - 1]  # 第 k 个最小的元素

# -----------------------------------------------
'''
def kth_smallest_median_based(arr, k):
  def median(arr):
    # Function that returns the median of an array
    # This can be considered as a black box
    # Assuming it takes O(n) operations
    return sorted(arr)[len(arr) // 2]

  n = len(arr)
  if k < 1 or k > n:
    return "Invalid k value"

  while True:
    pivot = median(arr)
    S1 = [x for x in arr if x < pivot]
    S2 = [x for x in arr if x > pivot]

    p, q = len(S1), len(S2)

    if p == k - 1:
      return pivot
    elif p > k - 1:
      arr = S1
    else:
      arr = S2
      k -= (p + 1)

# Example array
arr = [7, 10, 4, 3, 20, 15]
k = 3
result = kth_smallest_median_based(arr, k)
print(f"The {k}-th smallest element in the array is:", result)
'''

# if __name__ == '__main__':
  # s = f(18)
  # print(s)
  # -------------------

  # 示例数组
  # arr = [7, 10, 4, 3, 20, 15]
  # k = 4
  # result = kth_smallest_bubble_sort(arr, k)
  # print(f"The {k}-th smallest element in the array is:", result)



def median(arr):
  # Function that returns the median of an array
  # This can be considered as a black box
  # Assuming it takes O(n) operations
  return sorted(arr)[len(arr) // 2]

def kth_smallest_median_based(arr, k):
  iterations = []
  while True:
    pivot = median(arr)
    S1 = [x for x in arr if x < pivot]
    S2 = [x for x in arr if x > pivot]

    p, q = len(S1), len(S2)
    iterations.append((S1, S2, p, q, k))

    if p == k - 1:
      return iterations
    elif p > k - 1:
      arr = S1
    else:
      arr = S2
      k -= (p + 1)

# Example array and parameters
arr = [21, 8, 18, 12, 11, 22, 5, 9, 16, 1, 7, 3, 15, 17, 23]
k = 11
result = kth_smallest_median_based(arr, k)
for i, values in enumerate(result):
  print(f"Iteration {i + 1}:")
  print("S1:", values[0])
  print("S2:", values[1])
  print("p:", values[2])
  print("q:", values[3])
  print("k:", values[4])
  print()






