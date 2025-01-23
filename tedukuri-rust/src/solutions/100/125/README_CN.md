125\. 耍杂技的牛

*    [题目](https://www.acwing.com/problem/content/description/127/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/127/1/)
*    [题解](https://www.acwing.com/problem/content/solution/127/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/127/)

  

农民约翰的 NN 头奶牛（编号为 1..N1..N）计划逃跑并加入马戏团，为此它们决定练习表演杂技。

奶牛们不是非常有创意，只提出了一个杂技表演：

叠罗汉，表演时，奶牛们站在彼此的身上，形成一个高高的垂直堆叠。

奶牛们正在试图找到自己在这个堆叠中应该所处的位置顺序。

这 NN 头奶牛中的每一头都有着自己的重量 WiWi 以及自己的强壮程度 SiSi。

一头牛支撑不住的可能性取决于它头上所有牛的总重量（不包括它自己）减去它的身体强壮程度的值，现在称该数值为风险值，风险值越大，这只牛撑不住的可能性越高。

您的任务是确定奶牛的排序，使得所有奶牛的风险值中的最大值尽可能的小。

#### 输入格式

第一行输入整数 NN，表示奶牛数量。

接下来 NN 行，每行输入两个整数，表示牛的重量和强壮程度，第 ii 行表示第 ii 头牛的重量 WiWi 以及它的强壮程度 SiSi。

#### 输出格式

输出一个整数，表示最大风险值的最小可能值。

#### 数据范围

1≤N≤500001≤N≤50000,  
1≤Wi≤10,0001≤Wi≤10,000,  
1≤Si≤1,000,000,0001≤Si≤1,000,000,000

#### 输入样例：

    3
    10 3
    2 5
    3 3
    

#### 输出样例：

    2
    

难度：中等

时/空限制：1s / 64MB

总通过数：26477

总尝试数：43474

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3676&show_algorithm_tags=0)[模板题](https://www.acwing.com/problem/search/1/?search_content=%E6%A8%A1%E6%9D%BF%E9%A2%98&source_file_id=3676&show_algorithm_tags=0)

算法标签

[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3676&show_algorithm_tags=1)