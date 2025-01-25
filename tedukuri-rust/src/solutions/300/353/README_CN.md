353\. 雨天的尾巴

*    [题目](https://www.acwing.com/problem/content/description/355/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/355/1/)
*    [题解](https://www.acwing.com/problem/content/solution/355/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/355/)

  

深绘里一直很讨厌雨天。

灼热的天气穿透了前半个夏天，后来一场大雨和随之而来的洪水，浇灭了一切。

虽然深绘里家乡的小村落对洪水有着顽固的抵抗力，但也倒了几座老房子，几棵老树被连根拔起，以及田地里的粮食被弄得一片狼藉。

无奈的深绘里和村民们只好等待救济粮来维生。

不过救济粮的发放方式很特别。

有 nn 个点，形成一个树状结构。

有 mm 次发放操作，每次选择两个点 x,yx,y，对 xx 到 yy 的路径上（包括 x,yx,y）的每个点发放一袋 zz 类型的物品。

求完成所有发放操作后，每个点存放最多的是哪种类型的物品。

#### 输入格式

第一行两个正整数 n,mn,m，含义如题目所示。

接下来 n−1n−1 行，每行两个数 (a,b)(a,b)，表示 (a,b)(a,b) 间有一条边。

再接下来 mm 行，每行三个数 (x,y,z)(x,y,z)，含义如题目所示。

#### 输出格式

共 nn 行，第 ii 行一个整数，表示第 ii 座房屋里存放的最多的是哪种救济粮，如果有多种救济粮存放次数一样，输出编号最小的。

如果某座房屋里没有救济粮，则对应一行输出 00。

#### 数据范围

1≤n,m≤1000001≤n,m≤100000,  
1≤z≤1051≤z≤105

#### 输入样例：

    5 3
    1 2
    3 1
    3 4
    5 3
    2 3 3
    1 5 2
    3 3 3
    

#### 输出样例：

    2
    3
    3
    0
    2
    

难度：困难

时/空限制：1s / 256MB

总通过数：1508

总尝试数：3986

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3904&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3904&show_algorithm_tags=1)[树上差分](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%91%E4%B8%8A%E5%B7%AE%E5%88%86&source_file_id=3904&show_algorithm_tags=1)