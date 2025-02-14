197\. 阶乘分解


给定整数 $N$，试把阶乘 $N!$ 分解质因数，按照算术基本定理的形式输出分解结果中的 $p\_i$ 和 $c\_i$ 即可。

#### 输入格式

一个整数 $N$。

#### 输出格式

$N!$ 分解质因数后的结果，共若干行，每行一对 $p\_i, c\_i$，表示含有 $p\_i^{c\_i}$ 项。按照 $p\_i$ 从小到大的顺序输出。

#### 数据范围

$3 \\le N \\le 10^6$

#### 输入样例：

```
5
```

#### 输出样例：

```
2 3
3 1
5 1
```

#### 样例解释

$5! = 120 = 2^3 \* 3 \* 5$

<table class="table table-striped table-responsive"><tbody><tr><td>难度： <span class="label label-warning round">中等</span></td></tr><tr><td>时/空限制： <span>1s / 64MB</span></td></tr><tr><td>总通过数： <span>12371</span></td></tr><tr><td>总尝试数： <span>25261</span></td></tr><tr><td>来源：</td></tr><tr><td><span class="problem-algorithm-tag">算法标签<span class="caret"></span></span></td></tr></tbody></table>

* * *

代码编辑器设置

* * *

界面风格

对白色界面感到厌倦了吗？可以尝试其他的背景和代码高亮风格。

* * *

编辑类型

更喜欢Vim或者Emacs的输入方式吗？我们也为你提供了这些选项。

* * *

缩进长度

选择代码缩进的长度。默认是4个空格。

* * *

代码补全

写代码疲惫了吗？让我们来帮你一把。唤醒词列表在[这里](https://www.acwing.com/file_system/file/content/whole/index/content/2145234/)。

* * *