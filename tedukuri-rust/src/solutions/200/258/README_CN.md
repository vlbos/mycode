258\. 石头剪子布

*    [题目](https://www.acwing.com/problem/content/description/260/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/260/1/)
*    [题解](https://www.acwing.com/problem/content/solution/260/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/260/)

  

NN 个小朋友（编号为 0,1,2,…,N−10,1,2,…,N−1）一起玩石头剪子布游戏。

其中一人为裁判，其余的人被分为三个组（有可能有一些组是空的），第一个组的小朋友只能出石头，第二个组的小朋友只能出剪子，第三个组的小朋友只能出布，而裁判可以使用任意手势。

你不知道谁是裁判，也不知道小朋友们是怎么分组的。

然后，孩子们开始玩游戏，游戏一共进行 MM 轮，每轮从 NN 个小朋友中选出两个小朋友进行猜拳。

你将被告知两个小朋友猜拳的胜负结果，但是你不会被告知两个小朋友具体使用了哪种手势。

比赛结束后，你能根据这些结果推断出裁判是谁吗？

如果可以的话，你最早在第几轮可以找到裁判。

#### 输入格式

输入可能包含多组测试用例

每组测试用例第一行包含两个整数 NN 和 MM。

接下来 MM 行，每行包含两个整数 a,ba,b，中间夹着一个符号(`>`,`=`,`<`)，表示一轮猜拳的结果。

两个整数为小朋友的编号，`a>b` 表示 aa 赢了 bb，`a=b` 表示 aa 和 bb 平手，`a<b` 表示 aa 输给了 bb。

#### 输出格式

每组测试用例输出一行结果，如果可以找到裁判，且只有一个人可能是裁判，则输出裁判编号和确定轮数。

如果可以找到裁判，但裁判的可能人选多于 11 个，则输出 `Can not determine`。

如果根据输入推断的结果是必须没有裁判或者必须有多个裁判，则输出 `Impossible`。

具体格式可参考样例。

#### 数据范围

1≤N≤5001≤N≤500,  
0≤M≤20000≤M≤2000

#### 输入样例：

    3 3
    0<1
    1<2
    2<0
    3 5
    0<1
    0>1
    1<2
    1>2
    0<2
    4 4
    0<1
    0>1
    2<3
    2>3
    1 0
    

#### 输出样例：

    Can not determine
    Player 1 can be determined to be the judge after 4 lines
    Impossible
    Player 0 can be determined to be the judge after 0 lines
    

难度：中等

时/空限制：1s / 64MB

总通过数：1188

总尝试数：3029

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3809&show_algorithm_tags=0)[POJ2912](https://www.acwing.com/problem/search/1/?search_content=POJ2912&source_file_id=3809&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3809&show_algorithm_tags=0)

算法标签

[并查集](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%B6%E6%9F%A5%E9%9B%86&source_file_id=3809&show_algorithm_tags=1)[扩展域或边带权](https://www.acwing.com/problem/search/1/?search_content=%E6%89%A9%E5%B1%95%E5%9F%9F%E6%88%96%E8%BE%B9%E5%B8%A6%E6%9D%83&source_file_id=3809&show_algorithm_tags=1)