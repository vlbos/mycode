91\. 最短Hamilton路径

*    [题目](https://www.acwing.com/problem/content/description/93/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/93/1/)
*    [题解](https://www.acwing.com/problem/content/solution/93/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/93/)

  

给定一张 nn 个点的带权无向图，点从 0∼n−10∼n−1 标号，求起点 00 到终点 n−1n−1 的最短 Hamilton 路径。

Hamilton 路径的定义是从 00 到 n−1n−1 不重不漏地经过每个点恰好一次。

#### 输入格式

第一行输入整数 nn。

接下来 nn 行每行 nn 个整数，其中第 ii 行第 jj 个整数表示点 ii 到 jj 的距离（记为 a\[i,j\]a\[i,j\]）。

对于任意的 x,y,zx,y,z，数据保证 a\[x,x\]\=0，a\[x,y\]\=a\[y,x\]a\[x,x\]\=0，a\[x,y\]\=a\[y,x\] 并且 a\[x,y\]+a\[y,z\]≥a\[x,z\]a\[x,y\]+a\[y,z\]≥a\[x,z\]。

#### 输出格式

输出一个整数，表示最短 Hamilton 路径的长度。

#### 数据范围

1≤n≤201≤n≤20  
0≤a\[i,j\]≤1070≤a\[i,j\]≤107

#### 输入样例：

    5
    0 2 4 5 1
    2 0 6 5 3
    4 6 0 8 3
    5 5 8 0 5
    1 3 3 5 0
    

#### 输出样例：

    18
    

难度：中等

时/空限制：5s / 256MB

总通过数：49826

总尝试数：64764

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3642&show_algorithm_tags=0)[模板题](https://www.acwing.com/problem/search/1/?search_content=%E6%A8%A1%E6%9D%BF%E9%A2%98&source_file_id=3642&show_algorithm_tags=0)

算法标签

[二进制](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E8%BF%9B%E5%88%B6&source_file_id=3642&show_algorithm_tags=1)[状态压缩DP](https://www.acwing.com/problem/search/1/?search_content=%E7%8A%B6%E6%80%81%E5%8E%8B%E7%BC%A9DP&source_file_id=3642&show_algorithm_tags=1)