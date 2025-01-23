168\. 生日蛋糕

*    [题目](https://www.acwing.com/problem/content/description/170/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/170/1/)
*    [题解](https://www.acwing.com/problem/content/solution/170/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/170/)

  

77 月 1717 日是 Mr.W 的生日，ACM-THU 为此要制作一个体积为 NπNπ 的 MM 层生日蛋糕，每层都是一个圆柱体。

设从下往上数第 ii 层蛋糕是半径为 RiRi，高度为 HiHi 的圆柱。

当 i<Mi<M 时，要求 Ri\>Ri+1Ri\>Ri+1 且 Hi\>Hi+1Hi\>Hi+1。

由于要在蛋糕上抹奶油，为尽可能节约经费，我们希望蛋糕外表面（最下一层的下底面除外）的面积 QQ 最小。

令 Q\=SπQ\=Sπ ，请编程对给出的 NN 和 MM，找出蛋糕的制作方案（适当的 RiRi 和 HiHi 的值），使 SS 最小。

除 QQ 外，以上所有数据皆为正整数。

#### 输入格式

输入包含两行，第一行为整数 NN，表示待制作的蛋糕的体积为 NπNπ。

第二行为整数 MM，表示蛋糕的层数为 MM。

#### 输出格式

输出仅一行，是一个正整数 SS（若无解则 S\=0S\=0）。

#### 数据范围

1≤N≤100001≤N≤10000,  
1≤M≤201≤M≤20

#### 输入样例：

    100
    2
    

#### 输出样例：

    68
    

难度：中等

时/空限制：1s / 10MB

总通过数：9186

总尝试数：16862

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3719&show_algorithm_tags=0)[NOI1999](https://www.acwing.com/problem/search/1/?search_content=NOI1999&source_file_id=3719&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3719&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3719&show_algorithm_tags=1)[剪枝](https://www.acwing.com/problem/search/1/?search_content=%E5%89%AA%E6%9E%9D&source_file_id=3719&show_algorithm_tags=1)