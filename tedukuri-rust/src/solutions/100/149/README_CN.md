149\. 荷马史诗

*    [题目](https://www.acwing.com/problem/content/description/151/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/151/1/)
*    [题解](https://www.acwing.com/problem/content/solution/151/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/151/)

  

追逐影子的人，自己就是影子。 ——荷马

达达最近迷上了文学。

她喜欢在一个慵懒的午后，细细地品上一杯卡布奇诺，静静地阅读她爱不释手的《荷马史诗》。

但是由《奥德赛》和《伊利亚特》组成的鸿篇巨制《荷马史诗》实在是太长了，达达想通过一种编码方式使得它变得短一些。

一部《荷马史诗》中有 nn 种不同的单词，从 11 到 nn 进行编号。其中第 ii 种单词出现的总次数为 wiwi。

达达想要用 kk 进制串 sisi 来替换第 ii 种单词，使得其满足如下要求:

对于任意的 1≤i,j≤n，i≠j1≤i,j≤n，i≠j，都有：sisi 不是 sjsj 的前缀。

现在达达想要知道，如何选择 sisi，才能使替换以后得到的新的《荷马史诗》长度最小。

在确保总长度最小的情况下，达达还想知道最长的 sisi 的最短长度是多少？

一个字符串被称为 kk 进制字符串，当且仅当它的每个字符是 00 到 k−1k−1 之间（包括 00 和 k−1k−1）的整数。

字符串 Str1Str1 被称为字符串 Str2Str2 的前缀，当且仅当：存在 1≤t≤m1≤t≤m，使得 Str1\=Str2\[1..t\]Str1\=Str2\[1..t\]。

其中，mm 是字符串 Str2Str2 的长度，Str2\[1..t\]Str2\[1..t\] 表示 Str2Str2 的前 tt 个字符组成的字符串。

**注意**:请使用 6464 位整数进行输入输出、储存和计算。

#### 输入格式

输入文件的第 11 行包含 22 个正整数 n,kn,k，中间用单个空格隔开，表示共有 nn 种单词，需要使用 kk 进制字符串进行替换。

第 2∼n+12∼n+1 行：第 i+1i+1 行包含 11 个非负整数 wiwi，表示第 ii 种单词的出现次数。

#### 输出格式

输出文件包括 22 行。

第 11 行输出 11 个整数，为《荷马史诗》经过重新编码以后的最短长度。

第 22 行输出 11 个整数，为保证最短总长度的情况下，最长字符串 sisi 的最短长度。

#### 数据范围

2≤n≤1000002≤n≤100000,  
2≤k≤92≤k≤9  
1≤wi≤10121≤wi≤1012

#### 输入样例：

    4 2
    1
    1
    2
    2
    

#### 输出样例：

    12
    2
    

难度：简单

时/空限制：1s / 64MB

总通过数：3554

总尝试数：7213

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3700&show_algorithm_tags=0)[NOI2015](https://www.acwing.com/problem/search/1/?search_content=NOI2015&source_file_id=3700&show_algorithm_tags=0)

算法标签

[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3700&show_algorithm_tags=1)[二叉堆](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%8F%89%E5%A0%86&source_file_id=3700&show_algorithm_tags=1)[Huffman树](https://www.acwing.com/problem/search/1/?search_content=Huffman%E6%A0%91&source_file_id=3700&show_algorithm_tags=1)