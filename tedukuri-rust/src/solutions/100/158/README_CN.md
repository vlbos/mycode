158\. 项链

*    [题目](https://www.acwing.com/problem/content/description/160/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/160/1/)
*    [题解](https://www.acwing.com/problem/content/solution/160/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/160/)

  

有一天，达达捡了一条价值连城的宝石项链，但是，一个严重的问题是，他并不知道项链的主人是谁！

在得知此事后，很多人向达达发来了很多邮件，都说项链是自己的，要求他归还（显然其中最多只有一个人说了真话）。

达达要求每个人都写了一段关于自己项链的描述： 项链上的宝石用数字 00 至 99 来标示。

一个对于项链的表示就是从项链的某个宝石开始，顺指针绕一圈，沿途记下经过的宝石，比如项链： 0−1−2−30−1−2−3，它的可能的四种表示是 0123、1230、2301、30120123、1230、2301、3012。

达达现在心急如焚，于是他找到了你，希望你能够编写一个程序，判断两个给定的描述是否代表同一个项链（注意，项链是不会翻转的）。

也就是说给定两个项链的表示，判断他们是否可能是一条项链。

#### 输入格式

输入文件只有两行，每行一个由字符 00 至 99 构成的字符串，描述一个项链的表示（保证项链的长度是相等的）。

#### 输出格式

如果两个对项链的描述不可能代表同一个项链，那么输出 `No`，否则的话，第一行输出一个 `Yes`，第二行输出该项链的字典序最小的表示。

#### 数据范围

设项链的长度为 LL，1≤L≤10000001≤L≤1000000

#### 输入样例：

    2234342423
    2423223434
    

#### 输出样例：

    Yes
    2234342423
    

难度：简单

时/空限制：1s / 128MB

总通过数：2923

总尝试数：4753

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3709&show_algorithm_tags=0)

算法标签

[字符串](https://www.acwing.com/problem/search/1/?search_content=%E5%AD%97%E7%AC%A6%E4%B8%B2&source_file_id=3709&show_algorithm_tags=1)[最小表示法](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E5%B0%8F%E8%A1%A8%E7%A4%BA%E6%B3%95&source_file_id=3709&show_algorithm_tags=1)