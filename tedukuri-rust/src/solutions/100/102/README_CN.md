102\. 最佳牛围栏

*    [题目](https://www.acwing.com/problem/content/description/104/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/104/1/)
*    [题解](https://www.acwing.com/problem/content/solution/104/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/104/)

  

农夫约翰的农场由 NN 块田地组成，每块地里都有一定数量的牛，其数量不会少于 11 头，也不会超过 20002000 头。

约翰希望用围栏将一部分连续的田地围起来，并使得围起来的区域内每块地包含的牛的数量的平均值达到最大。

围起区域内至少需要包含 FF 块地，其中 FF 会在输入中给出。

在给定条件下，计算围起区域内每块地包含的牛的数量的平均值可能的最大值是多少。

#### 输入格式

第一行输入整数 NN 和 FF，数据间用空格隔开。

接下来 NN 行，每行输入一个整数，第 i+1i+1 行输入的整数代表第 ii 片区域内包含的牛的数目。

#### 输出格式

输出一个整数，表示平均值的最大值乘以 10001000 再 **向下取整** 之后得到的结果。

#### 数据范围

1≤N≤1000001≤N≤100000  
1≤F≤N1≤F≤N

#### 输入样例：

    10 6
    6 
    4
    2
    10
    3
    8
    5
    9
    4
    1
    

#### 输出样例：

    6500
    

难度：简单

时/空限制：1s / 64MB

总通过数：22356

总尝试数：55202

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3653&show_algorithm_tags=0)[POJ2018](https://www.acwing.com/problem/search/1/?search_content=POJ2018&source_file_id=3653&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3653&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3653&show_algorithm_tags=0)

算法标签

[二分](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86&source_file_id=3653&show_algorithm_tags=1)[前缀和](https://www.acwing.com/problem/search/1/?search_content=%E5%89%8D%E7%BC%80%E5%92%8C&source_file_id=3653&show_algorithm_tags=1)[双指针](https://www.acwing.com/problem/search/1/?search_content=%E5%8F%8C%E6%8C%87%E9%92%88&source_file_id=3653&show_algorithm_tags=1)