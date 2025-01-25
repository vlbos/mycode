224\. 计算器

*    [题目](https://www.acwing.com/problem/content/description/226/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/226/1/)
*    [题解](https://www.acwing.com/problem/content/solution/226/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/226/)

  

你被要求设计一个计算器完成以下三项任务：

1.  给定 Y,Z,PY,Z,P，计算 YZ(modP)YZ(modP) 的值；
2.  给定 Y,Z,PY,Z,P，计算满足 xY≡Z(modP)xY≡Z(modP) 的最小非负整数 xx；
3.  给定 Y,Z,PY,Z,P，计算满足 Yx≡Z(modP)Yx≡Z(modP) 的最小非负整数 xx。

#### 输入格式

输入包含多组数据。

第一行包含两个正整数 T,KT,K 分别表示数据组数和询问类型（对于一个测试点内的所有数据，询问类型相同）。

以下 TT 行每行包含三个正整数 Y,Z,PY,Z,P，描述一个询问。

#### 输出格式

对于每个询问，输出一行答案。

对于询问类型 22 和 33，如果不存在满足条件的数，则输出 `Orz, I cannot find x!`，注意逗号与 `I` 之间有一个空格。

#### 数据范围

1≤Y,Z,P≤1091≤Y,Z,P≤109,其中 PP 为质数。  
1≤T≤101≤T≤10

#### 输入样例：

    3 1
    2 1 3
    2 2 3
    2 3 3
    

#### 输出样例：

    2
    1
    2
    

难度：困难

时/空限制：1s / 64MB

总通过数：741

总尝试数：2030

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3775&show_algorithm_tags=0)[SDOI2011](https://www.acwing.com/problem/search/1/?search_content=SDOI2011&source_file_id=3775&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3775&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3775&show_algorithm_tags=1)[同余方程](https://www.acwing.com/problem/search/1/?search_content=%E5%90%8C%E4%BD%99%E6%96%B9%E7%A8%8B&source_file_id=3775&show_algorithm_tags=1)