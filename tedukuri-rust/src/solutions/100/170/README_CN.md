170\. 加成序列

*    [题目](https://www.acwing.com/problem/content/description/172/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/172/1/)
*    [题解](https://www.acwing.com/problem/content/solution/172/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/172/)

  

满足如下条件的序列 XX（序列中元素被标号为 1、2、3…m1、2、3…m）被称为“加成序列”：

1.  X\[1\]\=1X\[1\]\=1
2.  X\[m\]\=nX\[m\]\=n
3.  X\[1\]<X\[2\]<…<X\[m−1\]<X\[m\]X\[1\]<X\[2\]<…<X\[m−1\]<X\[m\]
4.  对于每个 kk（2≤k≤m2≤k≤m）都存在两个整数 ii 和 jj （1≤i,j≤k−11≤i,j≤k−1，ii 和 jj 可相等），使得 X\[k\]\=X\[i\]+X\[j\]X\[k\]\=X\[i\]+X\[j\]。

你的任务是：给定一个整数 nn，找出符合上述条件的长度 mm 最小的“加成序列”。

如果有多个满足要求的答案，只需要找出任意一个可行解。

#### 输入格式

输入包含多组测试用例。

每组测试用例占据一行，包含一个整数 nn。

当输入为单行的 00 时，表示输入结束。

#### 输出格式

对于每个测试用例，输出一个满足需求的整数序列，数字之间用空格隔开。

每个输出占一行。

#### 数据范围

1≤n≤1001≤n≤100

#### 输入样例：

    5
    7
    12
    15
    77
    0
    

#### 输出样例：

    1 2 4 5
    1 2 4 6 7
    1 2 4 8 12
    1 2 4 5 10 15
    1 2 4 8 9 17 34 68 77
    

难度：简单

时/空限制：1s / 64MB

总通过数：14866

总尝试数：24209

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3721&show_algorithm_tags=0)[UVA529](https://www.acwing.com/problem/search/1/?search_content=UVA529&source_file_id=3721&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3721&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3721&show_algorithm_tags=1)[迭代加深](https://www.acwing.com/problem/search/1/?search_content=%E8%BF%AD%E4%BB%A3%E5%8A%A0%E6%B7%B1&source_file_id=3721&show_algorithm_tags=1)