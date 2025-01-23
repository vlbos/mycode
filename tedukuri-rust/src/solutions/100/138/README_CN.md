138\. 兔子与兔子

*    [题目](https://www.acwing.com/problem/content/description/140/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/140/1/)
*    [题解](https://www.acwing.com/problem/content/solution/140/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/140/)

  

很久很久以前，森林里住着一群兔子。

有一天，兔子们想要研究自己的 DNA 序列。

我们首先选取一个好长好长的 DNA 序列（小兔子是外星生物，DNA 序列可能包含 2626 个小写英文字母）。

然后我们每次选择两个区间，询问如果用两个区间里的 DNA 序列分别生产出来两只兔子，这两个兔子是否一模一样。

注意两个兔子一模一样只可能是他们的 DNA 序列一模一样。

#### 输入格式

第一行输入一个 DNA 字符串 SS。

第二行一个数字 mm，表示 mm 次询问。

接下来 mm 行，每行四个数字 l1,r1,l2,r2l1,r1,l2,r2，分别表示此次询问的两个区间，注意字符串的位置从 11 开始编号。

#### 输出格式

对于每次询问，输出一行表示结果。

如果两只兔子完全相同输出 `Yes`，否则输出 `No`（注意大小写）。

#### 数据范围

1≤length(S),m≤10000001≤length(S),m≤1000000

#### 输入样例：

    aabbaabb
    3
    1 3 5 7
    1 3 6 8
    1 2 1 2
    

#### 输出样例：

    Yes
    No
    Yes
    

难度：简单

时/空限制：1s / 64MB

总通过数：8202

总尝试数：18925

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3689&show_algorithm_tags=0)

算法标签

[字符串hash](https://www.acwing.com/problem/search/1/?search_content=%E5%AD%97%E7%AC%A6%E4%B8%B2hash&source_file_id=3689&show_algorithm_tags=1)[哈希](https://www.acwing.com/problem/search/1/?search_content=%E5%93%88%E5%B8%8C&source_file_id=3689&show_algorithm_tags=1)