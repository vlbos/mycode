347\. 野餐规划

*    [题目](https://www.acwing.com/problem/content/description/349/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/349/1/)
*    [题解](https://www.acwing.com/problem/content/solution/349/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/349/)

  

一群小丑演员，以其出色的柔术表演，可以无限量的钻进同一辆汽车中，而闻名世界。

现在他们想要去公园玩耍，但是他们的经费非常紧缺。

他们将乘车前往公园，为了减少花费，他们决定选择一种合理的乘车方式，可以使得他们去往公园需要的所有汽车行驶的总公里数最少。

为此，他们愿意通过很多人挤在同一辆车的方式，来减少汽车行驶的总花销。

由此，他们可以很多人驾车到某一个兄弟的家里，然后所有人都钻进一辆车里，再继续前进。

公园的停车场能停放的车的数量有限，而且因为公园有入场费，所以一旦一辆车子进入到公园内，就必须停在那里，不能再去接其他人。

现在请你想出一种方法，可以使得他们全都到达公园的情况下，所有汽车行驶的总路程最少。

#### 输入格式

第一行包含整数 nn，表示人和人之间或人和公园之间的道路的总数量。

接下来 nn 行，每行包含两个字符串 A、BA、B 和一个整数 LL，用以描述人 AA 和人 BB 之前存在道路，路长为 LL（L≤200L≤200），或者描述某人和公园之间存在道路，路长为 LL。

道路都是双向的，并且人数不超过 2020，表示人的名字的字符串长度不超过 1010，公园用 `Park` 表示。

再接下来一行，包含整数 ss，表示公园的最大停车数量。

你可以假设每个人的家都有一条通往公园的道路。

#### 输出格式

输出 `Total miles driven: xxx`，其中 xxxxxx 表示所有汽车行驶的总路程。

#### 输入样例：

    10
    Alphonzo Bernardo 32
    Alphonzo Park 57
    Alphonzo Eduardo 43
    Bernardo Park 19
    Bernardo Clemenzi 82
    Clemenzi Park 65
    Clemenzi Herb 90
    Clemenzi Eduardo 109
    Park Herb 24
    Herb Eduardo 79
    3
    

#### 输出样例：

    Total miles driven: 183
    

难度：中等

时/空限制：1s / 10MB

总通过数：1582

总尝试数：4389

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3898&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3898&show_algorithm_tags=1)[最小生成树](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E5%B0%8F%E7%94%9F%E6%88%90%E6%A0%91&source_file_id=3898&show_algorithm_tags=1)