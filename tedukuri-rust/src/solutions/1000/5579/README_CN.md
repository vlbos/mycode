5579\. 增加模数

*    [题目](https://www.acwing.com/problem/content/description/5582/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/5582/1/)
*    [题解](https://www.acwing.com/problem/content/solution/5582/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/5582/)

  

给定 HH 对非负整数数对 (Ai,Bi)(Ai,Bi) 和一个正整数 MM。

请你计算并输出 (AB11+AB22+…+ABHH)modM(A1B1+A2B2+…+AHBH)modM。

#### 输入格式

第一行包含整数 TT，表示共有 TT 组测试数据。

每组数据第一行包含整数 MM。

第二行包含整数 HH。

接下来 HH 行，每行包含两个整数 Ai,BiAi,Bi。

#### 输出格式

每组数据输出一行结果。

#### 数据范围

1≤T≤1001≤T≤100,  
1≤M≤450001≤M≤45000,  
1≤H≤450001≤H≤45000,  
0≤Ai,Bi≤1070≤Ai,Bi≤107,  
AiAi 和 BiBi 不同时为 00。

#### 输入样例：

    3
    16
    4
    2 3
    3 4
    4 5
    5 6
    36123
    1
    2374859 3029382
    17
    1
    3 18132
    

#### 输出样例：

    2
    13195
    13
    

难度：简单

时/空限制：3s / 128MB

总通过数：1836

总尝试数：7445

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=11655622&show_algorithm_tags=0)[POJ1995](https://www.acwing.com/problem/search/1/?search_content=POJ1995&source_file_id=11655622&show_algorithm_tags=0)

算法标签

[快速幂](https://www.acwing.com/problem/search/1/?search_content=%E5%BF%AB%E9%80%9F%E5%B9%82&source_file_id=11655622&show_algorithm_tags=1)