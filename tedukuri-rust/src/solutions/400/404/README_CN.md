404\. 婚礼

*    [题目](https://www.acwing.com/problem/content/description/406/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/406/1/)
*    [题解](https://www.acwing.com/problem/content/solution/406/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/406/)

  

很多对（不超过 3030）夫妇将参加婚礼盛宴，他们将坐在长桌的两侧。

新娘和新郎坐在一端，彼此相对，新娘戴着精致的头饰，使她看不到与她在同一侧的人。

安排丈夫和妻子坐在桌子的同一侧是不幸的。

此外，有几对人进行通奸关系（不同性别和同性关系都是可能的），并且新娘看到这样的一对成员是不幸的。

你的工作是安排人们的位置，以避免不幸。

#### 输入格式

输入包含多组测试用例。

每组测试用例，第一行包含两个整数 nn 和 mm，表示共有 nn 对夫妇，mm 对奸夫淫妇。

接下来 mm 行，每行揭露一个通奸关系。

形如 `4h 2w` 表示第 44 对夫妇中的丈夫和第 22 对夫妇中的妻子通奸，`3h 1h` 表示第 33 对夫妇中的丈夫和第 11 对夫妇中的丈夫通奸。

每对夫妇被编号为 `0,1,...,n-1`，其中新郎新娘的编号为 00。

当输入一行为 `0 0` 时，表示输入终止。

#### 输出格式

每组测试用例输出一个结果，每个结果占一行。

结果包含同新娘坐在一侧的人员列表。

如果有多种方案，随便输出一种即可。

输出结果时，请按照编号从小到大（即 1∼n−11∼n−1）的顺序，输出人员。

如果没有方案，则输出 `bad luck`。

#### 输入样例：

    10 6
    3h 7h
    5w 3w
    7h 6w
    8w 3w
    7h 3w
    2w 5h
    0 0
    

#### 输出样例：

    1h 2h 3w 4h 5h 6h 7h 8h 9h
    

难度：困难

时/空限制：1s / 64MB

总通过数：344

总尝试数：1055

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3955&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3955&show_algorithm_tags=0)[POJ3648](https://www.acwing.com/problem/search/1/?search_content=POJ3648&source_file_id=3955&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3955&show_algorithm_tags=1)[2-SAT](https://www.acwing.com/problem/search/1/?search_content=2-SAT&source_file_id=3955&show_algorithm_tags=1)[输出方案](https://www.acwing.com/problem/search/1/?search_content=%E8%BE%93%E5%87%BA%E6%96%B9%E6%A1%88&source_file_id=3955&show_algorithm_tags=1)