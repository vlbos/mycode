316\. 减操作

*    [题目](https://www.acwing.com/problem/content/description/318/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/318/1/)
*    [题解](https://www.acwing.com/problem/content/solution/318/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/318/)

  

给定一个整数数组 a1,a2,…,ana1,a2,…,an。

定义数组第 ii 位上的减操作：把 aiai 和 ai+1ai+1 换成 ai−ai+1ai−ai+1。

用 con(a,i)con(a,i) 表示减操作，可以表示为：

con(a,i)\=\[a1,a2,…,ai−1,ai−ai+1,ai+2,…,an\]con(a,i)\=\[a1,a2,…,ai−1,ai−ai+1,ai+2,…,an\]

长度为 nn 的数组，经过 n−1n−1 次减操作后，就可以得到一个整数 tt。

例如数组 \[12,10,4,3,5\]\[12,10,4,3,5\] 经过如下操作可得到整数 44：

con(\[12,10,4,3,5\],2)\=\[12,6,3,5\]con(\[12,10,4,3,5\],2)\=\[12,6,3,5\]

con(\[12,6,3,5\],3)\=\[12,6,−2\]con(\[12,6,3,5\],3)\=\[12,6,−2\]

con(\[12,6,−2\],2)\=\[12,8\]con(\[12,6,−2\],2)\=\[12,8\]

con(\[12,8\],1)\=\[4\]con(\[12,8\],1)\=\[4\]

现在给定数组以及目标整数，求完整操作过程。

#### 输入格式

第 11 行包含两个整数 nn 和 tt。

第 2..n+12..n+1 行：第 ii 行包含数组中的第 ii 个整数 aiai。

#### 输出格式

输出共 n−1n−1 行，每行包含一个整数，第 ii 行的整数表示第 ii 次减操作的操作位置。

如果方案不唯一，输出任意合理方案均可。

#### 数据范围

1≤n≤1001≤n≤100,  
−10000≤t≤10000−10000≤t≤10000,  
1≤ai≤1001≤ai≤100

#### 输入样例：

    5 4
    12
    10
    4
    3
    5
    

#### 输出样例：

    2
    3
    2
    1
    

难度：中等

时/空限制：1s / 64MB

总通过数：1033

总尝试数：1633

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3867&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3867&show_algorithm_tags=1)[线性DP](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%80%A7DP&source_file_id=3867&show_algorithm_tags=1)