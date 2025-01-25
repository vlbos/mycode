223\. 阿九大战朱最学

*    [题目](https://www.acwing.com/problem/content/description/225/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/225/1/)
*    [题解](https://www.acwing.com/problem/content/solution/225/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/225/)

  

自从朱最学搞定了 QQ 农场以后，就开始捉摸去 QQ 牧场干些事业，不仅在自己的牧场养牛，还到阿九的牧场放牛！

阿九很生气，有一次朱最学想知道阿九牧场奶牛的数量，于是阿九想狠狠耍朱最学一把。

举个例子，假如有 1616 头奶牛，如果建了 33 个牛棚，剩下 11 头牛就没有地方安家了。

如果建造了 55 个牛棚，但是仍然有 11 头牛没有地方去，然后如果建造了 77 个牛棚，还有 22 头没有地方去。

你作为阿九的私人秘书理所当然要将准确的奶牛数报给阿九，你该怎么办？

#### 输入格式

第一行包含一个整数 nn 表示建立牛棚的次数。

接下来 nn 行，每行两个整数 ai,biai,bi，表示建立了 aiai 个牛棚，有 bibi 头牛没有去处。

你可以假定不同 aiai 之间互质。

#### 输出格式

输出包含一个正整数，即为阿九至少养奶牛的数目。

#### 数据范围

1≤n≤101≤n≤10,  
1≤ai,bi≤12000001≤ai,bi≤1200000，  
所有 aiai 的乘积不超过 10121012。

#### 输入样例：

    3
    3 1
    5 1
    7 2
    

#### 输出样例：

    16
    

难度：简单

时/空限制：1s / 64MB

总通过数：941

总尝试数：1514

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3774&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3774&show_algorithm_tags=1)[中国剩余定理](https://www.acwing.com/problem/search/1/?search_content=%E4%B8%AD%E5%9B%BD%E5%89%A9%E4%BD%99%E5%AE%9A%E7%90%86&source_file_id=3774&show_algorithm_tags=1)