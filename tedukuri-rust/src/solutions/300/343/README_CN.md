343\. 排序





 




  

给定n个变量和m个不等式。其中n小于等于26，变量分别用前n的大写英文字母表示。

不等式之间具有传递性，即若 A\>BA\>B 且 B\>CB\>C，则 A\>CA\>C。

请从前往后遍历每对关系，每次遍历时判断：

*   如果能够确定全部关系且无矛盾，则结束循环，输出确定的次序；
*   如果发生矛盾，则结束循环，输出有矛盾；
*   如果循环结束时没有发生上述两种情况，则输出无定解。

#### 输入格式

输入包含多组测试数据。

每组测试数据，第一行包含两个整数n和m。

接下来m行，每行包含一个不等式，不等式全部为小于关系。

当输入一行 `0 0` 时，表示输入终止。

#### 输出格式

每组数据输出一个占一行的结果。

结果可能为下列三种之一：

1.  如果可以确定两两之间的关系，则输出 `"Sorted sequence determined after t relations:yy...y."`,其中`'t'`指迭代次数，`'yyy...y'`是指升序排列的所有变量。
2.  如果有矛盾，则输出： `"Inconsistency found after t relations."`，其中`'t'`指迭代次数。
3.  如果没有矛盾，且不能确定两两之间的关系，则输出 `"Sorted sequence cannot be determined."`。

#### 数据范围

2≤n≤26，变量只可能为大写字母 A∼ZA∼Z。

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

例题/0x60 图论/0x61 最短路/Sorting It All Out

算法标签

* 图论 
* 传递闭包 
