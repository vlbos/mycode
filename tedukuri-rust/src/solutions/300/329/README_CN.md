329\. 围栏障碍训练场

*    [题目](https://www.acwing.com/problem/content/description/331/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/331/1/)
*    [题解](https://www.acwing.com/problem/content/solution/331/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/331/)

  

农夫约翰为他的奶牛们建造了一个围栏障碍训练场，以供奶牛们玩耍。

训练场由 NN 个不同长度的围栏组成，每个围栏都与 xx 轴平行，并且第 ii 个围栏的 yy 坐标为 ii。

训练场的出口位于原点，起点位于 (S,N)(S,N)

       +-S-+-+-+        (fence #N)
    
     +-+-+-+            (fence #N-1)
    
         ...               ...
    
       +-+-+-+          (fence #2)
    
         +-+-+-+        (fence #1)
    
    =|=|=|=*=|=|=|      (barn)
    
    -3-2-1 0 1 2 3 
    

这些牛会从起点处开始向下走，当它们碰到围栏时会选择沿着围栏向左或向右走，走到围栏端点时继续往下走，按照此种走法一直走到出口为止。

请问，这些牛从开始到结束，行走的水平距离最少为多少。

#### 输入格式

第一行包含两个整数 NN 和 SS。

第 2..N+12..N+1 行，每行包含两个整数 Ai,BiAi,Bi，表示一个围栏的起始横坐标和结束横坐标，其中第 i+1i+1 行表示第 ii 个围栏的数据。

起点坐标满足 AN≤S≤BNAN≤S≤BN。

#### 输出格式

输出一个整数，表示最小水平行走距离。

#### 数据范围

1≤N≤300001≤N≤30000,  
−105≤S≤105−105≤S≤105,  
−105≤Ai,Bi≤105−105≤Ai,Bi≤105

#### 输入样例：

    4 0 
    -2 1
    -1 2
    -3 0
    -2 1
    

#### 输出样例：

    4
    

难度：中等

时/空限制：1s / 64MB

总通过数：472

总尝试数：1100

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3880&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3880&show_algorithm_tags=1)[线段树优化DP](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%AE%B5%E6%A0%91%E4%BC%98%E5%8C%96DP&source_file_id=3880&show_algorithm_tags=1)