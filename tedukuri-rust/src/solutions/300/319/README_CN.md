319\. 折叠序列

*    [题目](https://www.acwing.com/problem/content/description/321/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/321/1/)
*    [题解](https://www.acwing.com/problem/content/solution/321/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/321/)

  

比尔正在试图用折叠重复子序列的方式紧凑的表示由大写字母 `A` 到 `Z` 组成的字符序列。

例如，表示序列 `AAAAAAAAAABABABCCD` 的一种方式是 `10(A)2(BA)B2(C)D`。

他通过以下方式定义了折叠的字符序列以及它们的展开变换：

1.  包含单个字符的序列被认为是折叠序列，展开它得到的序列为它本身。
2.  如果 SS 和 QQ 是两个折叠序列，并且 SS 可以展开得到 S′S′，QQ 可以展开得到 Q′Q′，则认为 SQSQ 也是一个折叠序列，并且 SQSQ 展开得到 S′Q′S′Q′。
3.  如果 SS 是折叠序列，则 X(S)X(S) 也是折叠序列，其中 XX 为大于 11 的整数。如果 SS 展开得到 S′S′，则 X(S)X(S) 展开得到 XX 个 S′S′。

根据定义可以展开任意给出的折叠序列，现在给出原序列，请你将它折叠，并使得折叠序列包含尽可能少的字符数。

#### 输入格式

输入包含一行由大写字母构成的字符序列，序列长度在 11 到 100100 之间。

#### 输出格式

输出包含字符数最少的折叠序列，如果答案不唯一则任意输出一个即可。

#### 输入样例：

    AAAAAAAAAABABABCCD
    

#### 输出样例：

    9(A)3(AB)CCD
    

难度：中等

时/空限制：1s / 64MB

总通过数：776

总尝试数：1448

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3870&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3870&show_algorithm_tags=1)[区间DP](https://www.acwing.com/problem/search/1/?search_content=%E5%8C%BA%E9%97%B4DP&source_file_id=3870&show_algorithm_tags=1)