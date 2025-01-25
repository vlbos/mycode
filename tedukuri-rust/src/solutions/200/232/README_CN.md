232\. 守卫者的挑战

*    [题目](https://www.acwing.com/problem/content/description/234/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/234/1/)
*    [题解](https://www.acwing.com/problem/content/solution/234/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/234/)

  

打开了黑魔法师 Vani 的大门，队员们在迷宫般的路上漫无目的地搜寻着关押 applepi 的监狱的所在地。

突然，眼前一道亮光闪过，“我，Nizem，是黑魔法圣殿的守卫者。如果你能通过我的挑战，那么你可以带走黑魔法圣殿的地图……”。

瞬间，队员们被传送到了一个擂台上，最初身边有一个容量为 KK 的包包。

擂台赛一共有 NN 项挑战，各项挑战依次进行。

第 ii 项挑战有一个属性 aiai，如果 ai≥0ai≥0，表示这次挑战成功后可以再获得一个容量为 aiai 的包包；如果 ai\=−1ai\=−1，则表示这次挑战成功后可以得到一个大小为 11 的地图残片。

地图残片必须装在包包里才能带出擂台，包包没有必要全部装满，但是队员们必须把获得的所有的地图残片都带走（没有得到的不用考虑，只需要完成所有 NN 项挑战后背包容量足够容纳地图残片即可），才能拼出完整的地图。

并且他们至少要挑战成功 LL 次才能离开擂台。

队员们一筹莫展之时，善良的守卫者 Nizem 帮忙预估出了每项挑战成功的概率，其中第 ii 项挑战成功的概率为 pi%pi%。

现在，请你帮忙预测一下，队员们能够带上他们获得的地图残片离开擂台的概率。

#### 输入格式

第一行三个整数 N,L,KN,L,K。

第二行 NN 个实数，第 ii 个实数 pipi 表示第 ii 项挑战成功的概率的百分比。

第三行 NN 个整数，第 ii 个整数 aiai 表示第 ii 项挑战的属性值。

#### 输出格式

一个实数，表示所求概率，四舍五入保留 66 位小数。

#### 数据范围

0≤K≤20000≤K≤2000,  
0≤N≤2000≤N≤200,  
−1≤ai≤1000−1≤ai≤1000,  
0≤L≤N0≤L≤N,  
0≤pi≤1000≤pi≤100

#### 输入样例：

    3 1 0
    10 20 30
    -1 -1 2
    

#### 输出样例：

    0.300000
    

难度：简单

时/空限制：1s / 256MB

总通过数：634

总尝试数：1269

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3783&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3783&show_algorithm_tags=1)[概率](https://www.acwing.com/problem/search/1/?search_content=%E6%A6%82%E7%8E%87&source_file_id=3783&show_algorithm_tags=1)[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3783&show_algorithm_tags=1)