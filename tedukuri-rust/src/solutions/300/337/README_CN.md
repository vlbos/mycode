337\. 扑克牌

*    [题目](https://www.acwing.com/problem/content/description/339/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/339/1/)
*    [题解](https://www.acwing.com/problem/content/solution/339/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/339/)

  

一副不含王的扑克牌由 5252 张牌组成，由红桃、黑桃、梅花、方块 44 组牌组成，每组 1313 张不同的面值。

现在给定 5252 张牌中的若干张，请计算将它们排成一列，相邻的牌面值不同的方案数。

牌的表示方法为 XYXY，其中 XX 为面值，为 2、3、4、5、6、7、8、9、T、J、Q、K、A2、3、4、5、6、7、8、9、T、J、Q、K、A 中的一个。

YY 为花色，为 S、H、D、CS、H、D、C 中的一个。

如 2S、2H、TD2S、2H、TD 等。

#### 输入格式

第一行为一个整数 TT，表示共有 TT 组测试数据。

之后每组数据占一行。

这一行首先包含一个整数 NN，表示给定的牌的张数，接下来 NN 个由空格分隔的字符串，每个字符串长度为 22，表示一张牌。

每组数据中的扑克牌各不相同。

#### 输出格式

对于每组数据输出一行，形如 `Case #X: Y`，XX 为数据组数，从 11 开始，YY 为可能的方案数。

由于答案可能很大，请输出对 264264 取模之后的值。

#### 数据范围

1≤T≤200001≤T≤20000,  
1≤N≤521≤N≤52

#### 输入样例：

    5
    1 TC
    2 TC TS
    5 2C AD AC JC JH
    4 AC KC QC JC
    6 AC AD AS JC JD KD
    

#### 输出样例：

    Case #1: 1
    Case #2: 0
    Case #3: 48
    Case #4: 24
    Case #5: 120
    

难度：困难

时/空限制：1s / 64MB

总通过数：332

总尝试数：732

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3888&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3888&show_algorithm_tags=1)[计数类DP](https://www.acwing.com/problem/search/1/?search_content=%E8%AE%A1%E6%95%B0%E7%B1%BBDP&source_file_id=3888&show_algorithm_tags=1)