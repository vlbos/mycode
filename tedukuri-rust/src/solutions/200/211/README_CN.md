211\. 计算系数

*    [题目](https://www.acwing.com/problem/content/description/213/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/213/1/)
*    [题解](https://www.acwing.com/problem/content/solution/213/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/213/)

  

给定一个多项式 (ax+by)k(ax+by)k，请求出多项式展开后 xnymxnym 项的系数。

#### 输入格式

共一行，包含 55 个整数，分别为 a，b，k，n，ma，b，k，n，m，每两个整数之间用一个空格隔开。

#### 输出格式

输出共 11 行，包含一个整数，表示所求的系数，这个系数可能很大，输出对 1000710007 取模后的结果。

#### 数据范围

0≤n,m≤k≤10000≤n,m≤k≤1000,  
n+m\=kn+m\=k,  
0≤a,b≤1060≤a,b≤106

#### 输入样例：

    1 1 3 1 2 
    

#### 输出样例：

    3
    

难度：简单

时/空限制：1s / 64MB

总通过数：4242

总尝试数：9000

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3762&show_algorithm_tags=0)[NOIP2011提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2011%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3762&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3762&show_algorithm_tags=0)

算法标签

[二项式定理](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E9%A1%B9%E5%BC%8F%E5%AE%9A%E7%90%86&source_file_id=3762&show_algorithm_tags=1)[组合计数](https://www.acwing.com/problem/search/1/?search_content=%E7%BB%84%E5%90%88%E8%AE%A1%E6%95%B0&source_file_id=3762&show_algorithm_tags=1)