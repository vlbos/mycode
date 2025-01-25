374\. 导弹防御塔

*    [题目](https://www.acwing.com/problem/content/description/376/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/376/1/)
*    [题解](https://www.acwing.com/problem/content/solution/376/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/376/)

  

Freda 的城堡遭受了 MM 个入侵者的攻击！

Freda 控制着 NN 座导弹防御塔，每座塔都有足够数量的导弹，但是每次只能发射一枚。

在发射导弹时，导弹需要 T1T1 秒才能从防御塔中射出，而在发射导弹后，发射这枚导弹的防御塔需要 T2T2 分钟来冷却。

所有导弹都有相同的匀速飞行速度 VV，并且会沿着距离最短的路径去打击目标。

计算防御塔到目标的距离 DistanceDistance 时，你只需要计算水平距离，而忽略导弹飞行的高度。

导弹在空中飞行的时间就是 (Distance/VDistance/V) 分钟，导弹到达目标后可以立即将它击毁。

现在，给出 NN 座导弹防御塔的坐标，MM 个入侵者的坐标，T1,T2T1,T2 和 VV。

因为 Freda 的小伙伴 Rainbow 就要来拜访城堡了，你需要求出至少多少分钟才能击退所有的入侵者。

#### 输入格式

第一行五个正整数 N,M,T1,T2,VN,M,T1,T2,V。

接下来 MM 行每行两个整数，代表入侵者的坐标。

接下来 NN 行每行两个整数，代表防御塔的坐标。

#### 输出格式

输出一个实数，表示最少需要多少分钟才能击中所有的入侵者，四舍五入保留六位小数。

#### 数据范围

1≤N,M≤501≤N,M≤50,坐标绝对值不超过 1000010000，T1,T2,VT1,T2,V 不超过 20002000。

#### 输入样例：

    3 3 30 20 1
    0 0
    0 50
    50 0
    50 50
    0 1000
    1000 0
    

#### 输出样例：

    91.500000
    

难度：中等

时/空限制：1s / 64MB

总通过数：1008

总尝试数：2669

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3925&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3925&show_algorithm_tags=1)[二分图最大匹配](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86%E5%9B%BE%E6%9C%80%E5%A4%A7%E5%8C%B9%E9%85%8D&source_file_id=3925&show_algorithm_tags=1)