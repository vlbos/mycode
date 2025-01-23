94\. 递归实现排列型枚举

*    [题目](https://www.acwing.com/problem/content/description/96/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/96/1/)
*    [题解](https://www.acwing.com/problem/content/solution/96/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/96/)

  

把 1∼n1∼n 这 nn 个整数排成一行后随机打乱顺序，输出所有可能的次序。

#### 输入格式

一个整数 nn。

#### 输出格式

按照从小到大的顺序输出所有方案，每行 11 个。

首先，同一行相邻两个数用一个空格隔开。

其次，对于两个不同的行，对应下标的数一一比较，字典序较小的排在前面。

#### 数据范围

1≤n≤91≤n≤9

#### 输入样例：

    3
    

#### 输出样例：

    1 2 3
    1 3 2
    2 1 3
    2 3 1
    3 1 2
    3 2 1
    

难度：简单

时/空限制：5s / 256MB

总通过数：55474

总尝试数：73354

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3645&show_algorithm_tags=0)

算法标签

[递归](https://www.acwing.com/problem/search/1/?search_content=%E9%80%92%E5%BD%92&source_file_id=3645&show_algorithm_tags=1)