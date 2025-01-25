212\. 计数交换

*    [题目](https://www.acwing.com/problem/content/description/214/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/214/1/)
*    [题解](https://www.acwing.com/problem/content/solution/214/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/214/)

  

给定一个 1∼n1∼n 的排列 p1,p2,…,pnp1,p2,…,pn，可进行若干次操作，每次选择两个整数 x,yx,y，交换 px,pypx,py。

设把 p1,p2,…,pnp1,p2,…,pn 变成单调递增的排列 1,2,…,n1,2,…,n 至少需要 mm 次交换。

求有多少种操作方法可以只用 mm 次交换达到上述目标。

因为结果可能很大，你只需要输出结果对 109+9109+9 取模之后的值。

例如排列 2,3,12,3,1 至少需要 22 次交换才能变为 1,2,31,2,3。

操作方法共有 33 种，分别是：

方法一：先交换数字 2,32,3，变成 3,2,13,2,1，再交换数字 3,13,1，变成 1,2,31,2,3。  
方法二：先交换数字 2,12,1，变成 1,3,21,3,2，再交换数字 3,23,2，变成 1,2,31,2,3。  
方法三：先交换数字 3,13,1，变成 2,1,32,1,3，再交换数字 2,12,1，变成 1,2,31,2,3。

#### 输入格式

第一行包含整数 TT，表示一共有 TT 组测试用例。

每个测试用例前都会有一个空行。

每个测试用例包含两行，第一行包含整数 nn。

第二行包含 nn 个整数，表示序列 p1,p2,…,pnp1,p2,…,pn。

#### 输出格式

每个测试用例输出一个结果，每个结果占一行。

#### 数据范围

1≤n≤1051≤n≤105

#### 输入样例：

    3
    
    3
    2 3 1
    
    4
    2 1 4 3
    
    2
    1 2
    

#### 输出样例：

    3
    2
    1
    

难度：简单

时/空限制：1s / 64MB

总通过数：860

总尝试数：2117

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3763&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3763&show_algorithm_tags=1)[组合计数](https://www.acwing.com/problem/search/1/?search_content=%E7%BB%84%E5%90%88%E8%AE%A1%E6%95%B0&source_file_id=3763&show_algorithm_tags=1)