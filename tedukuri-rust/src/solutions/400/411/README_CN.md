411\. 国王的任务

*    [题目](https://www.acwing.com/problem/content/description/413/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/413/1/)
*    [题解](https://www.acwing.com/problem/content/solution/413/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/413/)

  

曾经有一个国王，他有 NN 个儿子。

王国中有着 NN 个漂亮的姑娘，每个王子也都有自己喜欢的对象。

每个王子喜欢的对象可能不止一个。

因为王子们都到了结婚的年纪，所以国王想让王子们娶了这 NN 个姑娘，当然每个姑娘只能嫁给一名王子。

国王请巫师为他做一个统计，他想看看儿子们都有哪些喜欢的姑娘。

就这样，巫师制作了一个清单，上面具体列出了每一个王子喜欢哪些姑娘，并给出了一套初步的配对方案。

国王看了看巫师给他列出的清单，说道：“你总结的不错，但是我并不完全满意。我希望你列出每个王子可以婚配的女子清单，可以满足每个王子对应的清单上都是他喜欢的姑娘，并且任何一个王子从自己的清单上任意选择一名姑娘作为自己的结婚对象之后，剩下的王子仍然能够从自己的清单中选择的到自己喜欢的对象，使得所有的王子都能完成和自己喜欢的姑娘配对。”

请你帮助巫师列出国王满意的清单。

#### 输入格式

第一行包含整数 NN。

接下来 NN 行，每行包含多个整数，描述了一个王子喜欢的姑娘的清单，每行的第一个整数 KK，表示王子喜欢的姑娘的数量，接下来的 KK 个整数，为这 KK 个姑娘的编号。

最后一行，包含 NN 个整数，表示初步的配对方案，第 ii 个整数表示与第 ii 个王子进行配对的姑娘编号。

#### 输出格式

输出共 NN 行。

每行包含多个整数，描述一个王子可以婚配的女子清单，第一个整数 LL，表示王子可以婚配的姑娘的数量，接下来 LL 个整数，为这 LL 个姑娘的编号（请按升序排列）。

#### 数据范围

1≤N≤20001≤N≤2000,  
所有 KK 加起来的和不超过 200000200000。

#### 输入样例：

    4
    2 1 2
    2 1 2
    2 2 3
    2 3 4
    1 2 3 4
    

#### 输出样例：

    2 1 2
    2 1 2
    1 3
    1 4
    

难度：中等

时/空限制：1s / 64MB

总通过数：234

总尝试数：438

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3962&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3962&show_algorithm_tags=1)[二分图最大匹配的可行边](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86%E5%9B%BE%E6%9C%80%E5%A4%A7%E5%8C%B9%E9%85%8D%E7%9A%84%E5%8F%AF%E8%A1%8C%E8%BE%B9&source_file_id=3962&show_algorithm_tags=1)