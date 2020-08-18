# Execution

- Find out the set of smallest instances of every leader: `S`.
  Easy to see `S` includes all instances those should execute first.

- If there are any `a → b` relations(`a.final_deps ⊃ b.final_deps`)
  in `S`, replace `S` with:
  `S = {x | x ∈ S and (∃y: y → x)}`

  Repeat this step until there is no `a → b` in `S`.

- Execute all instances in `S` in instance-id-order.




```
Interfering relation:

a ~ b
b ~ c
c ~ d

---

   b
a  x  c  d
-- -- -- -- --
R0 R1 R2 R3 R4

流程:
a --> R1 
b --> R2
c --> R3
c --> R4
b --> R4
a --> R2
c slow-commit c.final_deps = {d}
b slow-commit b.final_deps = {c}
a slow-commit a.final_deps = {b}
x slow-commit x.final_deps = {}


0 a₀
1 x₀ b₀ a
2 c₀      b     a
3 d₀        c
4           c b
-------------------------------> time


最后 没有任何一个 after 关系形成.

如果从R0开始exec, 看到2个instance: a, x
如果从R2开始exec, 看到2个instance: c, d

会导致最先执行的instance不同.
```


```
 p →q
  ↖ ↓
    b
  ↗   ↘
a   x   c → d


从一个instnace a 开始执行时, 必须找到所有可能在a之前执行的instance.

∴ DFS from a, 走到下一个depends-on的instance, 优先选择最小的instance_id.

如果遇到环, 则环中某个节点


→ : depends on, may be after
> : depends on, and after

假设从a到z有这样一条路径连通:

a → ... → b → ... → x → ... z
x > b

则b一定在x之前执行
∴ 所有从a 到 x的路径, 都会先执行b
∴ 从a开始遍历所有路径, DFS, 找到所有这样的x则回溯: x > y: y是一个遍历过的节点.

遍历结束后没有到达的instance肯定不是需要第一个被执行的.

遍历一个leader 2 次就会找到一个 x > y的关系.

从遍历到的节点中, 执行之前的exec算法.



如果 A → b, A是一个SCC, 那必须先执行,  因为b可能看不到A

∵ exec-consistency
∴ 如果从a 到z 有一条路径: a → ... → z, 但z到a没有路径.
  那么决定a和z的执行顺序算法, 必须先执行z, 因为z可能看不到a.

  如果a到z有一条路径: a → ... → z, 那么z执行之前,
  必须至少有一条a到z的路径不能被删除.



一个SCC中, exec候选节点定义为: 一个after关系的dest instance, 且没有出向的after边:
candidate = {x | ∃y: y > x and ∀z: x ≯ z}

假设执行算法确定, 对一个完整的图可以确定一个顺序.
要求执行算法对一个SCC中的
一个SCC内的执行顺序: 执行SCC中最小的candidate中的节点,
直到candidate为空, 再按照某种方式执行SCC中剩下的节点.


假设整个图的结构是多个SCC连成的(没有出向边的一个节点也是一个SCC):
S₁ → S₂ → S₃ ..
     ↘   ↗
       S₄ ..
// SCC 之间没有环, 否则会构成一个更大的SCC.

∴ 先执行任何一个SCC中candidate中的节点不会影响最终结果.
例如在S₂中的 x > y:
在图完整的情况下的执行, y在S₃全部执行结束之后运行, 且在它的x之前执行.


## exec流程:
TODO use y > x as a barrier to stop search 
a always see a committed instance.

选择每个replica Ri上的最小instance Xi, DFS, 优先选择最小instance id的路径.
当走到一个节点y 有 y > Xi, 则放弃对y的遍历, 回溯.

直到遍历完成, 或包括所有replica

- 发现一个没有出向边的节点: 直接执行.

- 有回到Xi的After路径, 则Xi作为一个优先执行备选.

- 没有回到Xi的After路径(回到Xi的都是depends-on路径),
  把Xi作为延后执行备选.

然后:

如果有大于0个优先执行备选, 选择最小的执行.
这时它一定是一个SCC中的最小candidate, 先执行它不会影响一致性问题:
根据 TODO

选择一个延后执行备选Xi, 遍历直到找到一个距离Xi最远的节点z.
从z开始遍历找到所有可达节点, 一起排序执行.

// 因为如果没有找到After关系的dest instance,
   说明最小的几个instance所在的SCC都是有限大的(不断增长的SCC很快会跟旧节点形成一个After关系)

   这时距离Xi最远的那个instance就是没有出向边的SCC, 需要第一个被执行.


```


