155\. 内存分配

*    [题目](https://www.acwing.com/problem/content/description/157/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/157/1/)
*    [题解](https://www.acwing.com/problem/content/solution/157/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/157/)

  

内存是计算机重要的资源之一，程序运行的过程中必须对内存进行分配。

经典的内存分配过程是这样进行的：

1、 内存以内存单元为基本单位，每个内存单元用一个固定的整数作为标识，称为地址。地址从 00 开始连续排列，地址相邻的内存单元被认为是逻辑上连续的。我们把从地址 ii 开始的 ss 个连续的内存单元称为首地址为 ii 长度为 ss 的地址片。

2、 运行过程中有若干进程需要占用内存，对于每个进程有一个申请时刻 TT，需要内存单元数 MM 及运行时间 PP。在运行时间 PP 内（即 TT 时刻开始，T+PT+P 时刻结束），这 MM 个被占用的内存单元不能再被其他进程使用。

3、假设在T时刻有一个进程申请 MM 个单元，且运行时间为 PP，则：

1.  若 TT 时刻内存中存在长度为 MM 的空闲地址片，则系统将这 MM 个空闲单元分配给该进程。若存在多个长度为 MM 个空闲地址片，则系统将首地址最小的那个空闲地址片分配给该进程。
    
2.  如果 TT 时刻不存在长度为 MM 的空闲地址片，则该进程被放入一个等待队列。对于处于等待队列队头的进程，只要在任一时刻，存在长度为 MM 的空闲地址片，系统马上将该进程取出队列，并为它分配内存单元。注意，在进行内存分配处理过程中，处于等待队列队头的进程的处理优先级最高，队列中的其它进程不能先于队头进程被处理。
    

现在给出一系列描述进程的数据，请编写一程序模拟系统分配内存的过程。

#### 输入格式

第一行是一个数 NN，表示总内存单元数（即地址范围从 00 到 N−1N−1）。

从第二行开始每行描述一个进程的三个整数 T、M、P（M≤N）T、M、P（M≤N）。

最后一行用三个 00 表示结束。

数据已按 TT 从小到大排序。

输入文件最多 1000010000 行，且所有数据都小于 109109。

输入文件中同一行相邻两项之间用一个或多个空格隔开。

#### 输出格式

输出包括 22 行。

第一行是全部进程都运行完毕的时刻。

第二行是被放入过等待队列的进程总数。

#### 输入样例：

    10
    1 3 10
    2 4 3
    3 4 4
    4 1 4
    5 3 4
    0 0 0
    

#### 输出样例：

    12
    2
    

#### 提示

![内存.png](https://cdn.acwing.com/media/article/image/2019/01/15/19_42f2e01618-%E5%86%85%E5%AD%98.png)

难度：困难

时/空限制：1s / 64MB

总通过数：1056

总尝试数：2475

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3706&show_algorithm_tags=0)[NOI1999](https://www.acwing.com/problem/search/1/?search_content=NOI1999&source_file_id=3706&show_algorithm_tags=0)

算法标签

[链表](https://www.acwing.com/problem/search/1/?search_content=%E9%93%BE%E8%A1%A8&source_file_id=3706&show_algorithm_tags=1)[二叉堆](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%8F%89%E5%A0%86&source_file_id=3706&show_algorithm_tags=1)