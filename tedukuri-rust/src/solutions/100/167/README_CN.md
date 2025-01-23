167\. 木棒

*    [题目](https://www.acwing.com/problem/content/description/169/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/169/1/)
*    [题解](https://www.acwing.com/problem/content/solution/169/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/169/)

  

乔治拿来一组等长的木棒，将它们随机地砍断，使得每一节木棍的长度都不超过 5050 个长度单位。

然后他又想把这些木棍恢复到为裁截前的状态，但忘记了初始时有多少木棒以及木棒的初始长度。

请你设计一个程序，帮助乔治计算木棒的可能最小长度。

每一节木棍的长度都用大于零的整数表示。

#### 输入格式

输入包含多组数据，每组数据包括两行。

第一行是一个不超过 6464 的整数，表示砍断之后共有多少节木棍。

第二行是截断以后，所得到的各节木棍的长度。

在最后一组数据之后，是一个零。

#### 输出格式

为每组数据，分别输出原始木棒的可能最小长度，每组数据占一行。

#### 数据范围

数据保证每一节木棍的长度均不大于 5050。

#### 输入样例：

    9
    5 2 1 5 2 1 5 2 1
    4
    1 2 3 4
    0
    

#### 输出样例：

    6
    5
    

难度：中等

时/空限制：1s / 64MB

总通过数：20200

总尝试数：51590

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3718&show_algorithm_tags=0)[UVA307](https://www.acwing.com/problem/search/1/?search_content=UVA307&source_file_id=3718&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3718&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3718&show_algorithm_tags=1)[DFS](https://www.acwing.com/problem/search/1/?search_content=DFS&source_file_id=3718&show_algorithm_tags=1)[剪枝](https://www.acwing.com/problem/search/1/?search_content=%E5%89%AA%E6%9E%9D&source_file_id=3718&show_algorithm_tags=1)