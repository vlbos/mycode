239\. 奇偶游戏

*    [题目](https://www.acwing.com/problem/content/description/241/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/241/1/)
*    [题解](https://www.acwing.com/problem/content/solution/241/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/241/)

  

小 AA 和小 BB 在玩一个游戏。

首先，小 AA 写了一个由 00 和 11 组成的序列 SS，长度为 NN。

然后，小 BB 向小 AA 提出了 MM 个问题。

在每个问题中，小 BB 指定两个数 ll 和 rr，小 AA 回答 S\[l∼r\]S\[l∼r\] 中有奇数个 11 还是偶数个 11。

机智的小 BB 发现小 AA 有可能在撒谎。

例如，小 AA 曾经回答过 S\[1∼3\]S\[1∼3\] 中有奇数个 11，S\[4∼6\]S\[4∼6\] 中有偶数个 11，现在又回答 S\[1∼6\]S\[1∼6\] 中有偶数个 11，显然这是自相矛盾的。

请你帮助小 BB 检查这 MM 个答案，并指出在至少多少个回答之后可以确定小 AA 一定在撒谎。

即求出一个最小的 kk，使得 0101 序列 SS 满足第 1∼k1∼k 个回答，但不满足第 1∼k+11∼k+1 个回答。

#### 输入格式

第一行包含一个整数 NN，表示 0101 序列长度。

第二行包含一个整数 MM，表示问题数量。

接下来 MM 行，每行包含一组问答：两个整数 ll 和 rr，以及回答 `even` 或 `odd`，用以描述 S\[l∼r\]S\[l∼r\] 中有偶数个 11 还是奇数个 11。

#### 输出格式

输出一个整数 kk，表示 0101 序列满足第 1∼k1∼k 个回答，但不满足第 1∼k+11∼k+1 个回答，如果 0101 序列满足所有回答，则输出问题总数量。

#### 数据范围

N≤109,M≤5000N≤109,M≤5000

#### 输入样例：

    10
    5
    1 2 even
    3 4 odd
    5 6 even
    1 6 even
    7 10 odd
    

#### 输出样例：

    3
    

难度：中等

时/空限制：1s / 64MB

总通过数：15020

总尝试数：29510

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3790&show_algorithm_tags=0)[POJ1733](https://www.acwing.com/problem/search/1/?search_content=POJ1733&source_file_id=3790&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3790&show_algorithm_tags=0)

算法标签

[并查集](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%B6%E6%9F%A5%E9%9B%86&source_file_id=3790&show_algorithm_tags=1)