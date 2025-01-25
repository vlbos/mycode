392\. 会合

*    [题目](https://www.acwing.com/problem/content/description/394/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/394/1/)
*    [题解](https://www.acwing.com/problem/content/solution/394/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/394/)

  

给定一个 nn 个顶点的有向图，每个顶点有且仅有一条出边。

对于顶点 ii，记它的出边为 (i,a\[i\])(i,a\[i\])。

再给出 qq 组询问，每组询问由两个顶点 a、ba、b 组成，要求输出满足下面条件的 x、yx、y：

1.  从顶点 aa 沿着出边走 xx 步和从顶点 bb 沿着出边走 yy 步后到达的顶点相同。
2.  在满足条件 11 的情况下，如果解不唯一，则还需要令 max(x,y)max(x,y) 最小。
3.  在满足条件 11 和 22 的情况下，如果解不唯一，则还需要令 min(x,y)min(x,y) 最小。
4.  在满足条件 1、21、2 和 33 的情况下，如果解不唯一，则还需要令 x≥yx≥y。

如果不存在满足条件 11 的 x、yx、y，输出 `-1 -1`。

#### 输入格式

第一行两个正整数 nn 和 qq。

第二行 nn 个正整数 a\[1\],a\[2\],…,a\[n\]a\[1\],a\[2\],…,a\[n\]。

下面 qq 行，每行两个正整数 a,ba,b，表示一组询问。

#### 输出格式

输出 qq 行，每行两个整数。

#### 数据范围

n,q≤500000n,q≤500000,  
a\[i\]≤na\[i\]≤n,  
a,b≤na,b≤n

#### 输入样例：

    12 5
    4 3 5 5 1 1 12 12 9 9 7 1
    7 2
    8 11
    1 2
    9 10
    10 5
    

#### 输出样例：

    2 3
    1 2
    2 2
    0 1
    -1 -1
    

难度：简单

时/空限制：2s / 128MB

总通过数：310

总尝试数：1137

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3943&show_algorithm_tags=0)[POI2012](https://www.acwing.com/problem/search/1/?search_content=POI2012&source_file_id=3943&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3943&show_algorithm_tags=1)[基环树](https://www.acwing.com/problem/search/1/?search_content=%E5%9F%BA%E7%8E%AF%E6%A0%91&source_file_id=3943&show_algorithm_tags=1)[最近公共祖先](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E8%BF%91%E5%85%AC%E5%85%B1%E7%A5%96%E5%85%88&source_file_id=3943&show_algorithm_tags=1)