270\. 可持久化并查集加强版

*    [题目](https://www.acwing.com/problem/content/description/272/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/272/1/)
*    [题解](https://www.acwing.com/problem/content/solution/272/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/272/)

  

有 nn 个集合，mm 个操作，操作分为三种：

*   `1 a b` ——合并 a,ba,b 所在集合;
*   `2 k` ——回到输入的第 kk 次操作之后的状态;
*   `3 a b` ——询问 a,ba,b 是否属于同一集合，是则输出 11 否则输出 00。

#### 输入格式

第一行为 n，mn，m。

接下来 mm 行描述了每个操作，按照题目描述中所述的格式。

每个操作强制在线，需要对输入的 a,b,ka,b,k 进行运算得到真实的 a,b,ka,b,k 后再执行操作，运算方法为 x\=x xor lastansx\=x xor lastans，lastanslastans 表示上一个询问的答案，其初始值为 00。

#### 输出格式

对于每个询问操作，输出一个结果，每个结果占一行。

#### 数据范围

0<n,m≤2×1050<n,m≤2×105

#### 输入样例：

    5 6
    1 1 2
    3 1 2
    2 1
    3 0 3
    2 1
    3 1 2
    

#### 输出样例：

    1
    0
    1
    

难度：困难

时/空限制：1s / 128MB

总通过数：487

总尝试数：1414

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3821&show_algorithm_tags=0)

算法标签

[可持久化线段树](https://www.acwing.com/problem/search/1/?search_content=%E5%8F%AF%E6%8C%81%E4%B9%85%E5%8C%96%E7%BA%BF%E6%AE%B5%E6%A0%91&source_file_id=3821&show_algorithm_tags=1)[并查集按秩合并](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%B6%E6%9F%A5%E9%9B%86%E6%8C%89%E7%A7%A9%E5%90%88%E5%B9%B6&source_file_id=3821&show_algorithm_tags=1)