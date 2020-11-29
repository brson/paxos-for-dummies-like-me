/*

- https://en.wikipedia.org/wiki/Paxos_(computer_science)
- http://lamport.azurewebsites.net/pubs/pubs.html
- https://paxos.systems/
- https://lamport.azurewebsites.net/pubs/paxos-simple.pdf
- https://matklad.github.io/2020/11/01/notes-on-paxos.html
- https://explain.yshui.dev/distributed%20system/2020/09/20/paxos.html

*/

#![allow(unused)]
#![allow(non_camel_case_types)]

fn main() {
    println!("Hello, world!");
}

mod processor {
    use crate::roles::*;

    pub struct Processor {
        roles: Roles,
        others: Vec<RemoteProcessor>,
    }

    pub struct Roles {
        client: Option<Client>,
        acceptor: Option<Acceptor>,
        proposor: Option<Proposer>,
        learner: Option<Learner>,
        leader: Option<Leader>,
    }

    struct Id(u128);

    pub struct RemoteProcessor {
        id: Id,
        client: bool,
        acceptor: bool,
        proposor: bool,
        learner: bool,
        leader: bool,
    }


}

mod roles {

    pub struct Value(u32);
    pub struct ProposalNumber(u128);

    /// The Client issues a request to the distributed system, and waits for a
    /// response. For instance, a write request on a file in a distributed file
    /// server.
    pub struct Client;

    /// The Acceptors act as the fault-tolerant "memory" of the protocol. Acceptors
    /// are collected into groups called Quorums. Any message sent to an Acceptor
    /// must be sent to a Quorum of Acceptors. Any message received from an Acceptor
    /// is ignored unless a copy is received from each Acceptor in a Quorum.
    pub struct Acceptor {
        pub (crate) highest_proposal_number: ProposalNumber,
        pub (crate) last_accepted: Option<(ProposalNumber, Value)>;
    }

    /// A Proposer advocates a client request, attempting to convince the Acceptors
    /// to agree on it, and acting as a coordinator to move the protocol forward
    /// when conflicts occur.
    pub struct Proposer {
        pub (crate) next_proposal_number: ProposalNumber,
    }

    /// Learners act as the replication factor for the protocol. Once a Client
    /// request has been agreed upon by the Acceptors, the Learner may take action
    /// (i.e.: execute the request and send a response to the client). To improve
    /// availability of processing, additional Learners can be added.
    pub struct Learner;

    /// Paxos requires a distinguished Proposer (called the leader) to make
    /// progress. Many processes may believe they are leaders, but the protocol only
    /// guarantees progress if one of them is eventually chosen. If two processes
    /// believe they are leaders, they may stall the protocol by continuously
    /// proposing conflicting updates. However, the safety properties are still
    /// preserved in that case.
    pub struct Leader;
}

mod basic_paxos {

    use crate::roles::*;

    enum Message {
        Prepare(PrepareMessage),
    }

    struct PrepareMessage(ProposalNumber);
    
    enum BasicPaxosRound {
        Phase1(BasicPaxosRoundPhase1),
        Phase2(BasicPaxosRoundPhase2),
    }

    enum BasicPaxosRoundPhase1 {
        Phase1A_Prepare,
        Phase1B_Promise,
    }

    enum BasicPaxosRoundPhase2 {
        Phase2A_Accept,
        Phase2B_Accepted,
    }

    trait BasicProposer {
        fn create_prepare_msg(&mut self) -> Message;
    }

    impl BasicProposer for Proposer {
        fn create_prepare_msg(&mut self) -> PrepareMessage {
            let number = self.next_prepare_number;
            self.next_prepare_number += 1;
            PrepareMsg(ProposalNumber(number))
        }
    }

    trait BasicAcceptor {
        fn handle_prepare_msg(&mut self, msg: PrepareMsg) -> HandlePrepareResult {
            panic!()
        }
    }

    enum HandlePrepareResult {
        Promise(Promise),
    }

    struct Promise {
        
    }
    
}
