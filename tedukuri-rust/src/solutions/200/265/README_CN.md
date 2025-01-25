265\. 营业额统计

*    [题目](https://www.acwing.com/problem/content/description/267/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/267/1/)
*    [题解](https://www.acwing.com/problem/content/solution/267/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/267/)

  

Tiger 最近被公司升任为营业部经理，他上任后接受公司交给的第一项任务便是统计并分析公司成立以来的营业情况。

Tiger 拿出了公司的账本，账本上记录了公司成立以来每天的营业额。

分析营业情况是一项相当复杂的工作。

由于节假日，大减价或者是其他情况的时候，营业额会出现一定的波动，当然一定的波动是能够接受的，但是在某些时候营业额突变得很高或是很低，这就证明公司此时的经营状况出现了问题。

经济管理学上定义了一种最小波动值来衡量这种情况。

设第 ii 天的营业额为 aiai，则第 ii 天(i≥2i≥2)的最小波动值 fifi 被定义为：

fi\=min1≤j<i|ai−aj|fi\=min1≤j<i|ai−aj|

当最小波动值越大时，就说明营业情况越不稳定。

而分析整个公司的从成立到现在营业情况是否稳定，只需要把每一天的最小波动值加起来就可以了。

你的任务就是编写一个程序帮助 Tiger 来计算这一个值。

第一天的最小波动值为第一天的营业额 a1a1。

#### 输入格式

第一行为正整数 nn，表示该公司从成立一直到现在的天数。

接下来的 nn 行每行有一个整数 aiai(有可能有负数) ，表示第 ii 天公司的营业额。

#### 输出格式

输出一个正整数，表示最小波动值的和。

#### 数据范围

1≤n≤32767,|ai|≤1061≤n≤32767,|ai|≤106

#### 输入样例：

    6
    5
    1
    2
    5
    4
    6
    

#### 输出样例：

    12
    

#### 样例解释

在样例中，5+|1−5|+|2−1|+|5−5|+|4−5|+|6−5|\=5+4+1+0+1+1\=125+|1−5|+|2−1|+|5−5|+|4−5|+|6−5|\=5+4+1+0+1+1\=12。

难度：简单

时/空限制：1s / 64MB

总通过数：4550

总尝试数：7005

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3816&show_algorithm_tags=0)[HNOI2002](https://www.acwing.com/problem/search/1/?search_content=HNOI2002&source_file_id=3816&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3816&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3816&show_algorithm_tags=0)

算法标签

[平衡树](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%B3%E8%A1%A1%E6%A0%91&source_file_id=3816&show_algorithm_tags=1)[Treap](https://www.acwing.com/problem/search/1/?search_content=Treap&source_file_id=3816&show_algorithm_tags=1)