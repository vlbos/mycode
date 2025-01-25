304\. 诗人小G

*    [题目](https://www.acwing.com/problem/content/description/306/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/306/1/)
*    [题解](https://www.acwing.com/problem/content/solution/306/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/306/)

  

小 GG 是一个出色的诗人，经常作诗自娱自乐。

但是，他一直被一件事情所困扰，那就是诗的排版问题。

一首诗包含了若干个句子，对于一些连续的短句，可以将它们用空格隔开并放在一行中，注意一行中可以放的句子数目是没有限制的。

小 GG 给每首诗定义了一个行标准长度（行的长度为一行中符号的总个数），他希望排版后每行的长度都和行标准长度相差不远。

显然排版时，不应改变原有的句子顺序，并且小 GG 不允许把一个句子分在两行或者更多的行内。

在满足上面两个条件的情况下，小 GG 对于排版中的每行定义了一个不协调度，为这行的实际长度与行标准长度差值绝对值的 PP 次方，而一个排版的不协调度为所有行不协调度的总和。

小 GG 最近又作了几首诗，现在请你对这几首诗进行排版，使得排版后的诗尽量协调（即不协调度尽量小），并把排版的结果告诉他。

#### 输入格式

第一行包含一个整数 TT，表示诗的数量，接下来是 TT 首诗，每首诗是一组数据。

每组数据的第一行包含三个整数 N，LN，L 和 PP，其中 NN 表示这首诗句子的数目，LL 表示这首诗的行标准长度，PP 的含义参考问题描述。

从第二行开始，每行一个句子，句子由英文字母、数字、标点符号等符号组成（ASCII 码 33∼12733∼127，但不包含 `-`）。

#### 输出格式

对于每组测试数据，若最小的不协调度不超过 10181018，则第一行为一个数，表示不协调度。接下来若干行，表示你排版之后的诗。**注意：在同一行的相邻两个句子之间需要用一个空格分开。**

如果有多个可行解，它们的不协调度都是最小值，则**输出任意一个解均可**。（本题有 special judge）（由于本题数据量大，展示标准答案时，不展示可行解）

若最小的不协调度超过 10181018，则输出 `Too hard to arrange`。

每组测试数据结束后输出 `--------------------`，共 2020 个 `-`，`-` 的 ASCII 码为 4545，请勿输出多余的空行或者空格。

#### 数据范围

总共 1010 个测试点，数据范围满足：
| 测试点 | $T$ | $N$ | $L$ | $P$ |
| --- | --- | --- | --- | --- |
| $1$ | $\\le 10$ | $\\le18$ | $\\le 100$ | $\\le5$ |
| $2$ | $\\le 10$ | $\\le 2\\times 10^3$ | $\\le 6\\times 10^4$ | $\\le10$ |
| $3$ | $\\le 10$ | $\\le 2\\times 10^3$ | $\\le 6\\times 10^4$ | $\\le10$ |
| $4$ | $\\le 5$ | $\\le 10^5$ | $\\le 200$ | $\\le10$ |
| $5$ | $\\le 5$ | $\\le 10^5$ | $\\le 200$ | $\\le10$ |
| $6$ | $\\le 5$ | $\\le 10^5$ | $\\le 3\\times 10^6$ | $2$ |
| $7$ | $\\le 5$ | $\\le 10^5$ | $\\le 3\\times 10^6$ | $2$ |
| $8$ | $\\le 5$ | $\\le 10^5$ | $\\le 3\\times 10^6$ | $\\le10$ |
| $9$ | $\\le 5$ | $\\le 10^5$ | $\\le 3\\times 10^6$ | $\\le10$ |
| $10$ | $\\le 5$ | $\\le 10^5$ | $\\le 3\\times 10^6$ | $\\le10$ |

所有测试点中均满足句子长度不超过 3030，P≥1P≥1。

#### 输入样例：

    4
    4 9 3
    brysj,
    hhrhl.
    yqqlm,
    gsycl.
    4 9 2
    brysj,
    hhrhl.
    yqqlm,
    gsycl.
    1 1005 6
    poet
    1 1004 6
    poet
    

#### 输出样例：

    108
    brysj,
    hhrhl.
    yqqlm,
    gsycl.
    --------------------
    32
    brysj, hhrhl.
    yqqlm, gsycl.
    --------------------
    Too hard to arrange
    --------------------
    1000000000000000000
    poet
    --------------------
    

#### 样例解释

前两组输入数据中每行的实际长度均为 66，后两组输入数据每行的实际长度均为 44。

一个排版方案中每行相邻两个句子之间的空格也算在这行的长度中（可参见样例中第二组数据）。

每行末尾没有空格。

难度：困难

时/空限制：1s / 64MB

总通过数：1176

总尝试数：3465

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3855&show_algorithm_tags=0)[NOI2009](https://www.acwing.com/problem/search/1/?search_content=NOI2009&source_file_id=3855&show_algorithm_tags=0)

算法标签

[DP](https://www.acwing.com/problem/search/1/?search_content=DP&source_file_id=3855&show_algorithm_tags=1)[四边形不等式](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%9B%E8%BE%B9%E5%BD%A2%E4%B8%8D%E7%AD%89%E5%BC%8F&source_file_id=3855&show_algorithm_tags=1)