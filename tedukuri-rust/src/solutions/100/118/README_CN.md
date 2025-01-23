118\. 分形

*    [题目](https://www.acwing.com/problem/content/description/120/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/120/1/)
*    [题解](https://www.acwing.com/problem/content/solution/120/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/120/)

  

分形，具有以非整数维形式充填空间的形态特征。

通常被定义为“一个粗糙或零碎的几何形状，可以分成数个部分，且每一部分都（至少近似地）是整体缩小后的形状”，即具有自相似的性质。

现在，定义“盒子分形”如下：

一级盒子分形：

       X
    

二级盒子分形：

       X X
        X
       X X
    

如果用 B(n−1)B(n−1) 代表第 n−1n−1 级盒子分形，那么第 nn 级盒子分形即为：

      B(n - 1)        B(n - 1)
    
              B(n - 1)
    
      B(n - 1)        B(n - 1)
    

你的任务是绘制一个 nn 级的盒子分形。

#### 输入格式

输入包含几个测试用例。

输入的每一行包含一个不大于 77 的正整数 nn，代表要输出的盒子分形的等级。

输入的最后一行为 −1−1，代表输入结束。

#### 输出格式

对于每个测试用例，使用 `X` 符号输出对应等级的盒子分形。

请注意 `X` 是一个大写字母。

每个测试用例后输出一个独立一行的短划线。

#### 输入样例：

    1
    2
    3
    4
    -1
    

#### 输出样例

    X
    -
    X X
     X
    X X
    -
    X X   X X
     X     X
    X X   X X
       X X
        X
       X X
    X X   X X
     X     X
    X X   X X
    -
    X X   X X         X X   X X
     X     X           X     X
    X X   X X         X X   X X
       X X               X X
        X                 X
       X X               X X
    X X   X X         X X   X X
     X     X           X     X
    X X   X X         X X   X X
             X X   X X
              X     X
             X X   X X
                X X
                 X
                X X
             X X   X X
              X     X
             X X   X X
    X X   X X         X X   X X
     X     X           X     X
    X X   X X         X X   X X
       X X               X X
        X                 X
       X X               X X
    X X   X X         X X   X X
     X     X           X     X
    X X   X X         X X   X X
    -
    

难度：简单

时/空限制：1s / 64MB

总通过数：3336

总尝试数：6041

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3669&show_algorithm_tags=0)

算法标签

[递归](https://www.acwing.com/problem/search/1/?search_content=%E9%80%92%E5%BD%92&source_file_id=3669&show_algorithm_tags=1)[分形](https://www.acwing.com/problem/search/1/?search_content=%E5%88%86%E5%BD%A2&source_file_id=3669&show_algorithm_tags=1)[坐标变换](https://www.acwing.com/problem/search/1/?search_content=%E5%9D%90%E6%A0%87%E5%8F%98%E6%8D%A2&source_file_id=3669&show_algorithm_tags=1)