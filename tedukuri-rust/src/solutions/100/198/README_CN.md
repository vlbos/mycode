198\. 反素数

*    [题目](https://www.acwing.com/problem/content/description/200/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/200/1/)
*    [题解](https://www.acwing.com/problem/content/solution/200/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/200/)

  

对于任何正整数 xx，其约数的个数记作 g(x)g(x)，例如 g(1)\=1、g(6)\=4g(1)\=1、g(6)\=4。

如果某个正整数 xx 满足：对于任意的小于 xx 的正整数 ii，都有 g(x)\>g(i)g(x)\>g(i)，则称 xx 为反素数。

例如，整数 1，2，4，61，2，4，6 等都是反素数。

现在给定一个数 NN，请求出不超过 NN 的最大的反素数。

#### 输入格式

一个正整数 NN。

#### 输出格式

一个整数，表示不超过 NN 的最大反素数。

#### 数据范围

1≤N≤2∗1091≤N≤2∗109

#### 输入样例：

    1000
    

#### 输出样例：

    840
    

难度：中等

时/空限制：1s / 64MB

总通过数：7152

总尝试数：11341

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3749&show_algorithm_tags=0)[HAOI2007](https://www.acwing.com/problem/search/1/?search_content=HAOI2007&source_file_id=3749&show_algorithm_tags=0)[POI2001](https://www.acwing.com/problem/search/1/?search_content=POI2001&source_file_id=3749&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3749&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3749&show_algorithm_tags=1)[约数](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%A6%E6%95%B0&source_file_id=3749&show_algorithm_tags=1)[DFS](https://www.acwing.com/problem/search/1/?search_content=DFS&source_file_id=3749&show_algorithm_tags=1)