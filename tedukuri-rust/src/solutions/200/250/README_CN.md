250\. 磁力块

*    [题目](https://www.acwing.com/problem/content/description/252/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/252/1/)
*    [题解](https://www.acwing.com/problem/content/solution/252/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/252/)

  

在一片广袤无垠的原野上，散落着 NN 块磁石。

每个磁石的性质可以用一个五元组 (x,y,m,p,r)(x,y,m,p,r) 描述，其中 x,yx,y 表示其坐标，mm 是磁石的质量，pp 是磁力，rr 是吸引半径。

若磁石 AA 与磁石 BB 的距离不大于磁石 AA 的吸引半径，并且磁石 BB 的质量不大于磁石 AA 的磁力，那么 AA 可以吸引 BB。

小取酒带着一块自己的磁石 LL 来到了这片原野的 (x0,y0)(x0,y0) 处，我们可以视磁石 LL 的坐标为 (x0,y0)(x0,y0)。

小取酒手持磁石 LL 并保持原地不动，所有可以被 LL 吸引的磁石将会被吸引过来。

在每个时刻，他可以选择更换任意一块自己已经获得的磁石（当然也可以是自己最初携带的 LL 磁石）在 (x0,y0)(x0,y0) 处吸引更多的磁石。

小取酒想知道，他最多能获得多少块磁石呢？

#### 输入格式

第一行五个整数 x0,y0,pL,rL,Nx0,y0,pL,rL,N，表示小取酒所在的位置，磁石 LL 磁力、吸引半径和原野上散落磁石的个数。

接下来 NN 行每行五个整数 x,y,m,p,rx,y,m,p,r，描述一块磁石的性质。

#### 输出格式

输出一个整数，表示最多可以获得的散落磁石个数（不包含最初携带的磁石 LL）。

#### 数据范围

1≤N≤2500001≤N≤250000,  
−109≤x,y≤109−109≤x,y≤109,  
1≤p,r≤1091≤p,r≤109,  
0≤m≤1090≤m≤109

#### 输入样例：

    0 0 5 10 5
    5 4 7 11 5
    -7 1 4 7 8
    0 2 13 5 6
    2 -3 9 3 4
    13 5 1 9 9
    

#### 输出样例：

    3
    

难度：困难

时/空限制：2s / 64MB

总通过数：1461

总尝试数：5307

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3801&show_algorithm_tags=0)

算法标签

[分块](https://www.acwing.com/problem/search/1/?search_content=%E5%88%86%E5%9D%97&source_file_id=3801&show_algorithm_tags=1)