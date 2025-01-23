160\. 匹配统计

*    [题目](https://www.acwing.com/problem/content/description/162/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/162/1/)
*    [题解](https://www.acwing.com/problem/content/solution/162/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/162/)

  

阿轩在纸上写了两个字符串，分别记为 AA 和 BB。

利用在数据结构与算法课上学到的知识，他很容易地求出了“字符串 AA 从任意位置开始的后缀子串”与“字符串 BB”匹配的长度。

不过阿轩是一个勤学好问的同学，他向你提出了 QQ 个问题：

在每个问题中，他给定你一个整数 xx，请你告诉他有多少个位置，满足“字符串 AA 从该位置开始的后缀子串”与 BB 匹配的长度恰好为 xx。

例如：A\=aabcde，B\=abA\=aabcde，B\=ab，则 AA 有 aabcde、abcde、bcde、cde、de、eaabcde、abcde、bcde、cde、de、e 这 66 个后缀子串，它们与 B\=abB\=ab 的匹配长度分别是 1、2、0、0、0、01、2、0、0、0、0。

因此 AA 有 44 个位置与 BB 的匹配长度恰好为 00，有 11 个位置的匹配长度恰好为 11，有 11 个位置的匹配长度恰好为 22。

#### 输入格式

第一行输入三个整数 N,M,QN,M,Q，分别表示 AA 串长度、BB 串长度、问题个数。

第二行输入字符串 AA，第三行输入字符串 BB。

接下来 QQ 行每行输入 11 个整数 xx，表示一个问题。

#### 输出格式

输出共 QQ 行，依次表示每个问题的答案。

#### 数据范围

1≤N,M,Q,x≤2000001≤N,M,Q,x≤200000

#### 输入样例：

    6 2 5
    aabcde
    ab
    0
    1
    2
    3
    4
    

#### 输出样例：

    4
    1
    1
    0
    0
    

难度：中等

时/空限制：1s / 64MB

总通过数：1873

总尝试数：3646

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3711&show_algorithm_tags=0)

算法标签

[字符串hash](https://www.acwing.com/problem/search/1/?search_content=%E5%AD%97%E7%AC%A6%E4%B8%B2hash&source_file_id=3711&show_algorithm_tags=1)[哈希](https://www.acwing.com/problem/search/1/?search_content=%E5%93%88%E5%B8%8C&source_file_id=3711&show_algorithm_tags=1)[KMP模式匹配](https://www.acwing.com/problem/search/1/?search_content=KMP%E6%A8%A1%E5%BC%8F%E5%8C%B9%E9%85%8D&source_file_id=3711&show_algorithm_tags=1)