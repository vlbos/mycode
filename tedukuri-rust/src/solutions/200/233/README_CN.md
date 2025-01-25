233\. 换教室

*    [题目](https://www.acwing.com/problem/content/description/235/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/235/1/)
*    [题解](https://www.acwing.com/problem/content/solution/235/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/235/)

  

对于刚上大学的牛牛来说，他面临的第一个问题是如何根据实际情况申请合适的课程。

在可以选择的课程中，有 2n2n 节课程安排在 nn 个时间段上。

在第 ii (1≤i≤n1≤i≤n) 个时间段上，两节内容相同的课程同时在不同的地点进行，其中，牛牛预先被安排在教室 cici 上课，而另一节课程在教室 didi 进行。

在不提交任何申请的情况下，学生们需要按时间段的顺序依次完成所有的 nn 节安排好的课程。

如果学生想更换第 ii 节课程的教室，则需要提出申请。

若申请通过，学生就可以在第 ii 个时间段去教室 didi 上课，否则仍然在教室 cici 上课。

由于更换教室的需求太多，申请不一定能获得通过。

通过计算，牛牛发现申请更换第 ii 节课程的教室时，申请被通过的概率是一个已知的实数 kiki，并且对于不同课程的申请，被通过的概率是互相独立的。

学校规定，所有的申请只能在学期开始前一次性提交，并且每个人只能选择至多 mm 节课程进行申请。

这意味着牛牛必须一次性决定是否申请更换每节课的教室，而不能根据某些课程的申请结果来决定其他课程是否申请。

牛牛可以申请自己最希望更换教室的 mm 门课程，也可以不用完这 mm 个申请的机会，甚至可以一门课程都不申请。

因为不同的课程可能会被安排在不同的教室进行，所以牛牛需要利用课间时间从一间教室赶到另一间教室。

牛牛所在的大学有 vv 个教室，有 ee 条道路。

每条道路连接两间教室，并且是可以双向通行的。

由于道路的长度和路况不同，通过不同的道路耗费的体力可能会有所不同。

当第 ii (1≤i≤n−11≤i≤n−1) 节课结束后，牛牛就会从这节课的教室出发，选择一条耗费体力最少的路径前往下一节课的教室。

现在牛牛想知道，申请哪几门课程可以使他因在教室间移动耗费的体力值的总和的期望值最小，请你帮他求出这个最小值。

#### 输入格式

第一行四个整数 n,m,v,en,m,v,e，nn 表示这个学期内的时间段的数量; mm 表示牛牛最多可以申请更换多少节课程的教室; vv 表示牛牛学校里教室的数量; ee 表示牛牛的学校里道路的数量。

第二行 nn 个正整数，第 ii 个正整数表示 cici，即第 ii 个时间段牛牛被安排上课的教室；保证 1≤ci≤v1≤ci≤v。

第三行 nn 个正整数，第 ii 个正整数表示 didi，即第 ii 个时间段另一间上同样课程的教室；保证 1≤di≤v1≤di≤v。

第四行 nn 个实数，第 ii 个实数表示 kiki，即牛牛申请在第 ii 个时间段更换教室获得通过的概率；保证 0≤ki≤10≤ki≤1。

接下来 ee 行，每行三个正整数aj,bj,wjaj,bj,wj,表示有一条双向道路连接教室 aj,bjaj,bj ,通过这条道路需要耗费的体力值是 wjwj ;保证1≤aj,bj≤v,1≤wj≤1001≤aj,bj≤v,1≤wj≤100。

保证 1≤n≤2000,0≤m≤2000,1≤v≤300,0≤e≤900001≤n≤2000,0≤m≤2000,1≤v≤300,0≤e≤90000。

保证通过学校里的道路，从任何一间教室出发，都能到达其他所有的教室。

保证输入的实数最多包含 33 位小数。

#### 输出格式

输出一行，包含一个实数，四舎五入精确到小数点后恰好 22 位，表示答案。

你的输出必须和标准输出完全一样才算正确。

测试数据保证四舎五入后的答案和准确答案的差的绝对值不大于 4∗10−34∗10−3 。 (如果你不知道什么是浮点误差，这段话可以理解为: 对于大多数的算法， 你可以正常地使用浮点数类型而不用对它进行特殊的处理)

#### 输入样例：

    3 2 3 3
    2 1 2
    1 2 1
    0.8 0.2 0.5 
    1 2 5
    1 3 3
    2 3 1
    

#### 输出样例：

    2.80
    

难度：中等

时/空限制：1s / 128MB

总通过数：857

总尝试数：2146

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3784&show_algorithm_tags=0)[NOIP2016提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2016%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3784&show_algorithm_tags=0)

算法标签

[DP](https://www.acwing.com/problem/search/1/?search_content=DP&source_file_id=3784&show_algorithm_tags=1)[数学期望](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E6%9C%9F%E6%9C%9B&source_file_id=3784&show_algorithm_tags=1)[Floyd](https://www.acwing.com/problem/search/1/?search_content=Floyd&source_file_id=3784&show_algorithm_tags=1)