166\. 数独

*    [题目](https://www.acwing.com/problem/content/description/168/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/168/1/)
*    [题解](https://www.acwing.com/problem/content/solution/168/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/168/)

  

[数独](https://baike.baidu.com/item/%E6%95%B0%E7%8B%AC/74847?fr=aladdin) 是一种传统益智游戏，你需要把一个 9×99×9 的数独补充完整，使得数独中每行、每列、每个 3×33×3 的九宫格内数字 1∼91∼9 均恰好出现一次。

请编写一个程序填写数独。

#### 输入格式

输入包含多组测试用例。

每个测试用例占一行，包含 8181 个字符，代表数独的 8181 个格内数据（顺序总体由上到下，同行由左到右）。

每个字符都是一个数字（1−91−9）或一个 `.`（表示尚未填充）。

您可以假设输入中的每个谜题都只有一个解决方案。

文件结尾处为包含单词 `end` 的单行，表示输入结束。

#### 输出格式

每个测试用例，输出一行数据，代表填充完全后的数独。

#### 输入样例：

    4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......
    ......52..8.4......3...9...5.1...6..2..7........3.....6...1..........7.4.......3.
    end
    

#### 输出样例：

    417369825632158947958724316825437169791586432346912758289643571573291684164875293
    416837529982465371735129468571298643293746185864351297647913852359682714128574936
    

难度：中等

时/空限制：1s / 64MB

总通过数：15193

总尝试数：27760

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3717&show_algorithm_tags=0)[POJ3074](https://www.acwing.com/problem/search/1/?search_content=POJ3074&source_file_id=3717&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3717&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3717&show_algorithm_tags=1)[深度优先搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%B7%B1%E5%BA%A6%E4%BC%98%E5%85%88%E6%90%9C%E7%B4%A2&source_file_id=3717&show_algorithm_tags=1)[DFS](https://www.acwing.com/problem/search/1/?search_content=DFS&source_file_id=3717&show_algorithm_tags=1)