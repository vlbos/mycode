301\. 任务安排2

*    [题目](https://www.acwing.com/problem/content/description/303/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/303/1/)
*    [题解](https://www.acwing.com/problem/content/solution/303/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/303/)

  

有 NN 个任务排成一个序列在一台机器上等待执行，它们的顺序不得改变。

机器会把这 NN 个任务分成若干批，每一批包含连续的若干个任务。

从时刻 00 开始，任务被分批加工，执行第 ii 个任务所需的时间是 TiTi。

另外，在每批任务开始前，机器需要 SS 的启动时间，故执行一批任务所需的时间是启动时间 SS 加上每个任务所需时间之和。

一个任务执行后，将在机器中稍作等待，直至该批任务全部执行完毕。

也就是说，同一批任务将在同一时刻完成。

每个任务的费用是它的完成时刻乘以一个费用系数 CiCi。

请为机器规划一个分组方案，使得总费用最小。

#### 输入格式

第一行包含整数 NN。

第二行包含整数 SS。

接下来 NN 行每行有一对整数，分别为 TiTi 和 CiCi，表示第 ii 个任务单独完成所需的时间 TiTi 及其费用系数 CiCi。

#### 输出格式

输出一个整数，表示最小总费用。

#### 数据范围

1≤N≤3×1051≤N≤3×105,  
1≤Ti,Ci≤5121≤Ti,Ci≤512,  
0≤S≤5120≤S≤512

#### 输入样例：

    5
    1
    1 3
    3 2
    4 3
    2 3
    1 4
    

#### 输出样例：

    153
    

难度：困难

时/空限制：1s / 64MB

总通过数：5852

总尝试数：11143

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3852&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3852&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3852&show_algorithm_tags=1)[斜率优化](https://www.acwing.com/problem/search/1/?search_content=%E6%96%9C%E7%8E%87%E4%BC%98%E5%8C%96&source_file_id=3852&show_algorithm_tags=1)