use crate::qpaxos::MakeReply;
use crate::qpaxos::MakeRequest;
use crate::qpaxos::QPaxos;
use crate::qpaxos::FastAcceptRequest;
use crate::qpaxos::FastAcceptReply;
use crate::qpaxos::AcceptRequest;
use crate::qpaxos::AcceptReply;
use crate::qpaxos::CommitRequest;
use crate::qpaxos::CommitReply;
use crate::qpaxos::PrepareRequest;
use crate::qpaxos::PrepareReply;
use crate::qpaxos::Instance;

use tonic;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyQPaxos {}

#[tonic::async_trait]
impl QPaxos for MyQPaxos {
    async fn fast_accept(
        &self,
        request: Request<FastAcceptRequest>,
    ) -> Result<Response<FastAcceptReply>, Status> {
        // TODO I did nothing but let the test pass happily
        let inst = Instance {
            ..Default::default()
        };

        let reply = MakeReply::fast_accept(&inst, &[]);
        println!("Got a request: {:?}", request);
        Ok(Response::new(reply))
    }

    async fn accept(
        &self,
        request: Request<AcceptRequest>,
    ) -> Result<Response<AcceptReply>, Status> {
        // TODO I did nothing but let the test pass happily
        let inst = Instance {
            ..Default::default()
        };

        let reply = MakeReply::accept(&inst);
        println!("Got a request: {:?}", request);
        Ok(Response::new(reply))
    }

    async fn commit(
        &self,
        request: Request<CommitRequest>,
    ) -> Result<Response<CommitReply>, Status> {
        // TODO I did nothing but let the test pass happily
        let inst = Instance {
            ..Default::default()
        };

        let reply = MakeReply::commit(&inst);
        println!("Got a request: {:?}", request);
        Ok(Response::new(reply))
    }

    async fn prepare(
        &self,
        request: Request<PrepareRequest>,
    ) -> Result<Response<PrepareReply>, Status> {
        // TODO I did nothing but let the test pass happily
        let inst = Instance {
            ..Default::default()
        };

        let reply = MakeReply::prepare(&inst);
        println!("Got a request: {:?}", request);
        Ok(Response::new(reply))
    }
}
