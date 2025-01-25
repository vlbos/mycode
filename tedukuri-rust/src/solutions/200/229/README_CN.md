229\. 新NIM游戏

*    [题目](https://www.acwing.com/problem/content/description/231/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/231/1/)
*    [题解](https://www.acwing.com/problem/content/solution/231/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/231/)

  

传统的 Nim 游戏是这样的：有一些火柴堆，每堆都有若干根火柴（不同堆的火柴数量可以不同）。

两个游戏者轮流操作，每次可以选一个火柴堆拿走若干根火柴。

可以只拿一根，也可以拿走整堆火柴，但不能同时从超过一堆火柴中拿。

拿走最后一根火柴的游戏者胜利。

本题的游戏稍微有些不同：在第一个回合中，第一个游戏者可以直接拿走若干个整堆的火柴。

可以一堆都不拿，但不可以全部拿走。

第二回合也一样，第二个游戏者也有这样一次机会。

从第三个回合（又轮到第一个游戏者）开始，规则和 Nim 游戏一样。

如果你先拿，怎样才能保证获胜？

如果可以获胜的话，还要让第一回合拿的火柴总数尽量小。

#### 输入格式

第一行为整数 kk，即火柴堆数。

第二行包含 kk 个正整数（均不超过 109109），即各堆的火柴个数。

#### 输出格式

输出第一回合拿的火柴数目的最小值。

如果不能保证取胜，输出 −1−1。

#### 数据范围

1≤k≤1001≤k≤100

#### 输入样例：

    6
    5 5 6 6 5 5
    

#### 输出样例：

    21
    

难度：简单

时/空限制：1s / 64MB

总通过数：428

总尝试数：757

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3780&show_algorithm_tags=0)[CQOI2013](https://www.acwing.com/problem/search/1/?search_content=CQOI2013&source_file_id=3780&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3780&show_algorithm_tags=1)[高斯消元](https://www.acwing.com/problem/search/1/?search_content=%E9%AB%98%E6%96%AF%E6%B6%88%E5%85%83&source_file_id=3780&show_algorithm_tags=1)[博弈论](https://www.acwing.com/problem/search/1/?search_content=%E5%8D%9A%E5%BC%88%E8%AE%BA&source_file_id=3780&show_algorithm_tags=1)[线性基](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%80%A7%E5%9F%BA&source_file_id=3780&show_algorithm_tags=1)