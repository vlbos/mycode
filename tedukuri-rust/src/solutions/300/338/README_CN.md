338\. 计数问题

*    [题目](https://www.acwing.com/problem/content/description/340/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/340/1/)
*    [题解](https://www.acwing.com/problem/content/solution/340/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/340/)

  

给定两个整数 aa 和 bb，求 aa 和 bb 之间的所有数字中 0∼90∼9 的出现次数。

例如，a\=1024，b\=1032a\=1024，b\=1032，则 aa 和 bb 之间共有 99 个数如下：

`1024 1025 1026 1027 1028 1029 1030 1031 1032`

其中 `0` 出现 1010 次，`1` 出现 1010 次，`2` 出现 77 次，`3` 出现 33 次等等…

#### 输入格式

输入包含多组测试数据。

每组测试数据占一行，包含两个整数 aa 和 bb。

当读入一行为 `0 0` 时，表示输入终止，且该行不作处理。

#### 输出格式

每组数据输出一个结果，每个结果占一行。

每个结果包含十个用空格隔开的数字，第一个数字表示 `0` 出现的次数，第二个数字表示 `1` 出现的次数，以此类推。

#### 数据范围

0<a,b<1000000000<a,b<100000000

#### 输入样例：

    1 10
    44 497
    346 542
    1199 1748
    1496 1403
    1004 503
    1714 190
    1317 854
    1976 494
    1001 1960
    0 0
    

#### 输出样例：

    1 2 1 1 1 1 1 1 1 1
    85 185 185 185 190 96 96 96 95 93
    40 40 40 93 136 82 40 40 40 40
    115 666 215 215 214 205 205 154 105 106
    16 113 19 20 114 20 20 19 19 16
    107 105 100 101 101 197 200 200 200 200
    413 1133 503 503 503 502 502 417 402 412
    196 512 186 104 87 93 97 97 142 196
    398 1375 398 398 405 499 499 495 488 471
    294 1256 296 296 296 296 287 286 286 247
    

难度：中等

时/空限制：1s / 64MB

总通过数：26265

总尝试数：39607

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3889&show_algorithm_tags=0)[模板题](https://www.acwing.com/problem/search/1/?search_content=%E6%A8%A1%E6%9D%BF%E9%A2%98&source_file_id=3889&show_algorithm_tags=0)[UVA1640](https://www.acwing.com/problem/search/1/?search_content=UVA1640&source_file_id=3889&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3889&show_algorithm_tags=1)[数位统计DP](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E4%BD%8D%E7%BB%9F%E8%AE%A1DP&source_file_id=3889&show_algorithm_tags=1)