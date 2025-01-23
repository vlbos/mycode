141\. 周期

*    [题目](https://www.acwing.com/problem/content/description/143/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/143/1/)
*    [题解](https://www.acwing.com/problem/content/solution/143/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/143/)

  

一个字符串的前缀是从第一个字符开始的连续若干个字符，例如 `abaab` 共有 55 个前缀，分别是 `a`，`ab`，`aba`，`abaa`，`abaab`。

我们希望知道一个 NN 位字符串 SS 的前缀是否具有循环节。

换言之，对于每一个从头开始的长度为 ii（i\>1i\>1）的前缀，是否由重复出现的子串 AA 组成，即 AAA…AAAA…A （AA 重复出现 KK 次,K\>1K\>1）。

如果存在，请找出最短的循环节对应的 KK 值（也就是这个前缀串的所有可能重复节中，最大的 KK 值）。

#### 输入格式

输入包括多组测试数据，每组测试数据包括两行。

第一行输入字符串 SS 的长度 NN。

第二行输入字符串 SS。

输入数据以只包括一个 00 的行作为结尾。

#### 输出格式

对于每组测试数据，第一行输出 `Test case #` 和测试数据的编号。

接下来的每一行，输出具有循环节的前缀的长度 ii 和其对应 KK，中间用一个空格隔开。

前缀长度需要升序排列。

在每组测试数据的最后输出一个空行。

#### 数据范围

2≤N≤10000002≤N≤1000000

#### 输入样例：

    3
    aaa
    4
    abcd
    12
    aabaabaabaab
    0
    

#### 输出样例：

    Test case #1
    2 2
    3 3
    
    Test case #2
    
    Test case #3
    2 2
    6 2
    9 3
    12 4
    
    

难度：简单

时/空限制：1s / 64MB

总通过数：9099

总尝试数：15889

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3692&show_algorithm_tags=0)[HDU1358](https://www.acwing.com/problem/search/1/?search_content=HDU1358&source_file_id=3692&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3692&show_algorithm_tags=0)

算法标签

[KMP](https://www.acwing.com/problem/search/1/?search_content=KMP&source_file_id=3692&show_algorithm_tags=1)