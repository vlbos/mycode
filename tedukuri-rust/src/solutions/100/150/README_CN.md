150\. 括号画家

*    [题目](https://www.acwing.com/problem/content/description/152/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/152/1/)
*    [题解](https://www.acwing.com/problem/content/solution/152/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/152/)

  

达达是一名漫画家，她有一个奇特的爱好，就是在纸上画括号。

这一天，刚刚起床的达达画了一排括号序列，其中包含小括号 `( )`、中括号 `[ ]` 和大括号 `{ }`，总长度为 NN。

这排随意绘制的括号序列显得杂乱无章，于是达达定义了什么样的括号序列是美观的：

1.  空的括号序列是美观的；
2.  若括号序列 AA 是美观的，则括号序列 (A)(A)、\[A\]\[A\]、{A}{A} 也是美观的；
3.  若括号序列 A、BA、B 都是美观的，则括号序列 ABAB 也是美观的。

例如 `[(){}]()` 是美观的括号序列，而`)({)[}](` 则不是。

现在达达想在她绘制的括号序列中，找出其中连续的一段，满足这段子串是美观的，并且长度尽量大。

你能帮帮她吗？

#### 输入格式

输入一行由括号组成的字符串。

#### 输出格式

输出一个整数，表示最长的美观的子段的长度。

#### 数据范围

字符串长度不超过 105105。

#### 输入样例：

    ({({(({()}})}{())})})[){{{([)()((()]]}])[{)]}{[}{)
    

#### 输出样例：

    4
    

难度：简单

时/空限制：1s / 64MB

总通过数：3153

总尝试数：8986

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3701&show_algorithm_tags=0)

算法标签

[栈](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%88&source_file_id=3701&show_algorithm_tags=1)