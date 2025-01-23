182\. 破坏正方形

*    [题目](https://www.acwing.com/problem/content/description/184/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/184/1/)
*    [题解](https://www.acwing.com/problem/content/solution/184/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/184/)

  

下图左侧显示了一个用 2424 根火柴棍构成的完整 3×33×3 网格。

所有火柴的长度都是 11。

您可以在网格中找到许多不同大小的正方形。

在左图所示的网格中，有 99 个边长为 11 的正方形，44 个边长为 22 的正方形和 11 个边长为 33 的正方形。

组成完整网格的每一根火柴都有唯一编号，该编号从上到下，从左到右，从 11 开始按顺序分配。

如果你将一些火柴棍从完整网格中取出，形成一个不完整的网格，则一部分正方形将被破坏。

右图为移除编号 12,1712,17 和 2323 的三个火柴棍后的不完整的 3×33×3 网格。

这次移除破坏了 55 个边长为 11 的正方形，33 个边长为 22 的正方形和 11 个边长为 33 的正方形。

此时，网格不具有边长为 33 的正方形，但仍然具有 44 个边长为 11 的正方形和 11 个边长为 22 的正方形。

![火柴图.jpg](https://cdn.acwing.com/media/article/image/2019/01/16/19_2af90edc19-%E7%81%AB%E6%9F%B4%E5%9B%BE.jpg)

现在给定一个（完整或不完整）的 n×nn×n（nn 不大于 55）网格，求至少再去掉多少根火柴棒，可以使得网格内不再含有任何尺寸的正方形。

#### 输入格式

输入包含 TT 组测试用例。

测试用例的数量 TT 在输入文件的第一行中给出。

每个测试用例由两行组成：

第一行包含一个整数 nn，表示网格的规模大小。

第二行以非负整数 kk 开头，表示所给网格相较完整的 n×nn×n 网格所缺少的火柴杆数量，后跟 kk 个整数表示所有缺少的火柴杆的具体编号。

注意，如果 kk 等于零，则表示输入网格是完整的 n×nn×n 网格。

#### 输出格式

每个测试用例输出一个结果，表示破坏所有正方形，所需的去掉火柴棒的最小数量。

每个结果占一行。

#### 输入样例：

    2
    2
    0
    3
    3 12 17 23
    

#### 输出样例：

    3
    3
    

难度：困难

时/空限制：1s / 10MB

总通过数：1072

总尝试数：2238

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3733&show_algorithm_tags=0)[POJ1084](https://www.acwing.com/problem/search/1/?search_content=POJ1084&source_file_id=3733&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3733&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3733&show_algorithm_tags=1)[IDA\*](https://www.acwing.com/problem/search/1/?search_content=IDA*&source_file_id=3733&show_algorithm_tags=1)[剪枝](https://www.acwing.com/problem/search/1/?search_content=%E5%89%AA%E6%9E%9D&source_file_id=3733&show_algorithm_tags=1)[DLX](https://www.acwing.com/problem/search/1/?search_content=DLX&source_file_id=3733&show_algorithm_tags=1)[dancing links](https://www.acwing.com/problem/search/1/?search_content=dancing%20links&source_file_id=3733&show_algorithm_tags=1)[重复覆盖](https://www.acwing.com/problem/search/1/?search_content=%E9%87%8D%E5%A4%8D%E8%A6%86%E7%9B%96&source_file_id=3733&show_algorithm_tags=1)