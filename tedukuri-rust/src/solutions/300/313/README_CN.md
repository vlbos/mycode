313\. 花店橱窗

*    [题目](https://www.acwing.com/problem/content/description/315/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/315/1/)
*    [题解](https://www.acwing.com/problem/content/solution/315/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/315/)

  

小 qq 和他的老婆小 zz 最近开了一家花店，他们准备把店里最好看的花都摆在橱窗里。

但是他们有很多花瓶，每个花瓶都具有各自的特点，因此，当各个花瓶中放入不同的花束时，会产生不同的美学效果。

为了使橱窗里的花摆放的最合适，他们得想个办法安排每种花的摆放位置。

可是因为小 qq 和小 zz 每天都太忙，没有时间设计橱窗里花的摆法，所以他们想让你帮他们求出花摆放的最大美观程度和每种花所放的位置。

每种花都有一个标识，假设杜鹃花的标识数为 11，秋海棠的标识数为 22，康乃馨的标识数为 33，所有的花束在放入花瓶时必须保持其标识数的顺序，即：

杜鹃花必须放在秋海棠左边的花瓶中，秋海棠必须放在康乃馨左边的花瓶中。

如果花瓶的数目大于花束的数目。则多余的花瓶必须空置，且每个花瓶中只能放一束花。

每种花放在不同的瓶子里会产生不同的美观程度，美观程度可能是正数也可能是负数。

上述例子中，花瓶与花束的不同搭配所具有的美观程度，如下表所示：

                             花    瓶
                      1     2    3    4    5
       1 (杜鹃花)     7    23   -5  -24   16
       2 (秋海棠)     5    21   -4   10   23
       3 (康乃馨)    -21    5   -4  -20   20
    

根据上表，杜鹃花放在花瓶 22 中，会显得非常好看；但若放在花瓶 44 中则显得十分难看。

为取得最大美观程度，你必须在保持花束顺序的前提下，使花束的摆放取得最大的美学值，并求出每种花应该摆放的花瓶的编号。

#### 输入格式

第 11 行：两个整数 FF 和 VV，表示共有 FF 种花，VV 个花瓶。

第 22 行到第 F+1F+1 行：每行有 VV 个数，表示花摆放在不同花瓶里的美观程度值 valuevalue。(美观程度和小于231231，美观程度有正有负)

#### 输出格式

输出有两行：第一行为输出最大美观程度和的值，第二行有 FF 个数表示每朵花应该摆放的花瓶的编号。

若有多种方案，输出字典序较小的方案（美观程度不变的情况下，花尽量往前放）。

#### 数据范围

1≤F≤V≤1001≤F≤V≤100,

#### 输入样例：

    3 5 
    7 23 -5 -24 16
    5 21 -4 10 23
    -21 5 -4 -20 20
    

#### 输出样例：

    53
    2 4 5
    

难度：简单

时/空限制：1s / 64MB

总通过数：1785

总尝试数：4722

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3864&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3864&show_algorithm_tags=1)[线性DP](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%80%A7DP&source_file_id=3864&show_algorithm_tags=1)[输出方案](https://www.acwing.com/problem/search/1/?search_content=%E8%BE%93%E5%87%BA%E6%96%B9%E6%A1%88&source_file_id=3864&show_algorithm_tags=1)