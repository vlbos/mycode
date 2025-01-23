101\. 最高的牛

*    [题目](https://www.acwing.com/problem/content/description/103/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/103/1/)
*    [题解](https://www.acwing.com/problem/content/solution/103/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/103/)

  

有 NN 头牛站成一行，被编队为 1、2、3…N1、2、3…N，每头牛的身高都为整数。

当且仅当两头牛中间的牛身高都比它们矮时，两头牛方可看到对方。

现在，我们只知道其中最高的牛是第 PP 头，它的身高是 HH ，剩余牛的身高未知。

但是，我们还知道这群牛之中存在着 MM 对关系，每对关系都指明了某两头牛 AA 和 BB 可以相互看见。

求每头牛的身高的最大可能值是多少。

#### 输入格式

第一行输入整数 N,P,H,MN,P,H,M，数据用空格隔开。

接下来 MM 行，每行输出两个整数 AA 和 BB ，代表牛 AA 和牛 BB 可以相互看见，数据用空格隔开。

#### 输出格式

一共输出 NN 行数据，每行输出一个整数。

第 ii 行输出的整数代表第 ii 头牛可能的最大身高。

#### 数据范围

1≤N≤50001≤N≤5000,  
1≤H≤10000001≤H≤1000000,  
1≤A,B≤100001≤A,B≤10000,  
A≠BA≠B,  
0≤M≤100000≤M≤10000

#### 输入样例：

    9 3 5 5
    1 3
    5 3
    4 3
    3 7
    9 8
    

#### 输出样例：

    5
    4
    5
    3
    4
    4
    5
    5
    5
    

##### 注意：

*   此题中给出的关系对可能存在重复

难度：简单

时/空限制：2s / 64MB

总通过数：11014

总尝试数：21936

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3652&show_algorithm_tags=0)

算法标签

[差分](https://www.acwing.com/problem/search/1/?search_content=%E5%B7%AE%E5%88%86&source_file_id=3652&show_algorithm_tags=1)