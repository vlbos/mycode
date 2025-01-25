363\. B城

*    [题目](https://www.acwing.com/problem/content/description/365/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/365/1/)
*    [题解](https://www.acwing.com/problem/content/solution/365/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/365/)

  

BB 城有 nn 个城镇，mm 条双向道路。

每条道路连结两个不同的城镇，没有重复的道路，所有城镇连通。

把城镇看作节点，把道路看作边，容易发现，整个城市构成了一个无向图。

#### 输入格式

第一行包含两个整数 nn 和 mm。

接下来 mm 行，每行包含两个整数 aa 和 bb，表示城镇 aa 和 bb 之间存在一条道路。

#### 输出格式

输出共 nn 行，每行输出一个整数。

第 ii 行输出的整数表示把与节点 ii 关联的所有边去掉以后（不去掉节点 ii 本身），无向图有多少个有序点 (x,y)(x,y)，满足 xx 和 yy 不连通。

#### 数据范围

n≤100000,m≤500000n≤100000,m≤500000

#### 输入样例：

    5 5
    1 2
    2 3
    1 3
    3 4
    4 5
    

#### 输出样例：

    8
    8
    16
    14
    8
    

难度：简单

时/空限制：1s / 64MB

总通过数：1478

总尝试数：3361

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3914&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3914&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3914&show_algorithm_tags=1)[Tarjan算法](https://www.acwing.com/problem/search/1/?search_content=Tarjan%E7%AE%97%E6%B3%95&source_file_id=3914&show_algorithm_tags=1)[无向图连通性](https://www.acwing.com/problem/search/1/?search_content=%E6%97%A0%E5%90%91%E5%9B%BE%E8%BF%9E%E9%80%9A%E6%80%A7&source_file_id=3914&show_algorithm_tags=1)