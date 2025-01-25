227\. 小部件厂

*    [题目](https://www.acwing.com/problem/content/description/229/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/229/1/)
*    [题解](https://www.acwing.com/problem/content/solution/229/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/229/)

  

小部件工厂生产几种不同类型的小部件。

每个小部件都是精心制作而成。

制作小部件所需的时间取决于其类型：简单小部件仅需要 33 天，但最复杂的小部件可能需要多达 99 天。

工厂目前处于完全混乱的状态：最近，工厂被一位新主人收购，新主人解雇了几乎所有员工。

新员工对制作小部件毫无经验，没有人清楚制作每个不同类型的小部件分别需要多少天。

当客户订购小部件，工厂却无法告诉客户生产所需商品需要多少天时显得十分尴尬。

幸运的是，这里有记录记载了每个工人开始制作的日期，完成制作的日期以及制作的小部件型号。

但是问题是记录没有明确记载工人开始和完成工作的确切日期，只记录了该天是星期几。

尽管如此，这些信息也是有些帮助的：例如，如果一个人在星期二开始制作一个 4141 型小部件，并在周五完成，那么我们就知道了制作一个 4141 型小部件需要 44 天时间（因为最多不超过 99 天，所以不可能是 1111 天或更多）。

您的任务是从这些记录中（如果可能）找出制作不同类型的小部件所需的天数。

#### 输入格式

输入包含多组测试用例。

每个测试用例的第一行包含两个整数 nn 和 mm，分别代表小部件的型号总数以及记录总数。

接下来是对 mm 个记录的描述。

每个记录占两行，第一行描述该名工人制作的小部件总数 kk 以及他开始制作和完成制作具体是星期几。

一周的日子由字符串 `MON`，`TUE`，`WED`，`THU`，`FRI`，`SAT` 和 `SUN` 来表示。

第二行包含用空格分隔的 kk 个整数，表示该工人具体制作了哪些类型的部件。

如下面的例子中，这名工人星期三开始工作，星期日结束工作，期间先后完成制作了型号 1313，型号 1818，型号 11，型号 1313 四个小部件。

    4 WED SUN 
    13 18 1 13 
    

每名工人一周工作 77 天。

当输入用例 n\=m\=0n\=m\=0 时，表示输入终止，且该用例无需处理。

#### 输出格式

每个测试用例输出一行结果，结果包含 nn 个用空格隔开的整数，表示制作每个小部件所需的天数。

如果测试用例有多种可能结果，则输出 `Multiple solutions.`。

如果无解，则输出 `Inconsistent data.`。

#### 数据范围

1≤n,m≤3001≤n,m≤300,  
1≤k≤100001≤k≤10000

#### 输入样例：

    2 3
    2 MON THU
    1 2
    3 MON FRI
    1 1 2
    3 MON SUN
    1 2 2
    10 2
    1 MON TUE 
    3
    1 MON WED
    3
    0 0
    

#### 输出样例：

    8 3
    Inconsistent data.
    

难度：中等

时/空限制：1s / 64MB

总通过数：621

总尝试数：2497

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3778&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3778&show_algorithm_tags=1)[高斯消元](https://www.acwing.com/problem/search/1/?search_content=%E9%AB%98%E6%96%AF%E6%B6%88%E5%85%83&source_file_id=3778&show_algorithm_tags=1)[同余](https://www.acwing.com/problem/search/1/?search_content=%E5%90%8C%E4%BD%99&source_file_id=3778&show_algorithm_tags=1)