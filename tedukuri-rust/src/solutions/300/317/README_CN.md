317\. 陨石的秘密

*    [题目](https://www.acwing.com/problem/content/description/319/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/319/1/)
*    [题解](https://www.acwing.com/problem/content/solution/319/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/319/)

  

公元 1138011380 年，一颗巨大的陨石坠落在南极。

于是，灾难降临了，地球上出现了一系列反常的现象。

当人们焦急万分的时候，一支中国科学家组成的南极考察队赶到了出事地点。

经过一番侦察，科学家们发现陨石上刻有若干行密文，每一行都包含 55 个整数：

    1 1 1 1 6 
    0 0 6 3 57 
    8 0 11 3 2845 
    

著名的科学家 SSSS 发现，这些密文实际上是一种复杂运算的结果。

为了便于大家理解这种运算，他定义了一种 SSSS 表达式：

1.  SSSS 表达式是仅由 `{`，`}`，`[`，`]`，`(`，`)` 组成的字符串。
2.  一个空串是 SSSS 表达式。
3.  如果 AA 是 SSSS 表达式，且 AA 中不含字符 `{`，`}`，`[`，`]`，则 (A)(A) 是 SSSS 表达式。
4.  如果 AA 是 SSSS 表达式，且 AA 中不含字符 `{`，`}`，则 \[A\]\[A\] 是 SSSS 表达式。
5.  如果 AA 是 SSSS 表达式，则 {A}{A} 是 SSSS 表达式。
6.  如果 AA 和 BB 都是 SSSS 表达式，则 ABAB 也是 SSSS 表达式。

例如

    ()(())[] 
    {()[()]} 
    {{[[(())]]}} 
    

都是 SSSS 表达式。

而

    ()([])() 
    [() 
    

不是 SSSS 表达式。

一个 SSSS 表达式 EE 的深度 D(E)D(E) 定义如下：

![1187_1.jpg](https://cdn.acwing.com/media/article/image/2019/02/05/19_410d738a28-1187_1.jpg)

例如 `(){()}[]` 的深度为 22。

密文中的复杂运算是这样进行的：

设密文中每行前 44 个数依次为 L1，L2，L3，DL1，L2，L3，D，求出所有深度为 DD，含有 L1L1 对 `{}`，L2L2 对 `[]`，L3L3 对 `()` 的 SSSS 串的个数，并用这个数对当前的年份 1138011380 求余数，这个余数就是密文中每行的第 55 个数，我们称之为神秘数。

密文中某些行的第五个数已经模糊不清，而这些数字正是揭开陨石秘密的钥匙。

现在科学家们聘请你来计算这个神秘数。

#### 输入格式

共一行，44 个整数 L1，L2，L3，DL1，L2，L3，D。

#### 输出格式

共一行，包含一个整数，即神秘数。

#### 数据范围

0≤L1,L2,L3≤100≤L1,L2,L3≤10,  
0≤D≤300≤D≤30

#### 输入样例：

    1 1 1 2
    

#### 输出样例：

    8
    

难度：中等

时/空限制：1s / 10MB

总通过数：544

总尝试数：1137

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3868&show_algorithm_tags=0)[NOI2001](https://www.acwing.com/problem/search/1/?search_content=NOI2001&source_file_id=3868&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3868&show_algorithm_tags=1)[线性DP](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%80%A7DP&source_file_id=3868&show_algorithm_tags=1)