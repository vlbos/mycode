331\. 干草堆

*    [题目](https://www.acwing.com/problem/content/description/333/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/333/1/)
*    [题解](https://www.acwing.com/problem/content/solution/333/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/333/)

  

奶牛们讨厌黑暗。

为了调整牛棚顶的电灯的亮度，Bessie 必须建一座干草堆使得她能够爬上去够到灯泡。

一共有 NN 大包的干草(从 11 到 NN 编号)依靠传送带连续的传输进牛棚来。

第 ii 包干草有一个宽度 WiWi。

所有的干草包的厚度和高度都为 11。

Bessie 必须利用所有 NN 包干草来建立起干草堆。

她可以想放多少包就放多少包来建立起草堆的地基（当然是紧紧的放在一行中）。

接下来她可以将下一个草包放在之前一级的上方来建立新的一级。

注意：每一级不能比下面的一级宽。

她持续的这么放置，直到所有的草包都被安置完成。

她必须按照草包进入牛棚的顺序堆放草包。

说得更清楚一些：一旦她将一个草包放在第二级 ，那么她不能将接下来的草包放在第一级上。

Bessie 的目标是建立起最高的草包堆。

#### 输入格式

第 11 行：一个整数 NN。

第 2..N+12..N+1 行：第 i+1i+1 行包含整数 WiWi。

#### 输出格式

输出一个整数，表示草包堆的最高的高度。

#### 数据范围

1≤N≤1000001≤N≤100000,  
1≤Wi≤100001≤Wi≤10000

#### 输入样例：

    3
    1
    2
    3
    

#### 输出样例：

    2
    

难度：中等

时/空限制：1s / 64MB

总通过数：577

总尝试数：1417

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3882&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3882&show_algorithm_tags=1)[单调队列优化DP](https://www.acwing.com/problem/search/1/?search_content=%E5%8D%95%E8%B0%83%E9%98%9F%E5%88%97%E4%BC%98%E5%8C%96DP&source_file_id=3882&show_algorithm_tags=1)[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3882&show_algorithm_tags=1)