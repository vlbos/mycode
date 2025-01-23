127\. 任务

*    [题目](https://www.acwing.com/problem/content/description/129/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/129/1/)
*    [题解](https://www.acwing.com/problem/content/solution/129/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/129/)

  

今天某公司有 MM 个任务需要完成。

每个任务都有相应的难度级别和完成任务所需时间。

第 ii 个任务的难度级别为 yiyi，完成任务所需时间为 xixi 分钟。

如果公司完成此任务，他们将获得（500×xi+2×yi500×xi+2×yi）美元收入。

该公司有 NN 台机器，每台机器都有最长工作时间和级别。

如果任务所需时间超过机器的最长工作时间，则机器无法完成此任务。

如果任务难度级别超过机器的级别，则机器无法完成次任务。

每台机器一天内只能完成一项任务。

每个任务只能由一台机器完成。

请为他们设计一个任务分配方案，使得该公司能够最大化他们今天可以完成的任务数量。

如果有多种解决方案，他们希望选取赚取利润最高的那种。

#### 输入格式

输入包含几个测试用例。

对于每个测试用例，第一行包含两个整数 NN 和 MM，分别代表机器数量和任务数量。

接下来 NN 行，每行包含两个整数 xi,yixi,yi，分别代表机器最长工作时间和机器级别。

再接下来 MM 行，每行包含两个整数 xi,yixi,yi，分别代表完成任务所需时间和任务难度级别。

#### 输出格式

对于每个测试用例，输出两个整数，代表公司今天可以完成的最大任务数以及他们将获得的收入。

#### 数据范围

1≤N,M≤1000001≤N,M≤100000,  
0<xi<14400<xi<1440,  
0≤yi≤1000≤yi≤100

#### 输入样例：

    1 2
    100 3
    100 2
    100 1
    

#### 输出样例：

    1 50004
    

难度：困难

时/空限制：2s / 64MB

总通过数：2044

总尝试数：5774

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3678&show_algorithm_tags=0)

算法标签

[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3678&show_algorithm_tags=1)