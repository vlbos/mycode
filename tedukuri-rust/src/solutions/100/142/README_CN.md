142\. 前缀统计

*    [题目](https://www.acwing.com/problem/content/description/144/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/144/1/)
*    [题解](https://www.acwing.com/problem/content/solution/144/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/144/)

  

给定 NN 个字符串 S1,S2…SNS1,S2…SN，接下来进行 MM 次询问，每次询问给定一个字符串 TT，求 S1∼SNS1∼SN 中有多少个字符串是 TT 的前缀。

输入字符串的总长度不超过 106106，仅包含小写字母。

#### 输入格式

第一行输入两个整数 N，MN，M。

接下来 NN 行每行输入一个字符串 SiSi。

接下来 MM 行每行一个字符串 TT 用以询问。

#### 输出格式

对于每个询问，输出一个整数表示答案。

每个答案占一行。

#### 数据范围

1≤N,M≤1051≤N,M≤105

#### 输入样例：

    3 2
    ab
    bc
    abc
    abc
    efg
    

#### 输出样例：

    2
    0
    

难度：简单

时/空限制：1s / 256MB

总通过数：8043

总尝试数：14906

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3693&show_algorithm_tags=0)

算法标签

[Trie](https://www.acwing.com/problem/search/1/?search_content=Trie&source_file_id=3693&show_algorithm_tags=1)[字典树](https://www.acwing.com/problem/search/1/?search_content=%E5%AD%97%E5%85%B8%E6%A0%91&source_file_id=3693&show_algorithm_tags=1)