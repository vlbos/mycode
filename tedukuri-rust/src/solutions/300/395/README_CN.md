395\. 冗余路径

*    [题目](https://www.acwing.com/problem/content/description/397/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/397/1/)
*    [题解](https://www.acwing.com/problem/content/solution/397/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/397/)

  

为了从 FF 个草场中的一个走到另一个，奶牛们有时不得不路过一些她们讨厌的可怕的树。

奶牛们已经厌倦了被迫走某一条路，所以她们想建一些新路，使每一对草场之间都会至少有两条相互分离的路径，这样她们就有多一些选择。

每对草场之间已经有至少一条路径。

给出所有 RR 条双向路的描述，每条路连接了两个不同的草场，请计算最少的新建道路的数量，路径由若干道路首尾相连而成。

两条路径相互分离，是指两条路径没有一条重合的道路。

但是，两条分离的路径上可以有一些相同的草场。

可能有不止一条道路直接连接同一对草场，尽管如此，你仍可以在它们之间再建一条道路，作为另一条不同的道路。

#### 输入格式

第 11 行输入 FF 和 RR。

接下来 RR 行，每行输入两个整数，表示两个草场，它们之间有一条道路。

#### 输出格式

输出一个整数，表示最少的需要新建的道路数。

#### 数据范围

1≤F≤50001≤F≤5000,  
F−1≤R≤10000F−1≤R≤10000

#### 输入样例：

    7 7
    1 2
    2 3
    3 4
    2 5
    4 5
    5 6
    5 7
    

#### 输出样例：

    2
    

难度：简单

时/空限制：1s / 64MB

总通过数：6366

总尝试数：10993

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3946&show_algorithm_tags=0)[POJ3177](https://www.acwing.com/problem/search/1/?search_content=POJ3177&source_file_id=3946&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3946&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3946&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3946&show_algorithm_tags=1)[Tarjan算法](https://www.acwing.com/problem/search/1/?search_content=Tarjan%E7%AE%97%E6%B3%95&source_file_id=3946&show_algorithm_tags=1)[桥](https://www.acwing.com/problem/search/1/?search_content=%E6%A1%A5&source_file_id=3946&show_algorithm_tags=1)