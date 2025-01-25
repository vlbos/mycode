409\. 空袭

*    [题目](https://www.acwing.com/problem/content/description/411/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/411/1/)
*    [题解](https://www.acwing.com/problem/content/solution/411/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/411/)

  

一个城镇由 NN 个交叉口和 MM 个单向道路组成，每条道路从一个交叉口通向另一个交叉口。

从任何一个交叉口出发，经过城镇的街道都无法回到出发的交叉口，即城镇的街道无法形成一个循环。

现在要选择一些交叉口空降士兵，士兵可以沿着街道一直走到尽头。

要求空降士兵走遍所有的交叉口，并且每个交叉口只能被一个士兵走一次。

请问最少需要多少名士兵。

#### 输入格式

第一行包含整数 TT，表示共有 TT 组测试数据。

每组测试数据第一行包含整数 NN，第二行包含整数 MM。

接下来 MM 行，每行包含两个整数 XX 和 YY，表示存在一条道路从交叉口 XX 通向交叉口 YY。

#### 输出格式

每组数据输出一个整数，表示需要的最少士兵数目。

每个结果占一行。

#### 数据范围

0<N≤1200<N≤120,  
1≤X,Y≤N1≤X,Y≤N

#### 输入样例：

    2
    4
    3
    3 4
    1 3
    2 3
    3
    3
    1 3
    1 2
    2 3
    

#### 输出样例：

    2
    1
    

难度：简单

时/空限制：1s / 64MB

总通过数：314

总尝试数：463

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3960&show_algorithm_tags=0)[HDU1151](https://www.acwing.com/problem/search/1/?search_content=HDU1151&source_file_id=3960&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3960&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3960&show_algorithm_tags=1)[二分图最小路径点覆盖](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86%E5%9B%BE%E6%9C%80%E5%B0%8F%E8%B7%AF%E5%BE%84%E7%82%B9%E8%A6%86%E7%9B%96&source_file_id=3960&show_algorithm_tags=1)