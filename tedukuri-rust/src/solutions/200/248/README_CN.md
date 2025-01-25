248\. 窗内的星星

*    [题目](https://www.acwing.com/problem/content/description/250/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/250/1/)
*    [题解](https://www.acwing.com/problem/content/solution/250/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/250/)

  

在一个天空中有很多星星（看作平面直角坐标系），已知每颗星星的坐标和亮度（都是整数）。

求用宽为 WW、高为 HH 的矩形窗口（W,HW,H 为正整数）能圈住的星星的亮度总和最大是多少。（矩形边界上的星星不算）

#### 输入格式

输入包含多组测试用例。

每个用例的第一行包含 33 个整数：n，W，Hn，W，H，表示星星的数量，矩形窗口的宽和高。

然后是 nn 行，每行有 33 个整数：x，y，cx，y，c，表示每个星星的位置 (x，y)(x，y) 和亮度。

没有两颗星星在同一点上。

#### 输出格式

每个测试用例输出一个亮度总和最大值。

每个结果占一行。

#### 数据范围

1≤n≤100001≤n≤10000,  
1≤W,H≤10000001≤W,H≤1000000,  
0≤x,y<2310≤x,y<231,  
亮度取值范围 \[1,1000\]\[1,1000\]。

#### 输入样例：

    3 5 4
    1 2 3
    2 3 2
    6 3 1
    3 5 4
    1 2 3
    2 3 2
    5 3 1
    

#### 输出样例：

    5
    6
    

难度：中等

时/空限制：1s / 64MB

总通过数：2091

总尝试数：7247

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3799&show_algorithm_tags=0)

算法标签

[线段树](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%AE%B5%E6%A0%91&source_file_id=3799&show_algorithm_tags=1)[扫描线](https://www.acwing.com/problem/search/1/?search_content=%E6%89%AB%E6%8F%8F%E7%BA%BF&source_file_id=3799&show_algorithm_tags=1)