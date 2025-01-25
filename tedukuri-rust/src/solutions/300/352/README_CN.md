352\. 闇の連鎖

*    [题目](https://www.acwing.com/problem/content/description/354/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/354/1/)
*    [题解](https://www.acwing.com/problem/content/solution/354/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/354/)

  

传说中的暗之连锁被人们称为 Dark。

Dark 是人类内心的黑暗的产物，古今中外的勇者们都试图打倒它。

经过研究，你发现 Dark 呈现无向图的结构，图中有 NN 个节点和两类边，一类边被称为主要边，而另一类被称为附加边。

Dark 有 N–1N–1 条主要边，并且 Dark 的任意两个节点之间都存在一条只由主要边构成的路径。

另外，Dark 还有 MM 条附加边。

你的任务是把 Dark 斩为不连通的两部分。

一开始 Dark 的附加边都处于无敌状态，你只能选择一条主要边切断。

一旦你切断了一条主要边，Dark 就会进入防御模式，主要边会变为无敌的而附加边可以被切断。

但是你的能力只能再切断 Dark 的一条附加边。

现在你想要知道，一共有多少种方案可以击败 Dark。

注意，就算你第一步切断主要边之后就已经把 Dark 斩为两截，你也需要切断一条附加边才算击败了 Dark。

#### 输入格式

第一行包含两个整数 NN 和 MM。

之后 N–1N–1 行，每行包括两个整数 AA 和 BB，表示 AA 和 BB 之间有一条主要边。

之后 MM 行以同样的格式给出附加边。

#### 输出格式

输出一个整数表示答案。

#### 数据范围

N≤100000,M≤200000N≤100000,M≤200000，数据保证答案不超过231−1231−1

#### 输入样例：

    4 1
    1 2
    2 3
    1 4
    3 4
    

#### 输出样例：

    3
    

难度：困难

时/空限制：1s / 64MB

总通过数：6565

总尝试数：14438

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3903&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3903&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3903&show_algorithm_tags=1)[树上差分](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%91%E4%B8%8A%E5%B7%AE%E5%88%86&source_file_id=3903&show_algorithm_tags=1)[LCA](https://www.acwing.com/problem/search/1/?search_content=LCA&source_file_id=3903&show_algorithm_tags=1)