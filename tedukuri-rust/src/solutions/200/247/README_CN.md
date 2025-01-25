247\. 亚特兰蒂斯

*    [题目](https://www.acwing.com/problem/content/description/249/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/249/1/)
*    [题解](https://www.acwing.com/problem/content/solution/249/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/249/)

  

有几个古希腊书籍中包含了对传说中的亚特兰蒂斯岛的描述。

其中一些甚至包括岛屿部分地图。

但不幸的是，这些地图描述了亚特兰蒂斯的不同区域。

您的朋友 Bill 必须知道地图的总面积。

你自告奋勇写了一个计算这个总面积的程序。

#### 输入格式

输入包含多组测试用例。

对于每组测试用例，第一行包含整数 nn，表示总的地图数量。

接下来 nn 行，描绘了每张地图，每行包含四个数字 x1,y1,x2,y2x1,y1,x2,y2（不一定是整数），(x1,y1)(x1,y1) 和 (x2,y2)(x2,y2) 分别是地图的左上角位置和右下角位置。

注意，坐标轴 xx 轴从上向下延伸，yy 轴从左向右延伸。

当输入用例 n\=0n\=0 时，表示输入终止，该用例无需处理。

#### 输出格式

每组测试用例输出两行。

第一行输出 `Test case #k`，其中 kk 是测试用例的编号，从 11 开始。

第二行输出 `Total explored area: a`，其中 aa 是总地图面积（即此测试用例中所有矩形的面积并，注意如果一片区域被多个地图包含，则在计算总面积时只计算一次），精确到小数点后两位数。

在每个测试用例后输出一个空行。

#### 数据范围

1≤n≤100001≤n≤10000,  
0≤x1<x2≤1000000≤x1<x2≤100000,  
0≤y1<y2≤1000000≤y1<y2≤100000  
注意，本题 nn 的范围上限加强至 1000010000。

#### 输入样例：

    2
    10 10 20 20
    15 15 25 25.5
    0
    

#### 输出样例：

    Test case #1
    Total explored area: 180.00 
    
    

#### 样例解释

样例所示地图覆盖区域如下图所示，两个矩形区域所覆盖的总面积，即为样例的解。

![无标题.png](https://cdn.acwing.com/media/article/image/2019/12/26/19_4acba44c27-%E6%97%A0%E6%A0%87%E9%A2%98.png)

难度：困难

时/空限制：1s / 64MB

总通过数：10541

总尝试数：27087

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3798&show_algorithm_tags=0)[HDU1542](https://www.acwing.com/problem/search/1/?search_content=HDU1542&source_file_id=3798&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3798&show_algorithm_tags=0)

算法标签

[线段树](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%AE%B5%E6%A0%91&source_file_id=3798&show_algorithm_tags=1)[扫描线](https://www.acwing.com/problem/search/1/?search_content=%E6%89%AB%E6%8F%8F%E7%BA%BF&source_file_id=3798&show_algorithm_tags=1)