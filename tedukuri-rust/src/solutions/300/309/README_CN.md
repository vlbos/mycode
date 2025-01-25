309\. 装饰围栏

*    [题目](https://www.acwing.com/problem/content/description/311/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/311/1/)
*    [题解](https://www.acwing.com/problem/content/solution/311/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/311/)

  

有 NN 块长方形的木板，长度分别为 1,2,…,N1,2,…,N，宽度都是 11。

现在要用这 NN 块木板组成一个宽度为 NN 的围栏，满足在围栏中，每块木板两侧的木板要么都比它高，要么都比它低。

也就是说，围栏中的木板是高低交错的。

我们称“两侧比它低的木板”处于高位，“两侧比它高的木板”处于低位。

显然，有很多种构建围栏的方案。

每个方案可以写作一个长度为 NN 的序列，序列中的各元素是木板的长度。

把这些序列按照字典序排序，如下图所示，就是 N\=4N\=4 时，所有满足条件的围栏按照木板长度的字典序排序后的结果。

![fence.gif](https://cdn.acwing.com/media/article/image/2019/02/02/19_4f3d5ce226-fence.gif)

现在给定整数 CC，求排名为 CC 的围栏中，各木板的长度从左到右依次是多少。

#### 输入格式

第一行包含整数 KK，表示一共有 KK 组数据。

接下来 KK 行，每行包含一组数据，包括两个整数 NN 和 CC。

#### 输出格式

每组数据输出一行结果，结果表示排名为 CC 的围栏中，各木板的长度从左到右排成的序列。

同行数据用空格隔开。

#### 数据范围

1≤N≤201≤N≤20,  
0<C<2630<C<263

#### 输入样例：

    2
    2 1
    3 3
    

#### 输出样例：

    1 2
    2 3 1
    

难度：中等

时/空限制：1s / 64MB

总通过数：582

总尝试数：1060

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3860&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3860&show_algorithm_tags=1)[计数类DP](https://www.acwing.com/problem/search/1/?search_content=%E8%AE%A1%E6%95%B0%E7%B1%BBDP&source_file_id=3860&show_algorithm_tags=1)