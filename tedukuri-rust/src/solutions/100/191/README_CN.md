191\. 天气预报

*    [题目](https://www.acwing.com/problem/content/description/193/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/193/1/)
*    [题解](https://www.acwing.com/problem/content/solution/193/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/193/)

  

你是一个可以控制风的神仙。

通过把云吹到不同的位置，你可以控制降雨。

云下地区会降雨，没有云的地方阳光灿烂。

你是一个仁慈的神，希望土地在平时可以有足够的雨水，在赶集和过节能够充满阳光。

你负责掌控一个村子的天气状况。

这个村子呈 4×44×4 的网格状分布，村子内的每个区域被编号如下图所示：

![天气.jpg](https://cdn.acwing.com/media/article/image/2019/01/17/19_1d9de8ca19-%E5%A4%A9%E6%B0%94.jpg)

你拥有一片 2×22×2 大小的云，这片云不能到村子以外的地方。

你将获得一段时间内村子每个区域的赶集和过节时间表。

在这段时间的第一天，中部地区（6−7−10−116−7−10−11）将会下雨。

在接下来的每一天中，您可以在四个基本方向（东南西北）之中选取一个方向，将云移动 11 或 22 个方格，或将其保持在相同位置。

不允许对角线移动，所有动作都在一天开始时发生。

任何地区都不能连续七天或以上时间都不降雨。

这段时间以外的日子的下雨状况你无需做任何考虑。

#### 输入格式

输入包含多组测试用例。

对于每组测试用例，第一行包含一个整数 NN，表示这段时间的具体天数。

接下里 NN 行，描绘了接下来 NN 天的赶集和过节时间表，第 ii 行表示第 ii 天的时间表。

这 NN 行里，每行包含 1616 个数字（00 或 11），00 表示正常的一天，11 表示赶集和过节的一天，第 ii 个数字表示第 ii 个区域的具体情况。

每行数字之间用空格隔开。

当输入测试用例 N\=0N\=0 时，表示输入终止，且该用例无需处理。

#### 输出格式

每个测试用例输出一个整数 00 或 11，如果可以保证整个时间段内，该下雨的地方下雨，不该下的地方不下，则输出 11。

如果不能保证则输出 00，每个结果占一行。

#### 数据范围

1≤N≤3651≤N≤365

#### 输入样例：

    1
    0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0
    7
    0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
    1 0 0 0 0 0 1 0 0 0 0 1 1 0 0 1
    0 0 0 0 0 0 0 0 1 0 0 0 0 1 0 1
    0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0
    0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0
    1 0 0 1 0 0 0 0 0 0 0 0 0 0 0 1
    0 0 0 0 0 1 0 0 1 0 0 0 0 0 0 0
    7
    0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0
    0 0 1 0 0 0 0 1 0 0 0 0 0 1 0 0
    0 0 0 1 0 0 0 0 0 0 1 0 1 0 0 0
    0 1 0 0 0 0 0 1 0 0 0 0 1 0 0 0
    0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 1 1 0 1 0 0 0 0 1
    0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0
    15
    0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0
    0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 1 1 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 1 1 0 1 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0
    0 0 1 1 0 0 0 0 0 1 0 0 0 0 0 0
    1 1 0 0 0 0 0 0 0 0 1 0 0 1 0 0
    0 0 0 0 0 1 0 0 0 0 0 1 0 0 0 0
    0 0 1 0 0 0 0 0 0 0 0 0 0 0 1 0
    1 0 0 1 1 0 0 0 0 1 0 1 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0
    0 0 0 0 0 1 0 1 0 1 0 0 0 0 0 0
    0
    

输出样例：

    0
    1
    0
    1
    

难度：中等

时/空限制：1s / 64MB

总通过数：943

总尝试数：2247

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3742&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3742&show_algorithm_tags=1)[广度优先搜索](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%BF%E5%BA%A6%E4%BC%98%E5%85%88%E6%90%9C%E7%B4%A2&source_file_id=3742&show_algorithm_tags=1)[BFS](https://www.acwing.com/problem/search/1/?search_content=BFS&source_file_id=3742&show_algorithm_tags=1)[DP](https://www.acwing.com/problem/search/1/?search_content=DP&source_file_id=3742&show_algorithm_tags=1)[记忆化搜索](https://www.acwing.com/problem/search/1/?search_content=%E8%AE%B0%E5%BF%86%E5%8C%96%E6%90%9C%E7%B4%A2&source_file_id=3742&show_algorithm_tags=1)