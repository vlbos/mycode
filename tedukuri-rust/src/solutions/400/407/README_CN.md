407\. 稳定的牛分配

*    [题目](https://www.acwing.com/problem/content/description/409/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/409/1/)
*    [题解](https://www.acwing.com/problem/content/solution/409/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/409/)

  

农夫约翰的 NN 头奶牛住在 BB 个谷仓里，每个谷仓的容量有限，有的牛很喜欢现在的住所，而有的则对现在的住所非常不满意。

农夫约翰打算重新安排奶牛的住所，使得它们的幸福感尽可能的接近，哪怕这会使所有牛都对安排产生不满。

每头奶牛都给了约翰一个住所幸福感列表，被安排的谷仓在列表中的排名将直接影响牛的幸福感高低。

你需要给定一个合理的安排，使得每个谷仓安排的牛的数量不能超过容量上限，并且幸福感最高的牛和幸福感最低的牛的幸福感差距尽量的小。

换句话说，对住所最满意的牛被安排的住所在其列表中的排名和对住所最不满意的牛被安排的住所在其列表中的排名之间相差最小。

#### 输入格式

第 11 行包含两个整数 NN 和 BB。

第 2..N+12..N+1 行，每行包含 BB 个整数，第 i+1i+1 行描述了第 ii 头牛的住所幸福感列表，越靠前的住所牛越满意。

第 N+2N+2 行，包含 BB 个整数，第 ii 个整数表示第 ii 间谷仓的容量。

#### 输出格式

输出一个整数，表示牛被安排的住所在列表上的排名的范围是多少。

例如，一共 44 头牛，33 头被安排在满意度排名 11 的谷仓，11 头被安排在满意度排名 22 的谷仓，则范围是 \[1,2\]\[1,2\]，输出 22。

#### 数据范围

1≤N≤10001≤N≤1000,  
1≤B≤201≤B≤20

#### 输入样例：

    6 4
    1 2 3 4
    2 3 1 4
    4 2 3 1
    3 1 2 4
    1 3 4 2
    1 4 2 3
    2 1 3 2
    

#### 输出样例：

    2
    

难度：中等

时/空限制：1s / 64MB

总通过数：335

总尝试数：947

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3958&show_algorithm_tags=0)[POJ3189](https://www.acwing.com/problem/search/1/?search_content=POJ3189&source_file_id=3958&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3958&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3958&show_algorithm_tags=1)[二分图多重匹配](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86%E5%9B%BE%E5%A4%9A%E9%87%8D%E5%8C%B9%E9%85%8D&source_file_id=3958&show_algorithm_tags=1)