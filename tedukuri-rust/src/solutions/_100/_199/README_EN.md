199\. 余数之和


给出正整数 $n$ 和 $k$，计算 $j(n, k)=k \\bmod 1 + k \\bmod 2 + k \\bmod 3 + … + k \\bmod n$ 的值。

例如 $j(5, 3)=3 \\bmod 1 + 3 \\bmod 2 + 3 \\bmod 3 + 3 \\bmod 4 + 3 \\bmod 5=0+1+0+3+3=7$。

#### 输入格式

输入仅一行，包含两个整数 $n, k$。

#### 输出格式

输出仅一行，即 $j(n, k)$。

#### 数据范围

$1 \\le n,k \\le 10^9$

#### 输入样例：

```
5 3
```

#### 输出样例：

```
7
```

<table class="table table-striped table-responsive"><tbody><tr><td>难度： <span class="label label-warning round">中等</span></td></tr><tr><td>时/空限制： <span>1s / 64MB</span></td></tr><tr><td>总通过数： <span>2529</span></td></tr><tr><td>总尝试数： <span>5508</span></td></tr><tr><td>来源：</td></tr><tr><td><span class="problem-algorithm-tag">算法标签<span class="caret"></span></span></td></tr></tbody></table>

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