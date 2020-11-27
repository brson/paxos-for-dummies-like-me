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

mod id {
    
}

mod processor {
    use crate::roles::*;

    pub struct Processor {
        roles: Roles,
    }

    pub struct Roles {
        client: Option<Client>,
        acceptor: Option<Acceptor>,
        proposor: Option<Proposer>,
        learner: Option<Learner>,
        leader: Option<Leader>,
    }
}

mod roles {

    /// The Client issues a request to the distributed system, and waits for a
    /// response. For instance, a write request on a file in a distributed file
    /// server.
    pub struct Client;

    /// The Acceptors act as the fault-tolerant "memory" of the protocol. Acceptors
    /// are collected into groups called Quorums. Any message sent to an Acceptor
    /// must be sent to a Quorum of Acceptors. Any message received from an Acceptor
    /// is ignored unless a copy is received from each Acceptor in a Quorum.
    pub struct Acceptor; /* Voter */

    /// A Proposer advocates a client request, attempting to convince the Acceptors
    /// to agree on it, and acting as a coordinator to move the protocol forward
    /// when conflicts occur.
    pub struct Proposer {
        next_prepare_number: u128,
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

}
