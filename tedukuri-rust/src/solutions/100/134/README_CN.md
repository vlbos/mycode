134\. 双端队列

*    [题目](https://www.acwing.com/problem/content/description/136/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/136/1/)
*    [题解](https://www.acwing.com/problem/content/solution/136/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/136/)

  

达达现在碰到了一个棘手的问题，有 NN 个整数需要排序。

达达手头能用的工具就是若干个双端队列。

她从 11 到 NN 需要依次处理这 NN 个数，对于每个数，达达能做以下两件事：

1．新建一个双端队列，并将当前数作为这个队列中的唯一的数；

2．将当前数放入已有的队列的头之前或者尾之后。

对所有的数处理完成之后，达达将这些队列按一定的顺序连接起来后就可以得到一个非降的序列。

请你求出最少需要多少个双端序列。

#### 输入格式

第一行输入整数 NN，代表整数的个数。

接下来 NN 行，每行包括一个整数 DiDi，代表所需处理的整数。

#### 输出格式

输出一个整数，代表最少需要的双端队列数。

#### 数据范围

1≤N≤2000001≤N≤200000

#### 输入样例：

    6
    3
    6
    0
    9
    6
    3
    

#### 输出样例：

    2
    

难度：困难

时/空限制：1s / 64MB

总通过数：2610

总尝试数：7386

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3685&show_algorithm_tags=0)[beijing2011](https://www.acwing.com/problem/search/1/?search_content=beijing2011&source_file_id=3685&show_algorithm_tags=0)

算法标签

[队列](https://www.acwing.com/problem/search/1/?search_content=%E9%98%9F%E5%88%97&source_file_id=3685&show_algorithm_tags=1)