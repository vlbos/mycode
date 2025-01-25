312\. 乌龟棋

*    [题目](https://www.acwing.com/problem/content/description/314/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/314/1/)
*    [题解](https://www.acwing.com/problem/content/solution/314/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/314/)

  

小明过生日的时候，爸爸送给他一副乌龟棋当作礼物。

乌龟棋的棋盘只有一行，该行有 NN 个格子，每个格子上一个分数（非负整数）。

棋盘第 11 格是唯一的起点，第 NN 格是终点，游戏要求玩家控制一个乌龟棋子从起点出发走到终点。

乌龟棋中共有 MM 张爬行卡片，分成 44 种不同的类型（MM 张卡片中不一定包含所有 44 种类型的卡片），每种类型的卡片上分别标有 1、2、3、41、2、3、4 四个数字之一，表示使用这种卡片后，乌龟棋子将向前爬行相应的格子数。

游戏中，玩家每次需要从所有的爬行卡片中选择一张之前没有使用过的爬行卡片，控制乌龟棋子前进相应的格子数，每张卡片只能使用一次。

游戏中，乌龟棋子自动获得起点格子的分数，并且在后续的爬行中每到达一个格子，就得到该格子相应的分数。

玩家最终游戏得分就是乌龟棋子从起点到终点过程中到过的所有格子的分数总和。

很明显，用不同的爬行卡片使用顺序会使得最终游戏的得分不同，小明想要找到一种卡片使用顺序使得最终游戏得分最多。

现在，告诉你棋盘上每个格子的分数和所有的爬行卡片，你能告诉小明，他最多能得到多少分吗？

#### 输入格式

输入文件的每行中两个数之间用一个空格隔开。

第 11 行 22 个正整数 NN 和 MM，分别表示棋盘格子数和爬行卡片数。

第 22 行 NN 个非负整数，a1,a2,……,aNa1,a2,……,aN，其中 aiai 表示棋盘第 ii 个格子上的分数。

第 33 行 MM 个整数，b1,b2,……,bMb1,b2,……,bM，表示 MM 张爬行卡片上的数字。

输入数据保证到达终点时刚好用光 MM 张爬行卡片。

#### 输出格式

输出只有 11 行，包含 11 个整数，表示小明最多能得到的分数。

#### 数据范围

1≤N≤3501≤N≤350,  
1≤M≤1201≤M≤120,  
0≤ai≤1000≤ai≤100,  
1≤bi≤41≤bi≤4,  
每种爬行卡片的张数不会超过 4040。

#### 输入样例：

    9 5
    6 10 14 2 8 8 18 5 17
    1 3 1 2 1
    

#### 输出样例：

    73
    

难度：简单

时/空限制：1s / 64MB

总通过数：6036

总尝试数：9497

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3863&show_algorithm_tags=0)[NOIP2010提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2010%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3863&show_algorithm_tags=0)

算法标签

[DP](https://www.acwing.com/problem/search/1/?search_content=DP&source_file_id=3863&show_algorithm_tags=1)[线性DP](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%80%A7DP&source_file_id=3863&show_algorithm_tags=1)