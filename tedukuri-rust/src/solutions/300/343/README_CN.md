343\. 排序

*    [题目](https://www.acwing.com/problem/content/description/345/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/345/1/)
*    [题解](https://www.acwing.com/problem/content/solution/345/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/345/)

  

给定 nn 个变量和 mm 个不等式。其中 nn 小于等于 2626，变量分别用前 nn 的大写英文字母表示。

不等式之间具有传递性，即若 A\>BA\>B 且 B\>CB\>C，则 A\>CA\>C。

请从前往后遍历每对关系，每次遍历时判断：

*   如果能够确定全部关系且无矛盾，则结束循环，输出确定的次序；
*   如果发生矛盾，则结束循环，输出有矛盾；
*   如果循环结束时没有发生上述两种情况，则输出无定解。

#### 输入格式

输入包含多组测试数据。

每组测试数据，第一行包含两个整数 nn 和 mm。

接下来 mm 行，每行包含一个不等式，不等式全部为小于关系。

当输入一行 `0 0` 时，表示输入终止。

#### 输出格式

每组数据输出一个占一行的结果。

结果可能为下列三种之一：

1.  如果可以确定两两之间的关系，则输出 `"Sorted sequence determined after t relations: yyy...y."`,其中`'t'`指迭代次数，`'yyy...y'`是指升序排列的所有变量。
2.  如果有矛盾，则输出： `"Inconsistency found after t relations."`，其中`'t'`指迭代次数。
3.  如果没有矛盾，且不能确定两两之间的关系，则输出 `"Sorted sequence cannot be determined."`。

#### 数据范围

2≤n≤262≤n≤26，变量只可能为大写字母 A∼ZA∼Z。

#### 输入样例1：

    4 6
    A<B
    A<C
    B<C
    C<D
    B<D
    A<B
    3 2
    A<B
    B<A
    26 1
    A<Z
    0 0
    

#### 输出样例1：

    Sorted sequence determined after 4 relations: ABCD.
    Inconsistency found after 2 relations.
    Sorted sequence cannot be determined.
    

#### 输入样例2：

    6 6
    A<F
    B<D
    C<E
    F<D
    D<E
    E<F
    0 0
    

#### 输出样例2：

    Inconsistency found after 6 relations.
    

#### 输入样例3：

    5 5
    A<B
    B<C
    C<D
    D<E
    E<A
    0 0
    

#### 输出样例3：

    Sorted sequence determined after 4 relations: ABCDE.
    

难度：简单

时/空限制：1s / 10MB

总通过数：10753

总尝试数：23996

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3894&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3894&show_algorithm_tags=1)[传递闭包](https://www.acwing.com/problem/search/1/?search_content=%E4%BC%A0%E9%80%92%E9%97%AD%E5%8C%85&source_file_id=3894&show_algorithm_tags=1)