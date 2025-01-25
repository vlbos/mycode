280\. 陪审团

*    [题目](https://www.acwing.com/problem/content/description/282/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/282/1/)
*    [题解](https://www.acwing.com/problem/content/solution/282/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/282/)

  

在一个遥远的国家，一名嫌疑犯是否有罪需要由陪审团来决定。

陪审团是由法官从公民中挑选的。

法官先随机挑选 NN 个人（编号 1,2…,N1,2…,N）作为陪审团的候选人，然后再从这 NN 个人中按照下列方法选出 MM 人组成陪审团。

首先，参与诉讼的控方和辩方会给所有候选人打分，分值在 00 到 2020 之间。

第 ii 个人的得分分别记为 p\[i\]p\[i\] 和 d\[i\]d\[i\]。

为了公平起见，法官选出的 MM 个人必须满足：辩方总分 DD 和控方总分 PP 的差的绝对值 |D−P||D−P| 最小。

如果选择方法不唯一，那么再从中选择辨控双方总分之和 D+PD+P 最大的方案。

求最终的陪审团获得的辩方总分 DD、控方总分 PP，以及陪审团人选的编号。

**注意**：若陪审团的人选方案不唯一，则任意输出一组合法方案即可。

#### 输入格式

输入包含多组测试数据。

每组测试数据第一行包含两个整数 NN 和 MM。

接下来 NN 行，每行包含两个整数 p\[i\]p\[i\] 和 d\[i\]d\[i\]。

每组测试数据之间隔一个空行。

当输入数据 N\=0，M\=0N\=0，M\=0 时，表示结束输入，该数据无需处理。

#### 输出格式

对于每组数据，第一行输出 `Jury #C`，CC 为数据编号，从 11 开始。

第二行输出 `Best jury has value P for prosecution and value D for defence:`，PP 为控方总分，DD 为辩方总分。

第三行输出按升序排列的陪审人选编号，每个编号前输出一个空格。

每组数据输出完后，输出一个空行。

#### 数据范围

1≤N≤2001≤N≤200,  
1≤M≤201≤M≤20  
0≤p\[i\],d\[i\]≤200≤p\[i\],d\[i\]≤20

#### 输入样例：

    4 2
    1 2
    2 3
    4 1
    6 2
    0 0
    

#### 输出样例：

    Jury #1
    Best jury has value 6 for prosecution and value 4 for defence:
     2 3
    
    

难度：中等

时/空限制：1s / 64MB

总通过数：3988

总尝试数：15947

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3831&show_algorithm_tags=0)[POJ1015](https://www.acwing.com/problem/search/1/?search_content=POJ1015&source_file_id=3831&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3831&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3831&show_algorithm_tags=1)[01背包问题](https://www.acwing.com/problem/search/1/?search_content=01%E8%83%8C%E5%8C%85%E9%97%AE%E9%A2%98&source_file_id=3831&show_algorithm_tags=1)