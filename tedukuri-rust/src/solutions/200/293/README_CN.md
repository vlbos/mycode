293\. 开车旅行

*    [题目](https://www.acwing.com/problem/content/description/295/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/295/1/)
*    [题解](https://www.acwing.com/problem/content/solution/295/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/295/)

  

小 AA 和小 BB 决定利用假期外出旅行，他们将想去的城市从 11 到 NN 编号，且编号较小的城市在编号较大的城市的西边，已知各个城市的海拔高度互不相同，记城市 ii 的海拔高度为 HiHi。

城市 ii 和城市 jj 之间的距离 d\[i,j\]d\[i,j\] 恰好是这两个城市海拔高度之差的绝对值，即 d\[i,j\]\=|Hi−Hj|d\[i,j\]\=|Hi−Hj|。

旅行过程中，小 AA 和小 BB 轮流开车，第一天小 AA 开车，之后每天轮换一次。

每个人每天均会从一个城市出发走到另一个城市。

他们计划选择一个城市 SS 作为起点，一直向东行驶，并且最多行驶 XX 公里就结束旅行。

小 AA 和小 BB 的驾驶风格不同，小 BB 总是沿着前进方向选择一个最近的城市作为目的地，而小 AA 总是沿着前进方向选择第二近的城市作为目的地（注意：本题中如果当前城市到两个城市的距离相同，则认为离海拔低的那个城市更近）。

如果其中任何一人无法按照自己的原则选择目的城市，或者到达目的地会使行驶的总距离超出 XX 公里，他们就会结束旅行。

在启程之前，小 AA 想知道两个问题：

1.  对于一个给定的 X\=X0X\=X0，从哪一个城市出发，小 AA 开车行驶的路程总数与小 BB 行驶的路程总数的比值最小（如果小 BB 的行驶路程为 00，此时的比值可视为无穷大，且两个无穷大视为相等）。如果从多个城市出发，小 AA 开车行驶的路程总数与小 BB 行驶的路程总数的比值都最小，则输出海拔最高的那个城市。
2.  对任意给定的 X\=XiX\=Xi 和出发城市 SiSi，求出小 AA 开车行驶的路程总数以及小 BB 行驶的路程总数。

#### 输入格式

第一行包含一个整数 NN，表示城市的数目。

第二行有 NN 个整数，每两个整数之间用一个空格隔开，依次表示城市 11 到城市 NN 的海拔高度，即 H1,H2,…,HNH1,H2,…,HN，且每个 HiHi 都是不同的。

第三行包含一个整数 X0X0。

第四行为一个整数 MM，表示给定 MM 组 SiSi 和 XiXi。

接下来的 MM 行，每行包含 22 个整数 SiSi 和 XiXi，表示从城市 SiSi 出发，最多行驶 XiXi 公里。

#### 输出格式

输出共 M+1M+1 行。

第一行包含一个整数 S0S0，表示对于给定的 X0X0，从编号为 S0S0 的城市出发，小 AA 开车行驶的路程总数与小 BB 行驶的路程总数的比值最小。

接下来的 MM 行，每行包含 22 个整数，之间用一个空格隔开，依次表示在给定的 SiSi 和 XiXi 下小 AA 行驶的里程总数和小 BB 行驶的里程总数。

#### 数据范围

1≤N≤1051≤N≤105,  
1≤M≤1041≤M≤104,  
−109≤Hi≤109−109≤Hi≤109,  
0≤X0≤1090≤X0≤109,  
1≤Si≤N1≤Si≤N,  
0≤Xi≤1090≤Xi≤109，  
数据保证HiHi互不相同。

#### 输入样例：

    10
    4 5 6 1 2 3 7 8 9 10
    7
    10
    1 7
    2 7
    3 7
    4 7
    5 7
    6 7
    7 7
    8 7
    9 7
    10 7
    

#### 输出样例：

    2
    3 2
    2 4
    2 1
    2 4
    5 1
    5 1
    2 1
    2 0
    0 0
    0 0
    

难度：困难

时/空限制：1s / 128MB

总通过数：1371

总尝试数：3258

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3844&show_algorithm_tags=0)[NOIP2012提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2012%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3844&show_algorithm_tags=0)

算法标签

[DP](https://www.acwing.com/problem/search/1/?search_content=DP&source_file_id=3844&show_algorithm_tags=1)[倍增优化DP](https://www.acwing.com/problem/search/1/?search_content=%E5%80%8D%E5%A2%9E%E4%BC%98%E5%8C%96DP&source_file_id=3844&show_algorithm_tags=1)