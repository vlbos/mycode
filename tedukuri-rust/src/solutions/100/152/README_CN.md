152\. 城市游戏

*    [题目](https://www.acwing.com/problem/content/description/154/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/154/1/)
*    [题解](https://www.acwing.com/problem/content/solution/154/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/154/)

  

有一天，小猫 rainbow 和 freda 来到了湘西张家界的天门山玉蟾宫，玉蟾宫宫主蓝兔盛情地款待了它们，并赐予它们一片土地。

这片土地被分成 N×MN×M 个格子，每个格子里写着 `R` 或者 `F`，`R` 代表这块土地被赐予了 rainbow，`F` 代表这块土地被赐予了 freda。

现在 freda 要在这里卖萌。。。它要找一块矩形土地，要求这片土地都标着 `F` 并且面积最大。

但是 rainbow 和 freda 的 OI 水平都弱爆了，找不出这块土地，而蓝兔也想看 freda 卖萌（她显然是不会编程的……），所以它们决定，如果你找到的土地面积为 SS，它们将给你 3×S3×S 两银子。

#### 输入格式

第一行包括两个整数 N,MN,M，表示矩形土地有 NN 行 MM 列。

接下来 NN 行，每行 MM 个用空格隔开的字符 `F` 或 `R`，描述了矩形土地。

每行末尾没有多余空格。

#### 输出格式

输出一个整数，表示你能得到多少银子，即(3×3×最大 `F` 矩形土地面积)的值。

#### 数据范围

1≤N,M≤10001≤N,M≤1000

#### 输入样例：

    5 6
    R F F F F F
    F F F F F F
    R R R F F F
    F F F F F F
    F F F F F F
    

#### 输出样例：

    45
    

难度：中等

时/空限制：1s / 64MB

总通过数：3659

总尝试数：6363

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3703&show_algorithm_tags=0)

算法标签

[单调栈](https://www.acwing.com/problem/search/1/?search_content=%E5%8D%95%E8%B0%83%E6%A0%88&source_file_id=3703&show_algorithm_tags=1)