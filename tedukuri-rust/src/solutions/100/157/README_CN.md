157\. 树形地铁系统

*    [题目](https://www.acwing.com/problem/content/description/159/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/159/1/)
*    [题解](https://www.acwing.com/problem/content/solution/159/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/159/)

  

一些主要城市拥有树形的地铁系统，即在任何一对车站之间，有且只有一种方式可以乘坐地铁。

此外，这些城市大多数都有一个中央车站。

想象一下，你是一名在拥有树形地铁系统的城市游玩的游客，你想探索该城市完整的地铁线路。

你从中央车站出发，随机选择一条地铁线，然后乘坐地铁行进。

每次到达一个车站，你都将选择一条尚未乘坐过的地铁线路进行乘坐。

如果不存在未乘坐过的线路，则退回到上一个车站，再做选择。

直到你将所有地铁线路都乘坐过两次（往返各一次），此时你将回到中央车站。

之后，你以一种特殊的方式回忆自己的坐车过程，你将你的完整地铁乘坐路线编码为一个二进制字符串。

其中 00 编码表示你乘坐地铁线路到达距离中央车站更远的一站，11 编码表示你乘坐地铁线路到达距离中央车站更近的一站。

![subway.jpg](https://cdn.acwing.com/media/article/image/2019/01/15/19_2d7336ca18-subway.jpg)

#### 输入格式

第一行输入一个正整数 nn，代表测试用例数量。

每个测试用例由两行组成，每行输入一个由字符 00 和 11 构成的字符串，长度最多为 30003000， 两个字符串都描述了一种树形地铁系统的正确探索路线。

#### 输出格式

对于每个测试用例，如果两个字符串描述的探索路线可以视为同一个地铁系统的两种探索路线，则输出 `same`。

否则，输出 `different`。

每行输出一个结果。

#### 输入样例：

    2
    0010011101001011
    0100011011001011
    0100101100100111
    0011000111010101
    

#### 输出样例：

    same
    different
    

难度：简单

时/空限制：1s / 64MB

总通过数：1586

总尝试数：2453

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3708&show_algorithm_tags=0)

算法标签

[哈希](https://www.acwing.com/problem/search/1/?search_content=%E5%93%88%E5%B8%8C&source_file_id=3708&show_algorithm_tags=1)[树哈希](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%91%E5%93%88%E5%B8%8C&source_file_id=3708&show_algorithm_tags=1)[树的最小表示](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%91%E7%9A%84%E6%9C%80%E5%B0%8F%E8%A1%A8%E7%A4%BA&source_file_id=3708&show_algorithm_tags=1)