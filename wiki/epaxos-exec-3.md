# Probelm

In epaxos, the original execution would livelock:

> Avoiding Execution Livelock
> With a fast stream of interfering proposals, command execution could livelock:
> command γ will acquire dependencies on newer commands proposed between sending
> and receiving the PreAccept(γ). These new commands in turn gain dependencies on
> even newer commands.

Epaxos proposes a solution:

> To prevent this, we prioritize completing old commands over
> proposing new commands. Even without this op- timization, however, long
> dependency chains increase only execution latency, not commit latency. They also
> negligibly affect throughput, because executing a batch of n inter-dependent
> commands at once adds only modest computational overhead: finding the strongly
> connected components has linear time complexity (the number of dependencies for
> each command is usually constant— Section 4.5), and sorting the commands by
> their sequence attribute adds only an O(log n) factor.

This is not perfect because:

- All replicas have to defer new commands at the same time.
  One replica is quite enough to produce an infinite dependency chain.

- If searching for a big SCC takes long time, all commands in it are all
  delayed.
  The system latency would peek.

A better solution would a incremental execution algorithm which does not to deal
with circular dependency chain.

# Guarantees

The original epaxos execution guarantees are still offered.

- **Execution consistency**: If two interfering commands γ and δ are
  successfully committed (by any replicas) they will be executed in the same
  order by every replica.

- **Execution linearizability**: If two interfering commands γ and δ are
  serialized by clients (i.e., δ is pro- posed only after γ is committed by any
  replica), then every replica will execute γ before δ.

# Solution

This solution does not modify any part of the commit process.
It only uses another algorithm to execute.

First we turned the cyclic graph into a directed acyclic graph(DAG), so that there
wont be strongly connected component thus there wont be livelock.

Second we add additional dependency to ensure consistency.


Pseudo code:

```
def has_outgoing_edge(y):
    for d in y.deps:
        if not d.executed and y.seq > d.seq:
            return True
    return False

def exec(x):
    for d in x.deps:
        if d.executed:
            continue
        if x.seq > d.seq:
            exec(d)
        else:
            if x.id > d.id and not has_outgoing_edge(d):
                exec(d)

    apply_cmds(x)
    x.executed = True
```
