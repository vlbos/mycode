410\. 排版幻灯片

*    [题目](https://www.acwing.com/problem/content/description/412/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/412/1/)
*    [题解](https://www.acwing.com/problem/content/solution/412/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/412/)

  

一些透明的幻灯片重叠在了一起，每个幻灯片上都标着它的顺序编号。

由于它们是透明的，所以无法看清每个数字究竟标在哪个幻灯片上。

![1486_1.jpg](https://cdn.acwing.com/media/article/image/2019/02/15/19_0066e5fa31-1486_1.jpg)

虽然我们无法直接看出每个幻灯片的具体编号，但是却可以根据推理判断出每个幻灯片的编号。

例如上图中，幻灯片被标记为 A,B,C,DA,B,C,D，其中 AA 的编号为 44，BB 的编号为 11，CC 的编号为 22，DD 的编号为 33。

现在你需要编写一个程序，实现对幻灯片编号的判断。

#### 输入格式

输入包含多组测试数据。

每组测试数据第一行包含整数 nn，表示共有 nn 张幻灯片。

接下来 nn 行，每行包含四个整数 xmin,xmax,ymin,ymaxxmin,xmax,ymin,ymax，每个都是幻灯片的边界坐标，幻灯片按输入顺序标记为 A,B,C…A,B,C…

再接下来 nn 行，每行包含两个整数，第 ii 行的整数表示编号 ii 的横纵坐标。

当输入一行为 00 时，表示输入终止。

#### 输出格式

对于每组数据，第一行输出 `Heap x`，其中 xx 为数据编号，从 11 开始。

第二行输出每个幻灯片以及其对应的编号，形如 (A,4)(A,4)，不同幻灯片输出之间用空格隔开。

如果无法判断，则输出 `none`。

每组数据输出结束后，输出一个空行。

#### 数据范围

1≤n≤261≤n≤26，坐标可以取负值。

#### 输入样例:

    4
    6 22 10 20
    4 18 6 16
    8 20 2 18
    10 24 4 8
    9 15
    19 17
    11 7
    21 11
    2
    0 2 0 2
    0 2 0 2
    1 1
    1 1
    0
    

#### 输出样例：

    Heap 1
    (A,4) (B,1) (C,2) (D,3)
    
    Heap 2
    none
    
    

难度：中等

时/空限制：1s / 64MB

总通过数：211

总尝试数：551

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3961&show_algorithm_tags=0)[《信息学奥赛一本通》语言及算法基础篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E8%AF%AD%E8%A8%80%E5%8F%8A%E7%AE%97%E6%B3%95%E5%9F%BA%E7%A1%80%E7%AF%87&source_file_id=3961&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3961&show_algorithm_tags=1)[二分图最大匹配的必须边](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86%E5%9B%BE%E6%9C%80%E5%A4%A7%E5%8C%B9%E9%85%8D%E7%9A%84%E5%BF%85%E9%A1%BB%E8%BE%B9&source_file_id=3961&show_algorithm_tags=1)