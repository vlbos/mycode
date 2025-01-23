169\. 数独2

*    [题目](https://www.acwing.com/problem/content/description/171/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/171/1/)
*    [题解](https://www.acwing.com/problem/content/solution/171/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/171/)

  

请你将一个 16×1616×16 的数独填写完整，使得每行、每列、每个 4×44×4 十六宫格内字母 A∼PA∼P 均恰好出现一次。

保证每个输入只有唯一解决方案。

![数独2.jpg](https://cdn.acwing.com/media/article/image/2019/01/16/19_cabce58018-%E6%95%B0%E7%8B%AC2.jpg)

#### 输入格式

输入包含多组测试用例。

每组测试用例包括 1616 行，每行一组字符串，共 1616 个字符串。

第 ii 个字符串表示数独的第 ii 行。

字符串包含字符可能为字母 A∼PA∼P 或 `-`（表示等待填充）。

测试用例之间用单个空行分隔，输入至文件结尾处终止。

#### 输出格式

对于每个测试用例，均要求保持与输入相同的格式，将填充完成后的数独输出。

每个测试用例输出结束后，输出一个空行。

#### 输入样例：

    --A----C-----O-I
    -J--A-B-P-CGF-H-
    --D--F-I-E----P-
    -G-EL-H----M-J--
    ----E----C--G---
    -I--K-GA-B---E-J
    D-GP--J-F----A--
    -E---C-B--DP--O-
    E--F-M--D--L-K-A
    -C--------O-I-L-
    H-P-C--F-A--B---
    ---G-OD---J----H
    K---J----H-A-P-L
    --B--P--E--K--A-
    -H--B--K--FI-C--
    --F---C--D--H-N-
    

#### 输出样例：

    FPAHMJECNLBDKOGI
    OJMIANBDPKCGFLHE
    LNDKGFOIJEAHMBPC
    BGCELKHPOFIMAJDN
    MFHBELPOACKJGNID
    CILNKDGAHBMOPEFJ
    DOGPIHJMFNLECAKB
    JEKAFCNBGIDPLHOM
    EBOFPMIJDGHLNKCA
    NCJDHBAEKMOFIGLP
    HMPLCGKFIAENBDJO
    AKIGNODLBPJCEFMH
    KDEMJIFNCHGAOPBL
    GLBCDPMHEONKJIAF
    PHNOBALKMJFIDCEG
    IAFJOECGLDPBHMNK
    
    

难度：困难

时/空限制：2s / 64MB

总通过数：2502

总尝试数：4823

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3720&show_algorithm_tags=0)[ZOJ3122](https://www.acwing.com/problem/search/1/?search_content=ZOJ3122&source_file_id=3720&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3720&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3720&show_algorithm_tags=1)[剪枝](https://www.acwing.com/problem/search/1/?search_content=%E5%89%AA%E6%9E%9D&source_file_id=3720&show_algorithm_tags=1)[DLX](https://www.acwing.com/problem/search/1/?search_content=DLX&source_file_id=3720&show_algorithm_tags=1)[Dancing Links](https://www.acwing.com/problem/search/1/?search_content=Dancing%20Links&source_file_id=3720&show_algorithm_tags=1)