259\. 真正的骗子

*    [题目](https://www.acwing.com/problem/content/description/261/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/261/1/)
*    [题解](https://www.acwing.com/problem/content/solution/261/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/261/)

  

一个岛上存在着两种居民，一种是天神，一种是恶魔。

天神永远都不会说假话，而恶魔永远都不会说真话。

岛上的每一个成员都有一个整数编号（类似于身份证号，用以区分每个成员）。

现在你拥有 nn 次提问的机会，但是问题的内容只能是向其中一个居民询问另一个居民是否是天神，请你根据收集的回答判断各个居民的身份。

#### 输入格式

输入包含多组测试用例。

每组测试用例的第一行包含三个非负整数 n,p1,p2n,p1,p2，其中 nn 是你可以提问的总次数，p1p1 是天神的总数量，p2p2 是恶魔的总数量。

接下来 nn 行每行包含两个整数 xi,yixi,yi 以及一个字符串 aiai，其中 xi,yixi,yi 是岛上居民的编号，你将向编号为 xixi 的居民询问编号为 yiyi 的居民是否是天神，

aiai 是他的回答，如果 aiai 为 `yes`，表示他回答你“是”，如果 aiai 为 `no`，表示他回答你“不是”。

xi,yixi,yi 可能相同，表示你问的是那个人自己是否为天神。

当输入为占据一行的 `0 0 0` 时，表示输入终止。

#### 输出格式

对于每组测试用例，如果询问得到的信息足以使你判断每个居民的身份，则将所有天神的编号升序输出，每个编号占一行，在输出结束后，在另起一行输出 `end`，表示该用例输出结束。

如果得到的信息不足以判断每个居民的身份，则输出 `no`，输出同样占一行。

#### 数据范围

1≤xi,yi≤p1+p21≤xi,yi≤p1+p2,  
0≤n<1000,0≤p1,p2<3000≤n<1000,0≤p1,p2<300

#### 输入样例：

    2 1 1
    1 2 no
    2 1 no
    3 2 1
    1 1 yes
    2 2 yes
    3 3 yes
    2 2 1
    1 2 yes
    2 3 no
    5 4 3
    1 2 yes
    1 3 no
    4 5 yes
    5 6 yes
    6 7 no
    0 0 0
    

#### 输出样例：

    no
    no
    1
    2
    end
    3
    4
    5
    6
    end
    

难度：困难

时/空限制：1s / 10MB

总通过数：1024

总尝试数：3218

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3810&show_algorithm_tags=0)[POJ1417](https://www.acwing.com/problem/search/1/?search_content=POJ1417&source_file_id=3810&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3810&show_algorithm_tags=0)

算法标签

[并查集](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%B6%E6%9F%A5%E9%9B%86&source_file_id=3810&show_algorithm_tags=1)[背包](https://www.acwing.com/problem/search/1/?search_content=%E8%83%8C%E5%8C%85&source_file_id=3810&show_algorithm_tags=1)