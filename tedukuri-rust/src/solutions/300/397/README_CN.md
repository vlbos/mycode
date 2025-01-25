397\. 逃不掉的路

*    [题目](https://www.acwing.com/problem/content/description/399/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/399/1/)
*    [题解](https://www.acwing.com/problem/content/solution/399/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/399/)

  

现代社会，路是必不可少的。

共有 nn 个城镇，mm 条道路，任意两个城镇都有路相连，而且往往不止一条。

但有些路年久失修，走着很不爽。

按理说条条大路通罗马，大不了绕行其他路呗——可小撸却发现：从 aa 城到 bb 城不管怎么走，总有一些逃不掉的必经之路。

他想请你计算一下，aa 到 bb 的所有路径中，有几条路是逃不掉的？

#### 输入格式

第一行是 nn 和 mm，用空格隔开。

接下来 mm 行，每行两个整数 xx 和 yy，用空格隔开，表示 xx 城和 yy 城之间有一条长为 11 的双向路。

第 m+2m+2 行是 qq。

接下来 qq 行，每行两个整数 aa 和 bb，用空格隔开，表示一次询问。

#### 输出格式

对于每次询问，输出一个正整数，表示 aa 城到 bb 城必须经过几条路。

每个输出占一行。

#### 数据范围

n≤105,m≤2∗105,q≤105n≤105,m≤2∗105,q≤105  
对于全部的数据，1≤x,y,a,b≤n1≤x,y,a,b≤n；对于任意的道路，两端的城市编号之差不超过 104104；  
任意两个城镇都有路径相连；同一条道路不会出现两次；道路的起终点不会相同;查询的两个城市不会相同。

#### 输入样例：

    5 5
    1 2
    1 3
    2 4
    3 4
    4 5
    2
    1 4
    2 5
    

#### 输出样例：

    0
    1
    

难度：中等

时/空限制：1s / 64MB

总通过数：621

总尝试数：2024

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3948&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3948&show_algorithm_tags=1)[边双连通分量](https://www.acwing.com/problem/search/1/?search_content=%E8%BE%B9%E5%8F%8C%E8%BF%9E%E9%80%9A%E5%88%86%E9%87%8F&source_file_id=3948&show_algorithm_tags=1)[缩点](https://www.acwing.com/problem/search/1/?search_content=%E7%BC%A9%E7%82%B9&source_file_id=3948&show_algorithm_tags=1)[无向图必经边](https://www.acwing.com/problem/search/1/?search_content=%E6%97%A0%E5%90%91%E5%9B%BE%E5%BF%85%E7%BB%8F%E8%BE%B9&source_file_id=3948&show_algorithm_tags=1)