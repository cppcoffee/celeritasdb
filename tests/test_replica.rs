#[macro_use]
extern crate epaxos;

#[cfg(test)]
use pretty_assertions::assert_eq;

use epaxos::qpaxos::*;
use std::time::Duration;

use crate::support::*;
use tokio::time::delay_for;

mod support;

#[test]
fn test_replica_exec_thread() {
    _test_replica_exec_thread();
}

#[tokio::main]
async fn _test_replica_exec_thread() {
    let ctx = InProcContext::new();

    let cases = [
        (
            inst!(
                (1, 0),
                (3, 4, 2),
                [("Set", "x", "y")],
                [(1, 0)],
                [(1, 0)],
                [(1, 0)],
                true,
                false,
            ),
            InstanceId::from((1, 0)),
        ),
        (
            inst!(
                (1, 1),
                (3, 4, 2),
                [("Set", "z", "a")],
                [(1, 0)],
                [(1, 0)],
                [(1, 0)],
                true,
                false,
            ),
            InstanceId::from((1, 1)),
        ),
    ];

    // there is only replica

    for (inst, max) in cases.iter() {
        let rid: ReplicaId = 1;

        let sd = &ctx.server.server_data;
        let r = sd.local_replicas.get(&rid).unwrap();
        let sto = &r.storage;
        sto.set_instance(&inst).unwrap();
        sto.set_ref("max", 1, *max).unwrap();

        loop {
            let inst1 = sto
                .get_instance(inst.instance_id.unwrap())
                .unwrap()
                .unwrap();
            if inst1.executed {
                break;
            }

            delay_for(Duration::from_millis(10)).await;
        }

        for cmd in inst.cmds.iter() {
            let v = sto.get_kv(&cmd.key).unwrap().unwrap();
            assert_eq!(v, cmd.value);
        }
    }
}
