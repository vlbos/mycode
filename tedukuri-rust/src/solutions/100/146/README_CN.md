146\. 序列

*    [题目](https://www.acwing.com/problem/content/description/148/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/148/1/)
*    [题解](https://www.acwing.com/problem/content/solution/148/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/148/)

  

给定 mm 个序列，每个包含 nn 个非负整数。

现在我们可以从每个序列中选择一个数字以形成具有 mm 个整数的序列。

很明显，我们一共可以得到 nmnm 个这种序列，然后我们可以计算每个序列中的数字之和，并得到 nmnm 个值。

现在请你求出这些序列和之中最小的 nn 个值。

#### 输入格式

第一行输入一个整数 TT，代表输入中包含测试用例的数量。

接下来输入 TT 组测试用例。

对于每组测试用例，第一行输入两个整数 mm 和 nn。

接下在 mm 行输入 mm 个整数序列，数列中的整数均不超过 1000010000。

#### 输出格式

对于每组测试用例，均以递增顺序输出最小的 nn 个序列和，数值之间用空格隔开。

每组输出占一行。

#### 数据范围

0<m≤10000<m≤1000,  
0<n≤20000<n≤2000

#### 输入样例：

    1
    2 3
    1 2 3
    2 2 3
    

#### 输出样例：

    3 3 4
    

难度：简单

时/空限制：1s / 64MB

总通过数：5096

总尝试数：13911

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3697&show_algorithm_tags=0)

算法标签

[二叉堆](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%8F%89%E5%A0%86&source_file_id=3697&show_algorithm_tags=1)[多路归并](https://www.acwing.com/problem/search/1/?search_content=%E5%A4%9A%E8%B7%AF%E5%BD%92%E5%B9%B6&source_file_id=3697&show_algorithm_tags=1)