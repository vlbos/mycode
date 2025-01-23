179\. 八数码

*    [题目](https://www.acwing.com/problem/content/description/181/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/181/1/)
*    [题解](https://www.acwing.com/problem/content/solution/181/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/181/)

  

在一个 3×33×3 的网格中，1∼81∼8 这 88 个数字和一个 `x` 恰好不重不漏地分布在这 3×33×3 的网格中。

例如：

    1 2 3
    x 4 6
    7 5 8
    

在游戏过程中，可以把 `x` 与其上、下、左、右四个方向之一的数字交换（如果存在）。

我们的目的是通过交换，使得网格变为如下排列（称为正确排列）：

    1 2 3
    4 5 6
    7 8 x
    

例如，示例中图形就可以通过让 `x` 先后与右、下、右三个方向的数字交换成功得到正确排列。

交换过程如下：

    1 2 3   1 2 3   1 2 3   1 2 3
    x 4 6   4 x 6   4 5 6   4 5 6
    7 5 8   7 5 8   7 x 8   7 8 x
    

把 `x` 与上下左右方向数字交换的行动记录为 `u`、`d`、`l`、`r`。

现在，给你一个初始网格，请你通过最少的移动次数，得到正确排列。

#### 输入格式

输入占一行，将 3×33×3 的初始网格描绘出来。

例如，如果初始网格如下所示：

    1 2 3 
    x 4 6 
    7 5 8 
    

则输入为：`1 2 3 x 4 6 7 5 8`

#### 输出格式

输出占一行，包含一个字符串，表示得到正确排列的完整行动记录。

如果答案不唯一，输出任意一种合法方案即可。

如果不存在解决方案，则输出 `unsolvable`。

#### 输入样例：

    2 3 4 1 5 x 7 6 8
    

#### 输出样例

    ullddrurdllurdruldr
    

难度：中等

时/空限制：1s / 64MB

总通过数：14965

总尝试数：29526

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3730&show_algorithm_tags=0)[HDU1043](https://www.acwing.com/problem/search/1/?search_content=HDU1043&source_file_id=3730&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3730&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3730&show_algorithm_tags=1)[A\*](https://www.acwing.com/problem/search/1/?search_content=A*&source_file_id=3730&show_algorithm_tags=1)[哈希](https://www.acwing.com/problem/search/1/?search_content=%E5%93%88%E5%B8%8C&source_file_id=3730&show_algorithm_tags=1)[康托展开](https://www.acwing.com/problem/search/1/?search_content=%E5%BA%B7%E6%89%98%E5%B1%95%E5%BC%80&source_file_id=3730&show_algorithm_tags=1)