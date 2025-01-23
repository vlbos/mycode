140\. 后缀数组

*    [题目](https://www.acwing.com/problem/content/description/142/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/142/1/)
*    [题解](https://www.acwing.com/problem/content/solution/142/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/142/)

  

后缀数组 (SA) 是一种重要的数据结构，通常使用倍增或者 DC3 算法实现，这超出了我们的讨论范围。

在本题中，我们希望使用快排、Hash 与二分实现一个简单的 O(nlog2n)O(nlog2n) 的后缀数组求法。

详细地说，给定一个长度为 nn 的字符串 SS（下标 0∼n−10∼n−1），我们可以用整数 kk(0≤k<n0≤k<n) 表示字符串 SS 的后缀 S(k∼n−1)S(k∼n−1)。

把字符串 SS 的所有后缀按照字典序排列，排名为 ii 的后缀记为 SA\[i\]SA\[i\]。

额外地，我们考虑排名为 ii 的后缀与排名为 i−1i−1 的后缀，把二者的最长公共前缀的长度记为 Height\[i\]Height\[i\]。

我们的任务就是求出 SASA 与 HeightHeight 这两个数组。

#### 输入格式

输入一个字符串，其长度不超过 3030 万。

字符串由小写字母构成。

#### 输出格式

第一行为数组 SASA，相邻两个整数用 11 个空格隔开。

第二行为数组 HeightHeight，相邻两个整数用 11 个空格隔开，我们规定 Height\[1\]\=0Height\[1\]\=0。

#### 输入样例：

    ponoiiipoi
    

#### 输出样例：

    9 4 5 6 2 8 3 1 7 0
    0 1 2 1 0 0 2 1 0 2
    

难度：中等

时/空限制：3s / 64MB

总通过数：3020

总尝试数：5011

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3691&show_algorithm_tags=0)

算法标签

[字符串hash](https://www.acwing.com/problem/search/1/?search_content=%E5%AD%97%E7%AC%A6%E4%B8%B2hash&source_file_id=3691&show_algorithm_tags=1)[哈希](https://www.acwing.com/problem/search/1/?search_content=%E5%93%88%E5%B8%8C&source_file_id=3691&show_algorithm_tags=1)