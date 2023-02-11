#[cfg(test)]
mod metadata_parse_tests {
    use rip;

    #[test]
    fn test_parse_metadata() {
        let metadata = rip::parse_metadata("Metadata-Version: 2.1\nName: pandas\nVersion: 1.5.3".to_string());
        assert!(metadata.as_ref().is_ok());
        assert_eq!(metadata.as_ref().unwrap().version,
                   rip::Version { epoch: None,
                                  release: [1, 5, 3].to_vec(),
                                  pre: None,
                                  post: None,
                                  dev: None,
                                  local: None,
                                  version_string: "1.5.3".to_string() });
        assert_eq!(metadata.as_ref().unwrap().metadata_version,
                   rip::Version { epoch: None,
                                  release: [2, 1].to_vec(),
                                  pre: None,
                                  post: None,
                                  dev: None,
                                  local: None,
                                  version_string: "2.1".to_string() });

        assert_eq!(metadata.as_ref().unwrap().name, "pandas".to_string());
    }
}
