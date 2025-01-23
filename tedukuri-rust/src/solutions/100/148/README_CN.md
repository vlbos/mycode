148\. 合并果子

*    [题目](https://www.acwing.com/problem/content/description/150/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/150/1/)
*    [题解](https://www.acwing.com/problem/content/solution/150/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/150/)

  

在一个果园里，达达已经将所有的果子打了下来，而且按果子的不同种类分成了不同的堆。

达达决定把所有的果子合成一堆。

每一次合并，达达可以把两堆果子合并到一起，消耗的体力等于两堆果子的重量之和。

可以看出，所有的果子经过 n−1n−1 次合并之后，就只剩下一堆了。

达达在合并果子时总共消耗的体力等于每次合并所耗体力之和。

因为还要花大力气把这些果子搬回家，所以达达在合并果子时要尽可能地节省体力。

假定每个果子重量都为 11，并且已知果子的种类数和每种果子的数目，你的任务是设计出合并的次序方案，使达达耗费的体力最少，并输出这个最小的体力耗费值。

例如有 33 种果子，数目依次为 1，2，91，2，9。

可以先将 1、21、2 堆合并，新堆数目为 33，耗费体力为 33。

接着，将新堆与原先的第三堆合并，又得到新的堆，数目为 1212，耗费体力为 1212。

所以达达总共耗费体力\=3+12\=15\=3+12\=15。

可以证明 1515 为最小的体力耗费值。

#### 输入格式

输入包括两行，第一行是一个整数 nn，表示果子的种类数。

第二行包含 nn 个整数，用空格分隔，第 ii 个整数 aiai 是第 ii 种果子的数目。

#### 输出格式

输出包括一行，这一行只包含一个整数，也就是最小的体力耗费值。

输入数据保证这个值小于 231231。

#### 数据范围

1≤n≤100001≤n≤10000,  
1≤ai≤200001≤ai≤20000

#### 输入样例：

    3 
    1 2 9 
    

#### 输出样例：

    15
    

难度：简单

时/空限制：1s / 64MB

总通过数：37075

总尝试数：52866

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3699&show_algorithm_tags=0)[NOIP2004提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2004%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3699&show_algorithm_tags=0)[模板题](https://www.acwing.com/problem/search/1/?search_content=%E6%A8%A1%E6%9D%BF%E9%A2%98&source_file_id=3699&show_algorithm_tags=0)[《信息学奥赛一本通》语言及算法基础篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E8%AF%AD%E8%A8%80%E5%8F%8A%E7%AE%97%E6%B3%95%E5%9F%BA%E7%A1%80%E7%AF%87&source_file_id=3699&show_algorithm_tags=0)

算法标签

[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3699&show_algorithm_tags=1)[Huffman树](https://www.acwing.com/problem/search/1/?search_content=Huffman%E6%A0%91&source_file_id=3699&show_algorithm_tags=1)