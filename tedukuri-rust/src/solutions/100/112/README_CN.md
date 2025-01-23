112\. 雷达设备

*    [题目](https://www.acwing.com/problem/content/description/114/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/114/1/)
*    [题解](https://www.acwing.com/problem/content/solution/114/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/114/)

  

假设海岸是一条无限长的直线，陆地位于海岸的一侧，海洋位于另外一侧。

每个小岛都位于海洋一侧的某个点上。

雷达装置均位于海岸线上，且雷达的监测范围为 dd，当小岛与某雷达的距离不超过 dd 时，该小岛可以被雷达覆盖。

我们使用笛卡尔坐标系，定义海岸线为 xx 轴，海的一侧在 xx 轴上方，陆地一侧在 xx 轴下方。

现在给出每个小岛的具体坐标以及雷达的检测范围，请你求出能够使所有小岛都被雷达覆盖所需的最小雷达数目。

#### 输入格式

第一行输入两个整数 nn 和 dd，分别代表小岛数目和雷达检测范围。

接下来 nn 行，每行输入两个整数，分别代表小岛的 x，yx，y 轴坐标。

同一行数据之间用空格隔开。

#### 输出格式

输出一个整数，代表所需的最小雷达数目，若没有解决方案则所需数目输出 −1−1。

#### 数据范围

1≤n≤10001≤n≤1000,  
1≤d≤2001≤d≤200,  
−1000≤x,y≤1000−1000≤x,y≤1000

#### 输入样例：

    3 2
    1 2
    -3 1
    2 1
    

#### 输出样例：

    2
    

难度：中等

时/空限制：1s / 64MB

总通过数：13256

总尝试数：34985

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3663&show_algorithm_tags=0)

算法标签

[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3663&show_algorithm_tags=1)