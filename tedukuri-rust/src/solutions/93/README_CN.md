93\. 递归实现组合型枚举

*    [题目](https://www.acwing.com/problem/content/description/95/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/95/1/)
*    [题解](https://www.acwing.com/problem/content/solution/95/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/95/)

  

从 1∼n1∼n 这 nn 个整数中随机选出 mm 个，输出所有可能的选择方案。

#### 输入格式

两个整数 n,mn,m ,在同一行用空格隔开。

#### 输出格式

按照从小到大的顺序输出所有方案，每行 11 个。

首先，同一行内的数升序排列，相邻两个数用一个空格隔开。

其次，对于两个不同的行，对应下标的数一一比较，字典序较小的排在前面（例如 `1 3 5 7` 排在 `1 3 6 8` 前面）。

#### 数据范围

n\>0n\>0 ,  
0≤m≤n0≤m≤n ,  
n+(n−m)≤25n+(n−m)≤25

#### 输入样例：

    5 3
    

#### 输出样例：

    1 2 3 
    1 2 4 
    1 2 5 
    1 3 4 
    1 3 5 
    1 4 5 
    2 3 4 
    2 3 5 
    2 4 5 
    3 4 5 
    

**思考题**：如果要求使用非递归方法，该怎么做呢？

难度：简单

时/空限制：5s / 256MB

总通过数：65660

总尝试数：94177

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3644&show_algorithm_tags=0)[《信息学奥赛一本通》语言及算法基础篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E8%AF%AD%E8%A8%80%E5%8F%8A%E7%AE%97%E6%B3%95%E5%9F%BA%E7%A1%80%E7%AF%87&source_file_id=3644&show_algorithm_tags=0)

算法标签

[递归](https://www.acwing.com/problem/search/1/?search_content=%E9%80%92%E5%BD%92&source_file_id=3644&show_algorithm_tags=1)[DFS](https://www.acwing.com/problem/search/1/?search_content=DFS&source_file_id=3644&show_algorithm_tags=1)