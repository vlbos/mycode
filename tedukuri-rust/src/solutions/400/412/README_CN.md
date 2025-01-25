412\. 排水沟

*    [题目](https://www.acwing.com/problem/content/description/414/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/414/1/)
*    [题解](https://www.acwing.com/problem/content/solution/414/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/414/)

  

为了防止池塘里的三叶草被雨水淹没，农夫约翰挖了很多排水沟，将雨水排到河中。

约翰在每一个沟渠中都安装了调节器，借此可以调整水流入该沟渠的速度。

约翰不仅知道每个沟渠的具体排水速度，还知道它们的分布位置。

对于任何给定的沟渠，水都只能沿着一个方向流动，但是水有可能循环流动。

根据给定的信息，请你求出池塘排水到河中的最大速率。

#### 输入格式

第一行包含两个整数 NN 和 MM，NN 表示排水沟的数量，MM 是沟渠的交叉点数。

交叉点 11 处是池塘，交叉点 MM 处是河。

接下来 NN 行，每行包含三个整数 Si,Ei,CiSi,Ei,Ci，SiSi 和 EiEi 是一条沟渠的两个交叉点，水流从 SiSi 流向 EiEi，CiCi 是水流最大速率。

#### 输出格式

输出一个整数，表示水从池塘排到河中的最大速率。

#### 数据范围

0≤N≤2000≤N≤200,  
2≤M≤2002≤M≤200,  
1≤Si,Ei≤M1≤Si,Ei≤M,  
0≤Ci≤1070≤Ci≤107

#### 输入样例：

    5 4
    1 2 40
    1 4 20
    2 4 20
    2 3 30
    3 4 10
    

#### 输出样例：

    50
    

难度：中等

时/空限制：1s / 64MB

总通过数：571

总尝试数：829

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3963&show_algorithm_tags=0)[usaco training 4.2](https://www.acwing.com/problem/search/1/?search_content=usaco%20training%204.2&source_file_id=3963&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3963&show_algorithm_tags=1)[最大流](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E5%A4%A7%E6%B5%81&source_file_id=3963&show_algorithm_tags=1)