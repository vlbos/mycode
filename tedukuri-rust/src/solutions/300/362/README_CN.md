362\. 区间

*    [题目](https://www.acwing.com/problem/content/description/364/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/364/1/)
*    [题解](https://www.acwing.com/problem/content/solution/364/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/364/)

  

给定 nn 个区间 \[ai,bi\]\[ai,bi\] 和 nn 个整数 cici。

你需要构造一个整数集合 ZZ，使得 ∀i∈\[1,n\]∀i∈\[1,n\]，ZZ 中满足 ai≤x≤biai≤x≤bi 的整数 xx 不少于 cici 个。

求这样的整数集合 ZZ 最少包含多少个数。

#### 输入格式

第一行包含整数 nn。

接下来 nn 行，每行包含三个整数 ai,bi,ciai,bi,ci。

#### 输出格式

输出一个整数表示结果。

#### 数据范围

1≤n≤500001≤n≤50000,  
0≤ai,bi≤500000≤ai,bi≤50000,  
0≤ci≤bi−ai+10≤ci≤bi−ai+1

#### 输入样例：

    5
    3 7 3
    8 10 3
    6 8 1
    1 3 1
    10 11 1
    

#### 输出样例：

    6
    

难度：中等

时/空限制：1s / 64MB

总通过数：7614

总尝试数：13537

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3913&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3913&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3913&show_algorithm_tags=1)[差分约束](https://www.acwing.com/problem/search/1/?search_content=%E5%B7%AE%E5%88%86%E7%BA%A6%E6%9D%9F&source_file_id=3913&show_algorithm_tags=1)