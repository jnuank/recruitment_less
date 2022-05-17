use gateway::candidate_gateway::CandidateGateway;

struct GetCandidate {
    gateway: CandidateGateway
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_候補者が取得できる() {

        assert_eq!(1, 1)
    }
}