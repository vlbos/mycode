370\. 卡图难题

*    [题目](https://www.acwing.com/problem/content/description/372/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/372/1/)
*    [题解](https://www.acwing.com/problem/content/solution/372/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/372/)

  

有 NN 个变量 X0∼XN−1X0∼XN−1，每个变量的可能取值为 00 或 11。

给定 MM 个算式，每个算式形如 Xa op Xb\=cXa op Xb\=c，其中 a,ba,b 是变量编号，cc 是数字 00 或 11，opop 是 ANDAND,OROR,XORXOR 三个位运算之一。

求是否存在对每个变量的合法赋值，使所有算式都成立。

#### 输入格式

第一行包含两个整数 NN 和 MM。

接下来 MM 行，每行包含三个整数 a,b,ca,b,c，以及一个位运算（ANDAND,OROR,XORXOR 中的一个）。

#### 输出格式

输出结果，如果存在，输出 `YES`，否则输出 `NO`。

#### 数据范围

1≤N≤10001≤N≤1000,  
1≤M≤1061≤M≤106

#### 输入样例：

    4 4
    0 1 1 AND
    1 2 1 OR
    3 2 0 AND
    3 0 0 XOR
    

#### 输出样例：

    YES
    

难度：中等

时/空限制：1s / 64MB

总通过数：801

总尝试数：1550

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3921&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3921&show_algorithm_tags=0)[POJ3678](https://www.acwing.com/problem/search/1/?search_content=POJ3678&source_file_id=3921&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3921&show_algorithm_tags=1)[2-SAT问题](https://www.acwing.com/problem/search/1/?search_content=2-SAT%E9%97%AE%E9%A2%98&source_file_id=3921&show_algorithm_tags=1)