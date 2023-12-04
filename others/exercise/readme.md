Record
=================


# Answer
2.1:  
-------------------------
根据您提供的算法，当输入值 n 为 18 时，按照递归的方式来计算 f(18) 的值。

我们来追溯算法的执行流程：

�
(
18
)
=
3
+
�
(
15
)
f(18)=3+f(15)
�
(
15
)
=
3
+
�
(
12
)
f(15)=3+f(12)
�
(
12
)
=
3
+
�
(
9
)
f(12)=3+f(9)
�
(
9
)
=
3
+
�
(
6
)
f(9)=3+f(6)
�
(
6
)
=
3
+
�
(
3
)
f(6)=3+f(3)
�
(
3
)
=
3
+
�
(
0
)
f(3)=3+f(0)

在 f(3) 中，n 的值为 3，不是基本情况（0、1 或 2）。因此，我们将继续迭代：

�
(
0
)
=
1
f(0)=1

现在我们可以计算回之前的调用：

�
(
3
)
=
3
+
�
(
0
)
=
3
+
1
=
4
f(3)=3+f(0)=3+1=4
�
(
6
)
=
3
+
�
(
3
)
=
3
+
4
=
7
f(6)=3+f(3)=3+4=7
�
(
9
)
=
3
+
�
(
6
)
=
3
+
7
=
10
f(9)=3+f(6)=3+7=10
�
(
12
)
=
3
+
�
(
9
)
=
3
+
10
=
13
f(12)=3+f(9)=3+10=13
�
(
15
)
=
3
+
�
(
12
)
=
3
+
13
=
16
f(15)=3+f(12)=3+13=16
�
(
18
)
=
3
+
�
(
15
)
=
3
+
16
=
19
f(18)=3+f(15)=3+16=19

因此，根据您提供的算法，当输入值 n 为 18 时，f(18) 的返回值为 19。


2.2
--------------------------
In order to determine the number of function calls or "boxes" traced in the execution of the algorithm for 
�
=
18
n=18, we can count the number of recursive calls made during the execution.

Let's break down the recursive calls:

�
(
18
)
→
�
(
15
)
→
�
(
12
)
→
�
(
9
)
→
�
(
6
)
→
�
(
3
)
→
�
(
0
)
f(18)→f(15)→f(12)→f(9)→f(6)→f(3)→f(0)

The number of recursive calls made corresponds to the number of times the function f() was invoked during the calculation. Counting these recursive calls, we have a total of 6 function calls made before reaching the base case where 
�
=
0
n=0.

Therefore, for 
�
=
18
n=18, the number of "boxes" or recursive calls in the box-tracing of the algorithm is 6.



2.3
---------------------
recurrence relation for 𝑆(𝑛):   S(n)=1+S(n−3)

initial condition:  S(0)=S(1)=S(2)=0 



4.1
----------------------
O(kn)

4.2
----------------------
O(kn)

4.3
----------------------
O(kn)

4.4 
----------------------
O(n log n)，其中 n 是数组的长度。




4.5 
---------------------
1. **Iteration 1:**
   - Median of A = 15
   - S1 = [8, 12, 11, 5, 9, 1, 7, 3]
   - S2 = [21, 18, 22, 16, 17, 23]
   - p = 8 (length of S1)
   - q = 6 (length of S2)
   - k = 11

2. **Iteration 2 (Choosing S2):**
   - Median of S2 = 18
   - S1 = [16, 17]
   - S2 = [21, 22, 23]
   - p = 2
   - q = 3
   - k = 3 (11 - (8 + 1))

3. **Iteration 3 (Choosing S2):**
   - Median of S2 = 22
   - S1 = []
   - S2 = [23]
   - p = 0
   - q = 1
   - k = 2

4. **Iteration 4 (Choosing S2):**
   - Median of S2 = 23
   - S1 = []
   - S2 = []
   - p = 0
   - q = 0
   - k = 1

final iteration the 11th smallest element is 23.



Certainly! Let's proceed with the backward substitution method to derive the big-O complexity class for the Selectkth algorithm.

Given the recurrence relation: 
\(𝐶(𝑛) \leq 𝐶(⌊𝑛/2⌋) + 6𝑛 + 𝑑𝑛\)

We want to express \(𝐶(2𝑞)\) in terms of \(𝐶(2𝑞-1)\) using this relation.

(a) Formula for \(𝐶(2𝑞)\) after \(𝑖\) substitutions:

Starting with \(𝐶(2𝑞)\) and substituting \(𝐶(2𝑞-1)\), \(𝐶(2𝑞-2)\), and so on up to \(𝐶(1)\), we get:

\[
\begin{align*}
𝐶(2𝑞) &\leq 𝐶(2𝑞-1) + 6(2𝑞) + 𝑑(2𝑞) \\
&\leq (𝐶(2𝑞-2) + 6(2𝑞-1) + 𝑑(2𝑞-1)) + 6(2𝑞) + 𝑑(2𝑞) \\
&\leq \dots \\
&\leq 𝐶(1) + \sum_{𝑖=1}^{2𝑞} (6𝑖 + 𝑑𝑖) \\
&\leq 1 + 6\sum_{𝑖=1}^{2𝑞} 𝑖 + 𝑑\sum_{𝑖=1}^{2𝑞} 𝑖 \\
&\leq 1 + 6 \cdot \frac{(2𝑞)(2𝑞 + 1)}{2} + 𝑑 \cdot \frac{(2𝑞)(2𝑞 + 1)}{2} \\
&\leq 1 + 6𝑞(4𝑞 + 2) + 𝑑𝑞(4𝑞 + 2) \\
&\leq 24𝑞^2 + (12 + 𝑑)𝑞
\end{align*}
\]

(b) Closed form solution obtained for \(𝑖 = 𝑞\):

Substituting \(𝑞\) for \(𝑖\) in the derived formula:

\(𝐶(2𝑞) \leq 24𝑞^2 + (12 + 𝑑)𝑞\)

(c) Big-O complexity class:

The dominating term in the equation is \(𝑞^2\) as \(𝑞\) tends to infinity. Thus, the big-O complexity class for \(𝐶(2𝑞)\) is \(O(𝑛^2)\) (where \(𝑛\) is the size of the input array).

Note: The specific value of \(𝑑\) was not provided, but it's evident from the analysis that the complexity is \(O(𝑛^2)\) due to the \(𝑛^2\) term being the most significant one in the derived formula.


  
