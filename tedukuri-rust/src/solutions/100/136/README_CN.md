136\. 邻值查找

*    [题目](https://www.acwing.com/problem/content/description/138/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/138/1/)
*    [题解](https://www.acwing.com/problem/content/solution/138/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/138/)

  

给定一个长度为 nn 的序列 AA，AA 中的数各不相同。

对于 AA 中的每一个数 AiAi，求：

min1≤j<i|Ai−Aj|min1≤j<i|Ai−Aj|

以及令上式取到最小值的 jj（记为 PiPi）。若最小值点不唯一，则选择使 AjAj 较小的那个。

#### 输入格式

第一行输入整数 nn，代表序列长度。

第二行输入 nn 个整数A1…AnA1…An,代表序列的具体数值，数值之间用空格隔开。

#### 输出格式

输出共 n−1n−1 行，每行输出两个整数，数值之间用空格隔开。

分别表示当 ii 取 2∼n2∼n 时，对应的 min1≤j<i|Ai−Aj|min1≤j<i|Ai−Aj| 和 PiPi 的值。

#### 数据范围

n≤105n≤105,|Ai|≤109|Ai|≤109

#### 输入样例：

    3
    1 5 3
    

#### 输出样例：

    4 1
    2 1
    

难度：中等

时/空限制：1s / 64MB

总通过数：7447

总尝试数：18909

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3687&show_algorithm_tags=0)

算法标签

[链表](https://www.acwing.com/problem/search/1/?search_content=%E9%93%BE%E8%A1%A8&source_file_id=3687&show_algorithm_tags=1)[STL Map](https://www.acwing.com/problem/search/1/?search_content=STL%20Map&source_file_id=3687&show_algorithm_tags=1)[STL Set](https://www.acwing.com/problem/search/1/?search_content=STL%20Set&source_file_id=3687&show_algorithm_tags=1)