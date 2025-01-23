184\. 虫食算

*    [题目](https://www.acwing.com/problem/content/description/186/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/186/1/)
*    [题解](https://www.acwing.com/problem/content/solution/186/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/186/)

  

所谓虫食算，就是原先的算式中有一部分被虫子啃掉了，需要我们根据剩下的数字来判定被啃掉的字母。

来看一个简单的例子：

     43#9865#045
    +  8468#6633
    --------------
     44445509678
    

其中 `#` 号代表被虫子啃掉的数字。

根据算式，我们很容易判断：第一行的两个数字分别是 55 和 33，第二行的数字是 55。

现在，我们对问题做两个限制：

首先，我们只考虑加法的虫食算。这里的加法是 NN 进制加法，算式中三个数都有 NN 位，允许有前导的 00。

其次，虫子把所有的数都啃光了，我们只知道哪些数字是相同的，我们将相同的数字用相同的字母表示，不同的数字用不同的字母表示。

如果这个算式是 NN 进制的，我们就取英文字母表的前 NN 个大写字母来表示这个算式中的 00 到 N−1N−1 这 NN 个不同的数字：但是这 NN 个字母并不一定顺序地代表 00 到 N−1N−1。

输入数据保证 NN 个字母分别至少出现一次。

       BADC
    +  CBDA
    ----------
       DCCC
    

上面的算式是一个 44 进制的算式。

很显然，我们只要让 ABCDABCD 分别代表 01230123，便可以让这个式子成立了。

你的任务是，对于给定的 NN 进制加法算式，求出 NN 个不同的字母分别代表的数字，使得该加法算式成立。

输入数据保证有且仅有一组解。

#### 输入格式

输入包含 44 行。

第一行有一个正整数 N(N≤26)N(N≤26)，后面的 33 行每行有一个由大写字母组成的字符串，分别代表两个加数以及和。

这 33 个字符串左右两端都没有空格，并且恰好有 NN 位。

#### 输出格式

输出包含一行。

在这一行中，应当包含唯一的那组解。

解是这样表示的：输出 NN 个数字，分别表示 A，B，C……A，B，C…… 所代表的数字，相邻的两个数字用一个空格隔开，不能有多余的空格。

#### 输入样例：

    5
    ABCED
    BDACE
    EBBAA
    

#### 输出样例：

    1 0 3 4 2
    

难度：简单

时/空限制：2s / 64MB

总通过数：1916

总尝试数：4787

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3735&show_algorithm_tags=0)[NOIP2004提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2004%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3735&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3735&show_algorithm_tags=1)[DFS](https://www.acwing.com/problem/search/1/?search_content=DFS&source_file_id=3735&show_algorithm_tags=1)[剪枝](https://www.acwing.com/problem/search/1/?search_content=%E5%89%AA%E6%9E%9D&source_file_id=3735&show_algorithm_tags=1)