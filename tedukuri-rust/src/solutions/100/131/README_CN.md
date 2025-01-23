131\. 直方图中最大的矩形

*    [题目](https://www.acwing.com/problem/content/description/133/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/133/1/)
*    [题解](https://www.acwing.com/problem/content/solution/133/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/133/)

  

直方图是由在公共基线处对齐的一系列矩形组成的多边形。

矩形具有相等的宽度，但可以具有不同的高度。

例如，图例左侧显示了由高度为 2,1,4,5,1,3,32,1,4,5,1,3,3 的矩形组成的直方图，矩形的宽度都为 11：

![2559_1.jpg](https://cdn.acwing.com/media/article/image/2019/01/14/19_eac6c46017-2559_1.jpg)

通常，直方图用于表示离散分布，例如，文本中字符的频率。

现在，请你计算在公共基线处对齐的直方图中最大矩形的面积。

图例右图显示了所描绘直方图的最大对齐矩形。

#### 输入格式

输入包含几个测试用例。

每个测试用例占据一行，用以描述一个直方图，并以整数 nn 开始，表示组成直方图的矩形数目。

然后跟随 nn 个整数 h1，…，hnh1，…，hn。

这些数字以从左到右的顺序表示直方图的各个矩形的高度。

每个矩形的宽度为 11。

同行数字用空格隔开。

当输入用例为 n\=0n\=0 时，结束输入，且该用例不用考虑。

#### 输出格式

对于每一个测试用例，输出一个整数，代表指定直方图中最大矩形的区域面积。

每个数据占一行。

请注意，此矩形必须在公共基线处对齐。

#### 数据范围

1≤n≤1000001≤n≤100000,  
0≤hi≤10000000000≤hi≤1000000000

#### 输入样例：

    7 2 1 4 5 1 3 3
    4 1000 1000 1000 1000
    0
    

#### 输出样例：

    8
    4000
    

难度：简单

时/空限制：1s / 64MB

总通过数：11962

总尝试数：27115

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3682&show_algorithm_tags=0)[HDU1506](https://www.acwing.com/problem/search/1/?search_content=HDU1506&source_file_id=3682&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3682&show_algorithm_tags=0)

算法标签

[单调栈](https://www.acwing.com/problem/search/1/?search_content=%E5%8D%95%E8%B0%83%E6%A0%88&source_file_id=3682&show_algorithm_tags=1)