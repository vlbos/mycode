408\. 回家

*    [题目](https://www.acwing.com/problem/content/description/410/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/410/1/)
*    [题解](https://www.acwing.com/problem/content/solution/410/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/410/)

  

在网格地图上有 nn 个小人和 nn 个房子，在单位时间内，每个小人都可以水平或垂直移动一个格子。

每个小人移动一步都会花费你 11 美金，直到他进入到一间房子里为止，每间房子只能容纳一人。

你需要计算，所有小人都进入到房子里，你所需要花费的金额最少是多少。

在输入的地图场景中，`.` 表示空地，`H` 表示房子，`m` 表示小人。

地图足够大，并且小人在不进入房间的情况下，也可以踩在有房间的格子上。

#### 输入格式

输入包含多组测试数据。

每组测试数据第一行包含两个整数 NN 和 MM，表示地图大小为 NN 行 MM 列。

接下来 NN 行每行包含 MM 个字符，表示完整的地图场景。

房屋数量与人数量相同，且不超过 100100 个。

当输入一行为 `0 0` 时，表示输入终止。

#### 输出格式

每组数据输出一个整数，表示最少花费。

每个结果占一行。

#### 数据范围

2≤N,M≤1002≤N,M≤100

#### 输入样例：

    2 2
    .m
    H.
    5 5
    HH..m
    .....
    .....
    .....
    mm..H
    7 8
    ...H....
    ...H....
    ...H....
    mmmHmmmm
    ...H....
    ...H....
    ...H....
    0 0
    

#### 输出样例：

    2
    10
    28
    

难度：简单

时/空限制：1s / 64MB

总通过数：346

总尝试数：492

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3959&show_algorithm_tags=0)[POJ2195](https://www.acwing.com/problem/search/1/?search_content=POJ2195&source_file_id=3959&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3959&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3959&show_algorithm_tags=1)[二分图带权匹配](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86%E5%9B%BE%E5%B8%A6%E6%9D%83%E5%8C%B9%E9%85%8D&source_file_id=3959&show_algorithm_tags=1)