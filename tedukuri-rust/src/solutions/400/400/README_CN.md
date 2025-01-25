400\. 太鼓达人

*    [题目](https://www.acwing.com/problem/content/description/402/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/402/1/)
*    [题解](https://www.acwing.com/problem/content/solution/402/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/402/)

  

太鼓达人的鼓坏了，现在 vani 来修鼓。

鼓的主要元件是 MM 个围成一圈的传感器。

每个传感器都有开和关两种工作状态，分别用 11 和 00 表示。

显然，从不同的位置出发沿顺时针方向连续检查 KK 个传感器可以得到 MM 个长度为 KK 的 0101 串。

Vani 知道这 MM 个 0101 串应该是互不相同的。

而且鼓的设计很精密，MM 会取到可能的最大值。

现在 Vani 已经了解到了 KK 的值，他希望你求出 MM 的值，并给出字典序最小的传感器排布方案。

#### 输入格式

一个整数 KK。

#### 输出格式

一个整数和一个二进制串，由一个空格分隔，分别表示可能的最大的 MM 以及字典序最小的排布方案。

字符 00 表示关，11 表示开，你输出的串的第一个字和最后一个字是相邻的。

#### 数据范围

2≤K≤112≤K≤11

#### 输入样例：

    3
    

#### 输出样例：

    8 00010111
    

难度：简单

时/空限制：1s / 64MB

总通过数：301

总尝试数：464

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3951&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3951&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3951&show_algorithm_tags=1)[欧拉路](https://www.acwing.com/problem/search/1/?search_content=%E6%AC%A7%E6%8B%89%E8%B7%AF&source_file_id=3951&show_algorithm_tags=1)