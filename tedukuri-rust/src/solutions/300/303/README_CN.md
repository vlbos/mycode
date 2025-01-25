303\. 运输小猫

*    [题目](https://www.acwing.com/problem/content/description/305/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/305/1/)
*    [题解](https://www.acwing.com/problem/content/solution/305/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/305/)

  

小 SS 是农场主，他养了 MM 只猫，雇了 PP 位饲养员。

农场中有一条笔直的路，路边有 NN 座山，从 11 到 NN 编号。

第 ii 座山与第 i−1i−1 座山之间的距离为 DiDi。

饲养员都住在 11 号山。

有一天，猫出去玩。

第 ii 只猫去 HiHi 号山玩，玩到时刻 TiTi 停止，然后在原地等饲养员来接。

饲养员们必须回收所有的猫。

每个饲养员沿着路从 11 号山走到 NN 号山，把各座山上已经在等待的猫全部接走。

饲养员在路上行走需要时间，速度为 11 米/单位时间。

饲养员在每座山上接猫的时间可以忽略，可以携带的猫的数量为无穷大。

例如有两座相距为 11 的山，一只猫在 22 号山玩，玩到时刻 33 开始等待。

如果饲养员从 11 号山在时刻 22 或 33 出发，那么他可以接到猫，猫的等待时间为 00 或 11。

而如果他于时刻 11 出发，那么他将于时刻 22 经过 22 号山，不能接到当时仍在玩的猫。

你的任务是规划每个饲养员从 11 号山出发的时间，使得所有猫等待时间的总和尽量小。

饲养员出发的时间可以为负。

#### 输入格式

第一行包含三个整数 N，M，PN，M，P。

第二行包含 n−1n−1 个整数，D2,D3,…,DND2,D3,…,DN。

接下来 MM 行，每行包含两个整数 HiHi 和 TiTi。

#### 输出格式

输出一个整数，表示所有猫等待时间的总和的最小值。

#### 数据范围

2≤N≤1052≤N≤105,  
1≤M≤1051≤M≤105,  
1≤P≤1001≤P≤100,  
1≤Di<100001≤Di<10000,  
1≤Hi≤N1≤Hi≤N,  
0≤Ti≤1090≤Ti≤109

#### 输入样例：

    4 6 2
    1 3 5
    1 0
    2 1
    4 9
    1 10
    2 10
    3 12
    

#### 输出样例：

    3
    

难度：困难

时/空限制：2s / 256MB

总通过数：3962

总尝试数：8897

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3854&show_algorithm_tags=0)[CF311B](https://www.acwing.com/problem/search/1/?search_content=CF311B&source_file_id=3854&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3854&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3854&show_algorithm_tags=1)[斜率优化](https://www.acwing.com/problem/search/1/?search_content=%E6%96%9C%E7%8E%87%E4%BC%98%E5%8C%96&source_file_id=3854&show_algorithm_tags=1)