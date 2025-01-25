218\. 扑克牌

*    [题目](https://www.acwing.com/problem/content/description/220/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/220/1/)
*    [题解](https://www.acwing.com/problem/content/solution/220/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/220/)

  

Admin 生日那天，Rainbow 来找 Admin 玩扑克牌。

玩着玩着 Rainbow 觉得太没意思了，于是决定给 Admin 一个考验。

Rainbow 把一副扑克牌(5454 张)随机洗开，倒扣着放成一摞。

然后 Admin 从上往下依次翻开每张牌，每翻开一张黑桃、红桃、梅花或者方块，就把它放到对应花色的堆里去。

Rainbow 想问问 Admin，得到至少 AA 张黑桃、至少 BB 张红桃、至少 CC 张梅花、至少 DD 张方块需要翻开的牌的张数的期望值 EE 是多少？

特殊地，如果翻开的牌是大王或者小王，Admin 将会把它作为某种花色的牌放入对应堆中，使得放入之后 EE 的值尽可能小。

由于 Admin 和 Rainbow 还在玩扑克，所以这个程序就交给你来写了。

#### 输入格式

输入仅由一行，包含四个用空格隔开的整数，A,B,C,DA,B,C,D。

#### 输出格式

输出需要翻开的牌数的期望值 EE，四舍五入保留 33 位小数。

如果不可能达到输入的状态，输出 `-1.000`。

#### 数据范围

0≤A,B,C,D≤150≤A,B,C,D≤15

#### 输入样例：

    1 2 3 4
    

#### 输出样例：

    16.393
    

难度：中等

时/空限制：1s / 64MB

总通过数：3471

总尝试数：5774

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3769&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3769&show_algorithm_tags=1)[概率与数学期望](https://www.acwing.com/problem/search/1/?search_content=%E6%A6%82%E7%8E%87%E4%B8%8E%E6%95%B0%E5%AD%A6%E6%9C%9F%E6%9C%9B&source_file_id=3769&show_algorithm_tags=1)