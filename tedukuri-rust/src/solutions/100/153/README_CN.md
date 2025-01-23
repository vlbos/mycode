153\. 双栈排序

*    [题目](https://www.acwing.com/problem/content/description/155/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/155/1/)
*    [题解](https://www.acwing.com/problem/content/solution/155/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/155/)

  

Tom 最近在研究一个有趣的排序问题。

通过 22 个栈 S1S1 和 S2S2，Tom 希望借助以下 44 种操作实现将输入序列升序排序。

操作 aa

如果输入序列不为空，将第一个元素压入栈 S1S1

操作 bb

如果栈 S1S1 不为空，将 S1S1 栈顶元素弹出至输出序列

操作 cc

如果输入序列不为空，将第一个元素压入栈 S2S2

操作 dd

如果栈 S2S2 不为空，将 S2S2 栈顶元素弹出至输出序列

如果一个 1∼n1∼n 的排列 PP 可以通过一系列操作使得输出序列为 1,2,…,(n−1),n1,2,…,(n−1),n，Tom 就称 PP 是一个”可双栈排序排列”。

例如 (1,3,2,4)(1,3,2,4) 就是一个”可双栈排序序列”，而 (2,3,4,1)(2,3,4,1) 不是。

下图描述了一个将 (1,3,2,4)(1,3,2,4) 排序的操作序列：`<a, c, c, b, a, d, d, b>`

![untitled.png](https://cdn.acwing.com/media/article/image/2019/01/15/19_6fe3ea3c18-untitled.png)

当然，这样的操作序列有可能有几个，对于上例 (1,3,2,4)(1,3,2,4)，`<a, c, c, b, a, d, d, b>`是另外一个可行的操作序列。

Tom 希望知道其中字典序最小的操作序列是什么。

#### 输入格式

第一行是一个整数 nn。

第二行有 nn 个用空格隔开的正整数，构成一个 1∼n1∼n 的排列。

#### 输出格式

输出共一行，如果输入的排列不是”可双栈排序排列”，输出数字 00。

否则输出字典序最小的操作序列，每两个操作之间用空格隔开，行尾没有空格。

#### 数据范围

1≤n≤10001≤n≤1000

#### 输入样例：

    4
    1 3 2 4
    

#### 输出样例：

    a b a a b b a b
    

难度：困难

时/空限制：1s / 64MB

总通过数：1377

总尝试数：4212

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3704&show_algorithm_tags=0)[NOIP2008提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2008%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3704&show_algorithm_tags=0)

算法标签

[栈](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%88&source_file_id=3704&show_algorithm_tags=1)[二分图](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86%E5%9B%BE&source_file_id=3704&show_algorithm_tags=1)[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3704&show_algorithm_tags=1)