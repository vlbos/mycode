200\. Hankson的趣味题

*    [题目](https://www.acwing.com/problem/content/description/202/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/202/1/)
*    [题解](https://www.acwing.com/problem/content/solution/202/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/202/)

  

Hanks 博士是 BT（Bio-Tech，生物技术）领域的知名专家，他的儿子名叫 Hankson。

现在，刚刚放学回家的 Hankson 正在思考一个有趣的问题。

今天在课堂上，老师讲解了如何求两个正整数 c1c1 和 c2c2 的最大公约数和最小公倍数。

现在 Hankson 认为自己已经熟练地掌握了这些知识，他开始思考一个“求公约数”和“求公倍数”之类问题的“逆问题”，这个问题是这样的：

已知正整数 a0,a1,b0,b1a0,a1,b0,b1，设某未知正整数 xx 满足：

1.  xx 和 a0a0 的最大公约数是 a1a1；
2.  xx 和 b0b0 的最小公倍数是 b1b1。

Hankson 的“逆问题”就是求出满足条件的正整数 xx。

但稍加思索之后，他发现这样的 xx 并不唯一，甚至可能不存在。

因此他转而开始考虑如何求解满足条件的 xx 的个数。

请你帮助他编程求解这个问题。

#### 输入格式

输入第一行为一个正整数 nn，表示有 nn 组输入数据。

接下来的 nn 行每行一组输入数据，为四个正整数 a0，a1，b0，b1a0，a1，b0，b1，每两个整数之间用一个空格隔开。

输入数据保证 a0a0 能被 a1a1 整除，b1b1 能被 b0b0 整除。

#### 输出格式

输出共 nn 行。

每组输入数据的输出结果占一行，为一个整数。

对于每组数据：若不存在这样的 xx，请输出 00；

若存在这样的 xx，请输出满足条件的 xx 的个数；

#### 数据范围

1≤n≤20001≤n≤2000,  
1≤a0,a1,b0,b1≤2∗1091≤a0,a1,b0,b1≤2∗109

#### 输入样例：

    2
    41 1 96 288
    95 1 37 1776
    

#### 输出样例：

    6
    2
    

难度：中等

时/空限制：1s / 64MB

总通过数：6946

总尝试数：17574

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3751&show_algorithm_tags=0)[NOIP2009提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2009%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3751&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3751&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3751&show_algorithm_tags=1)[最大公约数](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E5%A4%A7%E5%85%AC%E7%BA%A6%E6%95%B0&source_file_id=3751&show_algorithm_tags=1)