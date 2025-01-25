257\. 关押罪犯

*    [题目](https://www.acwing.com/problem/content/description/259/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/259/1/)
*    [题解](https://www.acwing.com/problem/content/solution/259/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/259/)

  

SS 城现有两座监狱，一共关押着 NN 名罪犯，编号分别为 1∼N1∼N。

他们之间的关系自然也极不和谐。

很多罪犯之间甚至积怨已久，如果客观条件具备则随时可能爆发冲突。

我们用“怨气值”（一个正整数值）来表示某两名罪犯之间的仇恨程度，怨气值越大，则这两名罪犯之间的积怨越多。

如果两名怨气值为 cc 的罪犯被关押在同一监狱，他们俩之间会发生摩擦，并造成影响力为 cc 的冲突事件。

每年年末，警察局会将本年内监狱中的所有冲突事件按影响力从大到小排成一个列表，然后上报到 SS 城 ZZ 市长那里。

公务繁忙的 ZZ 市长只会去看列表中的第一个事件的影响力，如果影响很坏，他就会考虑撤换警察局长。

在详细考察了 NN 名罪犯间的矛盾关系后，警察局长觉得压力巨大。

他准备将罪犯们在两座监狱内重新分配，以求产生的冲突事件影响力都较小，从而保住自己的乌纱帽。

假设只要处于同一监狱内的某两个罪犯间有仇恨，那么他们一定会在每年的某个时候发生摩擦。

那么，应如何分配罪犯，才能使 ZZ 市长看到的那个冲突事件的影响力最小？这个最小值是多少？

#### 输入格式

第一行为两个正整数 NN 和 MM，分别表示罪犯的数目以及存在仇恨的罪犯对数。

接下来的 MM 行每行为三个正整数 aj，bj，cjaj，bj，cj，表示 ajaj 号和 bjbj 号罪犯之间存在仇恨，其怨气值为 cjcj。

数据保证 1≤aj<bj<N,0<cj≤1091≤aj<bj<N,0<cj≤109 且每对罪犯组合只出现一次。

#### 输出格式

输出共 11 行，为 ZZ 市长看到的那个冲突事件的影响力。

如果本年内监狱中未发生任何冲突事件，请输出 00。

#### 数据范围

N≤20000,M≤100000N≤20000,M≤100000

#### 输入样例：

    4 6
    1 4 2534
    2 3 3512
    1 2 28351
    1 3 6618
    2 4 1805
    3 4 12884
    

#### 输出样例：

    3512
    

难度：中等

时/空限制：1s / 64MB

总通过数：11008

总尝试数：18803

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3808&show_algorithm_tags=0)[NOIP2010提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2010%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3808&show_algorithm_tags=0)

算法标签

[二分](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86&source_file_id=3808&show_algorithm_tags=1)[染色法](https://www.acwing.com/problem/search/1/?search_content=%E6%9F%93%E8%89%B2%E6%B3%95&source_file_id=3808&show_algorithm_tags=1)[二分图](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86%E5%9B%BE&source_file_id=3808&show_algorithm_tags=1)[并查集](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%B6%E6%9F%A5%E9%9B%86&source_file_id=3808&show_algorithm_tags=1)