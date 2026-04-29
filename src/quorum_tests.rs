#[cfg(test)]
mod tests {
    use crate::quorum::Quorum;
    use crate::quorum_config::QuorumConfig;

    fn make_quorum(threshold: usize) -> Quorum {
        Quorum::new(QuorumConfig::new(threshold, "test"))
    }

    #[test]
    fn single_vote_threshold_one() {
        let q = make_quorum(1);
        assert!(!q.is_reached().unwrap());
        let reached = q.vote("node-a").unwrap();
        assert!(reached);
        assert!(q.is_reached().unwrap());
    }

    #[test]
    fn majority_config() {
        let cfg = QuorumConfig::majority(3, "cluster");
        assert_eq!(cfg.threshold, 2);
    }

    #[test]
    fn quorum_not_reached_until_threshold() {
        let q = make_quorum(3);
        assert!(!q.vote("a").unwrap());
        assert!(!q.vote("b").unwrap());
        assert!(q.vote("c").unwrap());
        assert_eq!(q.vote_count().unwrap(), 3);
    }

    #[test]
    fn duplicate_vote_does_not_inflate_count() {
        let q = make_quorum(2);
        q.vote("x").unwrap();
        q.vote("x").unwrap(); // same participant
        assert_eq!(q.vote_count().unwrap(), 1);
        assert!(!q.is_reached().unwrap());
    }

    #[test]
    fn reset_clears_votes() {
        let q = make_quorum(1);
        q.vote("node").unwrap();
        assert!(q.is_reached().unwrap());
        q.reset().unwrap();
        assert!(!q.is_reached().unwrap());
        assert_eq!(q.vote_count().unwrap(), 0);
    }

    #[test]
    fn voters_returns_sorted_list() {
        let q = make_quorum(3);
        q.vote("charlie").unwrap();
        q.vote("alice").unwrap();
        q.vote("bob").unwrap();
        let voters = q.voters().unwrap();
        assert_eq!(voters, vec!["alice", "bob", "charlie"]);
    }

    #[test]
    fn default_config_threshold_is_one() {
        let cfg = QuorumConfig::default();
        assert_eq!(cfg.threshold, 1);
    }
}
