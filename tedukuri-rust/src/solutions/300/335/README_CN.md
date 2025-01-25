335\. 特别行动队

*    [题目](https://www.acwing.com/problem/content/description/337/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/337/1/)
*    [题解](https://www.acwing.com/problem/content/solution/337/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/337/)

  

你有一支由 nn 名士兵组成的部队，士兵从 11 到 nn 编号，要将他们拆分成若干个特别行动队调入战场。

出于默契的考虑，同一支行动队的队员的编号应该连续。

编号为 ii 的士兵的初始战斗力为 xixi，一支行动队的初始战斗力为队内所有队员初始战斗力之和。

通过长期观察，你总结出一支特别行动队的初始战斗力 xx 将按如下公式修正为 x′x′:

x′\=ax2+bx+cx′\=ax2+bx+c

其中，a,b,ca,b,c 是已知的系数(a<0a<0)。

作为部队统帅，你要为这支部队进行编队，使得所有特别行动队修正后的战斗力之和最大。

试求出这个最大和。

#### 输入格式

第一行包含一个整数 nn，表示士兵总数。

第二行包含三个整数 a,b,ca,b,c。

第三行包含 nn 个整数 x1,x2,…,xnx1,x2,…,xn,表示每名士兵的初始战斗力。

#### 输出格式

输出一个整数，表示战斗力之和的最大值。

#### 数据范围

1≤n≤10000001≤n≤1000000,  
−5≤a≤−1−5≤a≤−1,  
|b|,|c|≤107|b|,|c|≤107,  
1≤xi≤1001≤xi≤100

#### 输入样例：

    4 
    -1 10 -20 
    2 2 3 4 
    

#### 输出样例：

    9
    

难度：简单

时/空限制：1s / 64MB

总通过数：500

总尝试数：1052

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3886&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3886&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3886&show_algorithm_tags=1)[斜率优化](https://www.acwing.com/problem/search/1/?search_content=%E6%96%9C%E7%8E%87%E4%BC%98%E5%8C%96&source_file_id=3886&show_algorithm_tags=1)