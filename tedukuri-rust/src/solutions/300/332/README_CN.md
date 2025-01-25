332\. 股票交易

*    [题目](https://www.acwing.com/problem/content/description/334/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/334/1/)
*    [题解](https://www.acwing.com/problem/content/solution/334/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/334/)

  

最近 lxhgww 又迷上了投资股票，通过一段时间的观察和学习，他总结出了股票行情的一些规律。

通过一段时间的观察，lxhgww 预测到了未来 TT 天内某只股票的走势，第 ii 天的股票买入价为每股 APiAPi，第 ii 天的股票卖出价为每股 BPiBPi（数据保证对于每个 ii，都有 APi≥BPiAPi≥BPi），但是每天不能无限制地交易，于是股票交易所规定第 ii 天的一次买入至多只能购买 ASiASi 股，一次卖出至多只能卖出 BSiBSi 股。

另外，股票交易所还制定了两个规定。

为了避免大家疯狂交易，股票交易所规定在两次交易（某一天的买入或者卖出均算是一次交易）之间，至少要间隔 WW 天，也就是说如果在第 ii 天发生了交易，那么从第 i+1i+1 天到第 i+Wi+W 天，均不能发生交易。

同时，为了避免垄断，股票交易所还规定在任何时间，一个人的手里的股票数不能超过 MaxPMaxP。

在第 11 天之前，lxhgww 手里有一大笔钱（可以认为钱的数目无限），但是没有任何股票，当然，TT 天以后，lxhgww 想要赚到最多的钱，聪明的程序员们，你们能帮助他吗？

#### 输入格式

第 11 行包括 33 个整数，分别是 T，MaxP，WT，MaxP，W。

第 2..T+12..T+1 行，第 i+1i+1 行代表第 ii 天的股票走势，每行 44 个整数，分别表示 APi，BPi，ASi，BSiAPi，BPi，ASi，BSi。

#### 输出格式

输出包含一个整数，表示能赚到的做多的钱数。

#### 数据范围

0≤W<T≤20000≤W<T≤2000,  
1≤MaxP≤20001≤MaxP≤2000,  
1≤BPi≤APi≤10001≤BPi≤APi≤1000,  
1≤ASi≤BSi≤MaxP1≤ASi≤BSi≤MaxP

#### 输入样例：

    5 2 0
    2 1 1 1
    2 1 1 1
    3 2 1 1
    4 3 1 1
    5 4 1 1
    

#### 输出样例：

    3
    

难度：困难

时/空限制：1s / 64MB

总通过数：707

总尝试数：1991

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3883&show_algorithm_tags=0)[SCOI2010](https://www.acwing.com/problem/search/1/?search_content=SCOI2010&source_file_id=3883&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3883&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3883&show_algorithm_tags=1)[单调队列优化DP](https://www.acwing.com/problem/search/1/?search_content=%E5%8D%95%E8%B0%83%E9%98%9F%E5%88%97%E4%BC%98%E5%8C%96DP&source_file_id=3883&show_algorithm_tags=1)