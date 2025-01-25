299\. 裁剪序列

*    [题目](https://www.acwing.com/problem/content/description/301/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/301/1/)
*    [题解](https://www.acwing.com/problem/content/solution/301/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/301/)

  

给定一个长度为 NN 的序列 AA，要求把该序列分成若干段，在满足“每段中所有数的和”不超过 MM 的前提下，让“每段中所有数的最大值”之和最小。

试计算这个最小值。

#### 输入格式

第一行包含两个整数 NN 和 MM。

第二行包含 NN 个整数，表示完整的序列 AA。

#### 输出格式

输出一个整数，表示结果。

如果结果不存在，则输出 −1−1。

#### 数据范围

0≤N≤1050≤N≤105,  
0≤M≤10110≤M≤1011,  
序列A中的数非负，且不超过106106

#### 输入样例：

    8 17
    2 2 2 8 1 8 2 1
    

#### 输出样例：

    12
    

难度：困难

时/空限制：1s / 64MB

总通过数：2150

总尝试数：5779

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3850&show_algorithm_tags=0)

算法标签

[DP](https://www.acwing.com/problem/search/1/?search_content=DP&source_file_id=3850&show_algorithm_tags=1)[双指针](https://www.acwing.com/problem/search/1/?search_content=%E5%8F%8C%E6%8C%87%E9%92%88&source_file_id=3850&show_algorithm_tags=1)[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3850&show_algorithm_tags=1)[单调队列](https://www.acwing.com/problem/search/1/?search_content=%E5%8D%95%E8%B0%83%E9%98%9F%E5%88%97&source_file_id=3850&show_algorithm_tags=1)[堆](https://www.acwing.com/problem/search/1/?search_content=%E5%A0%86&source_file_id=3850&show_algorithm_tags=1)[STL Set](https://www.acwing.com/problem/search/1/?search_content=STL%20Set&source_file_id=3850&show_algorithm_tags=1)