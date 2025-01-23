145\. 超市

*    [题目](https://www.acwing.com/problem/content/description/147/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/147/1/)
*    [题解](https://www.acwing.com/problem/content/solution/147/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/147/)

  

超市里有 NN 件商品，每件商品都有利润 pipi 和过期时间 didi，每天只能卖一件商品，过期商品不能再卖。

求合理安排每天卖的商品的情况下，可以得到的最大收益是多少。

#### 输入格式

输入包含多组测试用例。

每组测试用例，以输入整数 NN 开始，接下来输入 NN 对 pipi 和 didi，分别代表第 ii 件商品的利润和过期时间。

在输入中，数据之间可以自由穿插任意个空格或空行，输入至文件结尾时终止输入，保证数据正确。

#### 输出格式

对于每组产品，输出一个该组的最大收益值。

每个结果占一行。

#### 数据范围

0≤N≤100000≤N≤10000,  
1≤pi,di≤100001≤pi,di≤10000  
最多有 1414 组测试样例

#### 输入样例：

    4  50 2  10 1   20 2   30 1
    
    7  20 1   2 1   10 3  100 2   8 2
       5 20  50 10
    

#### 输出样例：

    80
    185
    

难度：简单

时/空限制：1s / 64MB

总通过数：7673

总尝试数：15264

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3696&show_algorithm_tags=0)[POJ1456](https://www.acwing.com/problem/search/1/?search_content=POJ1456&source_file_id=3696&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3696&show_algorithm_tags=0)

算法标签

[二叉堆](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%8F%89%E5%A0%86&source_file_id=3696&show_algorithm_tags=1)[并查集](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%B6%E6%9F%A5%E9%9B%86&source_file_id=3696&show_algorithm_tags=1)[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3696&show_algorithm_tags=1)