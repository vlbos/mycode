105\. 七夕祭

*    [题目](https://www.acwing.com/problem/content/description/107/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/107/1/)
*    [题解](https://www.acwing.com/problem/content/solution/107/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/107/)

  

七夕节因牛郎织女的传说而被扣上了「情人节」的帽子。

于是 TYVJ 今年举办了一次线下七夕祭。

Vani 同学今年成功邀请到了 cl 同学陪他来共度七夕，于是他们决定去 TYVJ 七夕祭游玩。

TYVJ 七夕祭和 11 区的夏祭的形式很像。

矩形的祭典会场由 NN 排 MM 列共计 N×MN×M 个摊点组成。

虽然摊点种类繁多，不过 cl 只对其中的一部分摊点感兴趣，比如章鱼烧、苹果糖、棉花糖、射的屋……什么的。

Vani 预先联系了七夕祭的负责人 zhq，希望能够通过恰当地布置会场，使得各行中 cl 感兴趣的摊点数一样多，并且各列中 cl 感兴趣的摊点数也一样多。

不过 zhq 告诉 Vani，摊点已经随意布置完毕了，如果想满足 cl 的要求，唯一的调整方式就是交换两个相邻的摊点。

两个摊点相邻，当且仅当他们处在同一行或者同一列的相邻位置上。

由于 zhq 率领的 TYVJ 开发小组成功地扭曲了空间，每一行或每一列的第一个位置和最后一个位置也算作相邻。

现在 Vani 想知道他的两个要求最多能满足多少个。

在此前提下，至少需要交换多少次摊点。

#### 输入格式

第一行包含三个整数 NN 和 MM 和 TT，TT 表示 cl 对多少个摊点感兴趣。

接下来 TT 行，每行两个整数 x,yx,y，表示 cl 对处在第 xx 行第 yy 列的摊点感兴趣。

#### 输出格式

首先输出一个字符串。

如果能满足 Vani 的全部两个要求，输出 both；

如果通过调整只能使得各行中 cl 感兴趣的摊点数一样多，输出 row；

如果只能使各列中 cl 感兴趣的摊点数一样多，输出 column；

如果均不能满足，输出 impossible。

如果输出的字符串不是 impossible， 接下来输出最小交换次数，与字符串之间用一个空格隔开。

#### 数据范围

1≤N,M≤1000001≤N,M≤100000,  
0≤T≤min(N∗M,100000)0≤T≤min(N∗M,100000),  
1≤x≤N1≤x≤N,  
1≤y≤M1≤y≤M

#### 输入样例：

    2 3 4
    1 3
    2 1
    2 2
    2 3
    

#### 输出样例：

    row 1
    

难度：困难

时/空限制：1s / 64MB

总通过数：9328

总尝试数：25906

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3656&show_algorithm_tags=0)

算法标签

[排序](https://www.acwing.com/problem/search/1/?search_content=%E6%8E%92%E5%BA%8F&source_file_id=3656&show_algorithm_tags=1)[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3656&show_algorithm_tags=1)[推公式](https://www.acwing.com/problem/search/1/?search_content=%E6%8E%A8%E5%85%AC%E5%BC%8F&source_file_id=3656&show_algorithm_tags=1)