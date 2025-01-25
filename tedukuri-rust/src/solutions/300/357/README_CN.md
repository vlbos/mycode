357\. 疫情控制

*    [题目](https://www.acwing.com/problem/content/description/359/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/359/1/)
*    [题解](https://www.acwing.com/problem/content/solution/359/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/359/)

  

HH 国有 nn 个城市，这 nn 个城市用 n−1n−1 条双向道路相互连通构成一棵树，11 号城市是首都，也是树中的根节点。

HH 国的首都爆发了一种危害性极高的传染病。

当局为了控制疫情，不让疫情扩散到边境城市（叶子节点所表示的城市），决定动用军队在一些城市建立检查点，使得从首都到边境城市的每一条路径上都至少有一个检查点，边境城市也可以建立检查点。

但要注意的是，首都是不能建立检查点的。

现在，在 HH 国的一些城市中已经驻扎有军队，且一个城市可以驻扎多个军队。

军队总数为 mm 支。

一支军队可以在有道路连接的城市间移动，并在除首都以外的任意一个城市建立检查点，且只能在一个城市建立检查点。

一支军队经过一条道路从一个城市移动到另一个城市所需要的时间等于道路的长度（单位：小时）。

请问：最少需要多少个小时才能控制疫情？

注意：不同的军队可以同时移动。

#### 输入格式

第一行一个整数 nn，表示城市个数。

接下来的 n−1n−1 行，每行 33 个整数，u、v、wu、v、w，每两个整数之间用一个空格隔开，表示从城市 uu 到城市 vv 有一条长为 ww 的道路，数据保证输入的是一棵树，且根节点编号为 11。

接下来一行一个整数 mm，表示军队个数。

接下来一行 mm 个整数，每两个整数之间用一个空格隔开，分别表示这 mm 个军队所驻扎的城市的编号。

#### 输出格式

共一行，包含一个整数，表示控制疫情所需要的最少时间。如果无法控制疫情则输出 −1−1。

#### 数据范围

2≤m≤n≤500002≤m≤n≤50000,  
0<w<1090<w<109

#### 输入样例：

    4
    1 2 1
    1 3 2
    3 4 3
    2
    2 2
    

#### 输出样例：

    3
    

难度：困难

时/空限制：2s / 64MB

总通过数：964

总尝试数：2685

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3908&show_algorithm_tags=0)[NOIP2012提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2012%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3908&show_algorithm_tags=0)

算法标签

[二分](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86&source_file_id=3908&show_algorithm_tags=1)[倍增](https://www.acwing.com/problem/search/1/?search_content=%E5%80%8D%E5%A2%9E&source_file_id=3908&show_algorithm_tags=1)[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3908&show_algorithm_tags=1)