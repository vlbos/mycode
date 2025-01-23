110\. 防晒

*    [题目](https://www.acwing.com/problem/content/description/112/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/112/1/)
*    [题解](https://www.acwing.com/problem/content/solution/112/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/112/)

  

有 CC 头奶牛进行日光浴，第 ii 头奶牛需要 minSPF\[i\]minSPF\[i\] 到 maxSPF\[i\]maxSPF\[i\] 单位强度之间的阳光。

每头奶牛在日光浴前必须涂防晒霜，防晒霜有 LL 种，涂上第 ii 种之后，身体接收到的阳光强度就会稳定为 SPF\[i\]SPF\[i\]，第 ii 种防晒霜有 cover\[i\]cover\[i\] 瓶。

求最多可以满足多少头奶牛进行日光浴。

#### 输入格式

第一行输入整数 CC 和 LL。

接下来的 CC 行，按次序每行输入一头牛的 minSPFminSPF 和 maxSPFmaxSPF 值，即第 ii 行输入 minSPF\[i\]minSPF\[i\] 和 maxSPF\[i\]maxSPF\[i\]。

再接下来的 LL 行，按次序每行输入一种防晒霜的 SPFSPF 和 covercover 值，即第 ii 行输入 SPF\[i\]SPF\[i\] 和 cover\[i\]cover\[i\]。

每行的数据之间用空格隔开。

#### 输出格式

输出一个整数，代表最多可以满足奶牛日光浴的奶牛数目。

#### 数据范围

1≤C,L≤25001≤C,L≤2500,  
1≤minSPF≤maxSPF≤10001≤minSPF≤maxSPF≤1000,  
1≤SPF≤10001≤SPF≤1000

#### 输入样例：

    3 2
    3 10
    2 5
    1 5
    6 2
    4 1
    

#### 输出样例：

    2
    

难度：简单

时/空限制：1s / 64MB

总通过数：6700

总尝试数：16921

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3661&show_algorithm_tags=0)

算法标签

[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3661&show_algorithm_tags=1)