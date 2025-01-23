128\. 编辑器

*    [题目](https://www.acwing.com/problem/content/description/130/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/130/1/)
*    [题解](https://www.acwing.com/problem/content/solution/130/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/130/)

  

你将要实现一个功能强大的整数序列编辑器。

在开始时，序列是空的。

编辑器共有五种指令，如下：

1、`I x`，在光标处插入数值 xx。  
2、`D`，将光标前面的第一个元素删除，如果前面没有元素，则忽略此操作。  
3、`L`，将光标向左移动，跳过一个元素，如果左边没有元素，则忽略此操作。  
4、`R`，将光标向右移动，跳过一个元素，如果右边没有元素，则忽略此操作。  
5、`Q k`，假设此刻光标之前的序列为 a1,a2,…,ana1,a2,…,an，输出 max1≤i≤kSimax1≤i≤kSi，其中 Si\=a1+a2+…+aiSi\=a1+a2+…+ai。

#### 输入格式

第一行包含一个整数 QQ，表示指令的总数。

接下来 QQ 行，每行一个指令，具体指令格式如题目描述。

#### 输出格式

每一个 `Q k` 指令，输出一个整数作为结果，每个结果占一行。

#### 数据范围

1≤Q≤1061≤Q≤106,  
|x|≤103|x|≤103,  
1≤k≤n1≤k≤n

#### 输入样例：

    8
    I 2
    I -1
    I 1
    Q 3
    L
    D
    R
    Q 2
    

#### 输出样例：

    2
    3
    

#### 样例解释

下图包含了对样例的过程描述：

![C464-1004-2.jpg](https://cdn.acwing.com/media/article/image/2019/01/22/19_e0778ec41d-C464-1004-2.jpg)

难度：简单

时/空限制：1s / 64MB

总通过数：4965

总尝试数：15702

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3679&show_algorithm_tags=0)

算法标签

[栈](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%88&source_file_id=3679&show_algorithm_tags=1)