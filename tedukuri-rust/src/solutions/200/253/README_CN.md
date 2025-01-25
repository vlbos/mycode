253\. 普通平衡树

*    [题目](https://www.acwing.com/problem/content/description/255/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/255/1/)
*    [题解](https://www.acwing.com/problem/content/solution/255/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/255/)

  

您需要写一种数据结构（可参考题目标题），来维护一些数，其中需要提供以下操作：

1.  插入数值 xx。
2.  删除数值 xx(若有多个相同的数，应只删除一个)。
3.  查询数值 xx 的排名(若有多个相同的数，应输出最小的排名)。
4.  查询排名为 xx 的数值。
5.  求数值 xx 的前驱(前驱定义为小于 xx 的最大的数)。
6.  求数值 xx 的后继(后继定义为大于 xx 的最小的数)。

**注意：** 数据保证查询的结果一定存在。

#### 输入格式

第一行为 nn，表示操作的个数。

接下来 nn 行每行有两个数 optopt 和 xx，optopt 表示操作的序号(1≤opt≤61≤opt≤6)。

#### 输出格式

对于操作 3,4,5,63,4,5,6 每行输出一个数，表示对应答案。

#### 数据范围

1≤n≤1000001≤n≤100000,所有数均在 −107−107 到 107107 内。

#### 输入样例：

    8
    1 10
    1 20
    1 30
    3 20
    4 2
    2 10
    5 25
    6 -1
    

#### 输出样例：

    2
    20
    20
    20
    

难度：中等

时/空限制：1s / 64MB

总通过数：14250

总尝试数：26723

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3804&show_algorithm_tags=0)[模板题](https://www.acwing.com/problem/search/1/?search_content=%E6%A8%A1%E6%9D%BF%E9%A2%98&source_file_id=3804&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3804&show_algorithm_tags=0)

算法标签

[平衡树](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%B3%E8%A1%A1%E6%A0%91&source_file_id=3804&show_algorithm_tags=1)[Treap](https://www.acwing.com/problem/search/1/?search_content=Treap&source_file_id=3804&show_algorithm_tags=1)