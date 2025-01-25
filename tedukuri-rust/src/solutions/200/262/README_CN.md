262\. 海报

*    [题目](https://www.acwing.com/problem/content/description/264/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/264/1/)
*    [题解](https://www.acwing.com/problem/content/solution/264/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/264/)

  

墙上粘贴了 nn 个相同形状的矩形海报。

它们的边都是垂直或水平的。

每个矩形可以被其他矩形部分或完全覆盖。

所有矩形的并集边界的长度称为周长。

现在请你编程计算这个周长是多少。

图 11 显示了一个包含 77 个矩形的图形样例：

![1177_1.jpg](https://cdn.acwing.com/media/article/image/2019/01/21/19_aa9258341c-1177_1.jpg)

图 22 给出了它的并集边界：

![1177_2.jpg](https://cdn.acwing.com/media/article/image/2019/01/21/19_ce0c08f01c-1177_2.jpg)

每个矩形的顶点都有一个整数坐标。

#### 输入格式

第一行输入整数 nn，表示矩形的数量。

接下来 nn 行，每行四个整数 x1,y1,x2,y2x1,y1,x2,y2 用以描述一个矩形，(x1,y1)(x1,y1) 为矩形的左下角坐标，(x2,y2)(x2,y2) 为矩形的右上角坐标。

#### 输出格式

输出一个整数，表示矩形并集的周长。

#### 数据范围

0≤n<50000≤n<5000,  
−10000≤xi,yi≤10000−10000≤xi,yi≤10000

#### 输入样例：

    7
    -15 0 5 10
    -5 8 20 25
    15 -4 24 14
    0 -6 16 4
    2 15 10 22
    30 10 36 20
    34 0 40 16
    

#### 输出样例：

    228
    

难度：简单

时/空限制：1s / 10MB

总通过数：777

总尝试数：1613

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3813&show_algorithm_tags=0)[usaco training 5.5](https://www.acwing.com/problem/search/1/?search_content=usaco%20training%205.5&source_file_id=3813&show_algorithm_tags=0)[POJ1177](https://www.acwing.com/problem/search/1/?search_content=POJ1177&source_file_id=3813&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3813&show_algorithm_tags=0)

算法标签

[扫描线](https://www.acwing.com/problem/search/1/?search_content=%E6%89%AB%E6%8F%8F%E7%BA%BF&source_file_id=3813&show_algorithm_tags=1)