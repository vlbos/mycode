260\. 买票

*    [题目](https://www.acwing.com/problem/content/description/262/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/262/1/)
*    [题解](https://www.acwing.com/problem/content/solution/262/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/262/)

  

达达在买回家的火车票，因为正值春运，售票处排起了长队。

因为晚上室内光线很暗，所以很多人趁机插队。

现在给每个人赋予一个整数作为编号，告诉你每一个排队的人的编号，和他进入队列时的具体位置。

请你确定最终的队列顺序。

#### 输入格式

输入可能包含多组测试用例。

对于每组测试用例，第一行包含整数 NN，表示排队的总人数。

接下来 NN 行，每行两个整数 Pi,ViPi,Vi，第 ii 行数据表示第 ii 个人进入队列时的位置以及他的个人编号。

一个人的 PiPi 值具体表示为当该人员进入队列时，他前面的人数。

例如，如果一个人插到了队首，则其 PiPi 值为 00，如果插到了第三个位置（第二个人后面），则其 PiPi 值为 22。

#### 输出格式

每个测试用例，输出一行包含 NN 个整数（表示每个人的编号）的结果，表示最终的人员队列顺序。

每个结果占一行，同行数据之间空格隔开。

#### 数据范围

1≤N≤2000001≤N≤200000,  
0≤Vi≤327670≤Vi≤32767,  
0≤Pi≤i−10≤Pi≤i−1

#### 输入样例：

    4
    0 77
    1 51
    1 33
    2 69
    4
    0 20523
    1 19243
    1 3890
    0 31492
    

#### 输出样例：

    77 33 69 51
    31492 20523 3890 19243
    

#### 样例解释

下图描述了输入样例中第一组测试用例的场景。

![2828.gif](https://cdn.acwing.com/media/article/image/2019/01/24/19_2ce647201f-2828.gif)

难度：简单

时/空限制：1s / 64MB

总通过数：1016

总尝试数：1624

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3811&show_algorithm_tags=0)[POJ2828](https://www.acwing.com/problem/search/1/?search_content=POJ2828&source_file_id=3811&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3811&show_algorithm_tags=0)

算法标签

[树状数组](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%91%E7%8A%B6%E6%95%B0%E7%BB%84&source_file_id=3811&show_algorithm_tags=1)