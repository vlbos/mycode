114\. 国王游戏

*    [题目](https://www.acwing.com/problem/content/description/116/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/116/1/)
*    [题解](https://www.acwing.com/problem/content/solution/116/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/116/)

  

恰逢 HH 国国庆，国王邀请 nn 位大臣来玩一个有奖游戏。

首先，他让每个大臣在左、右手上面分别写下一个整数，国王自己也在左、右手上各写一个整数。

然后，让这 nn 位大臣排成一排，国王站在队伍的最前面。

排好队后，所有的大臣都会获得国王奖赏的若干金币，每位大臣获得的金币数分别是:

排在该大臣前面的所有人的左手上的数的乘积除以他自己右手上的数，然后向下取整得到的结果。

国王不希望某一个大臣获得特别多的奖赏，所以他想请你帮他重新安排一下队伍的顺序，使得获得奖赏最多的大臣，所获奖赏尽可能的少。

注意，国王的位置始终在队伍的最前面。

#### 输入格式

第一行包含一个整数 nn，表示大臣的人数。

第二行包含两个整数 aa 和 bb，之间用一个空格隔开，分别表示国王左手和右手上的整数。

接下来 nn 行，每行包含两个整数 aa 和 bb，之间用一个空格隔开，分别表示每个大臣左手和右手上的整数。

#### 输出格式

输出只有一行，包含一个整数，表示重新排列后的队伍中获奖赏最多的大臣所获得的金币数。

#### 数据范围

1≤n≤10001≤n≤1000  
0<a,b<100000<a,b<10000

#### 输入样例：

    3
    1 1
    2 3
    7 4
    4 6
    

#### 输出样例：

    2
    

难度：中等

时/空限制：1s / 64MB

总通过数：4887

总尝试数：13733

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3665&show_algorithm_tags=0)[NOIP2012提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2012%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3665&show_algorithm_tags=0)

算法标签

[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3665&show_algorithm_tags=1)[高精度](https://www.acwing.com/problem/search/1/?search_content=%E9%AB%98%E7%B2%BE%E5%BA%A6&source_file_id=3665&show_algorithm_tags=1)