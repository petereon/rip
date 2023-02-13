#[cfg(test)]
mod metadata_parse {
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

mod version_parse {
    use rip;

    #[test]
    fn test_parse_release_version() {
        assert_eq!(rip::parse_version("1.5.3".to_string()),
                   Ok(rip::Version { epoch: None,
                                     release: [1, 5, 3].to_vec(),
                                     pre: None,
                                     post: None,
                                     dev: None,
                                     local: None,
                                     version_string: "".to_string() }))
    }

    #[test]
    fn test_parse_release_version_with_epoch_and_dev() {
        assert_eq!(rip::parse_version("2!1.dev0".to_string()),
                   Ok(rip::Version { epoch: Some(2),
                                     release: [1].to_vec(),
                                     pre: None,
                                     post: None,
                                     dev: Some(0),
                                     local: None,
                                     version_string: "".to_string() }))
    }

    #[test]
    fn test_parse_release_version_with_alpha() {
        assert_eq!(rip::parse_version("1.5alpha1".to_string()),
                   Ok(rip::Version { epoch: None,
                                     release: [1, 5].to_vec(),
                                     pre: Some((rip::PreReleaseType::Alpha, [1].to_vec())),
                                     post: None,
                                     dev: None,
                                     local: None,
                                     version_string: "".to_string() }))
    }

    #[test]
    fn test_parse_release_version_with_beta() {
        assert_eq!(rip::parse_version("1.5b2".to_string()),
                   Ok(rip::Version { epoch: None,
                                     release: [1, 5].to_vec(),
                                     pre: Some((rip::PreReleaseType::Beta, [2].to_vec())),
                                     post: None,
                                     dev: None,
                                     local: None,
                                     version_string: "".to_string() }))
    }

    #[test]
    fn test_parse_release_version_with_rc() {
        assert_eq!(rip::parse_version("1.5rc5".to_string()),
                   Ok(rip::Version { epoch: None,
                                     release: [1, 5].to_vec(),
                                     pre: Some((rip::PreReleaseType::Rc, [5].to_vec())),
                                     post: None,
                                     dev: None,
                                     local: None,
                                     version_string: "".to_string() }))
    }

    #[test]
    fn test_parse_release_version_with_preview() {
        assert_eq!(rip::parse_version("1.5-preview1".to_string()),
                   Ok(rip::Version { epoch: None,
                                     release: [1, 5].to_vec(),
                                     pre: Some((rip::PreReleaseType::Rc, [1].to_vec())),
                                     post: None,
                                     dev: None,
                                     local: None,
                                     version_string: "".to_string() }))
    }

    #[test]
    fn test_parse_release_version_with_post() {
        assert_eq!(rip::parse_version("1.3.9-post12".to_string()),
                   Ok(rip::Version { epoch: None,
                                     release: [1, 3, 9].to_vec(),
                                     pre: None,
                                     post: Some(12),
                                     dev: None,
                                     local: None,
                                     version_string: "1.dev0".to_string() }))
    }

    #[test]
    fn test_parse_release_version_with_dev() {
        assert_eq!(rip::parse_version("1.dev0".to_string()),
                   Ok(rip::Version { epoch: None,
                                     release: [1].to_vec(),
                                     pre: None,
                                     post: None,
                                     dev: Some(0),
                                     local: None,
                                     version_string: "1.dev0".to_string() }))
    }

    #[test]
    fn test_parse_release_version_with_local() {
        assert_eq!(rip::parse_version("1.2.3+some.local.version".to_string()),
                   Ok(rip::Version { epoch: None,
                                     release: [1, 2, 3].to_vec(),
                                     pre: None,
                                     post: None,
                                     dev: None,
                                     local: Some("some.local.version".to_string()),
                                     version_string: "1.dev0".to_string() }))
    }
}

mod version_compare {

    #[test]
    fn test_version_eq() {
        let version1 = rip::Version { epoch: None,
                                      release: [1, 5, 3].to_vec(),
                                      pre: None,
                                      post: None,
                                      dev: None,
                                      local: None,
                                      version_string: "1.5.3".to_string() };
        let version2 = rip::Version { epoch: None,
                                      release: [1, 5, 3].to_vec(),
                                      pre: None,
                                      post: None,
                                      dev: None,
                                      local: None,
                                      version_string: "1.05.3".to_string() };
        assert_eq!(version1, version2);
    }

    #[test]
    fn test_version_compare_basic_release() {
        let version1 = rip::Version { epoch: None,
                                      release: [1, 5, 3].to_vec(),
                                      pre: None,
                                      post: None,
                                      dev: None,
                                      local: None,
                                      version_string: "1.5.3".to_string() };
        let version2 = rip::Version { epoch: None,
                                      release: [1, 5, 4].to_vec(),
                                      pre: None,
                                      post: None,
                                      dev: None,
                                      local: None,
                                      version_string: "1.05.4".to_string() };
        assert!(version1 < version2);
    }

    #[test]
    fn test_version_compare_epoch() {
        let version1 = rip::Version { epoch: None,
                                      release: [2022, 2, 3].to_vec(),
                                      pre: None,
                                      post: None,
                                      dev: None,
                                      local: None,
                                      version_string: "1.5.3".to_string() };
        let version2 = rip::Version { epoch: Some(1),
                                      release: [1, 5, 4].to_vec(),
                                      pre: None,
                                      post: None,
                                      dev: None,
                                      local: None,
                                      version_string: "1.05.4".to_string() };
        assert!(version1 < version2);
    }

    // #[test]
    // fn test_version_compare_epoch() {
    //     let version1 = rip::Version { epoch: None,
    //                                   release: [1, 5, 4].to_vec(),
    //                                   pre: None,
    //                                   post: None,
    //                                   dev: None,
    //                                   local: None,
    //                                   version_string: "1.5.3".to_string() };
    //     let version2 = rip::Version { epoch: None,
    //                                   release: [1, 5, 4].to_vec(),
    //                                   pre: None,
    //                                   post: None,
    //                                   dev: None,
    //                                   local: None,
    //                                   version_string: "1.05.4".to_string() };
    //     assert!(version1 < version2);
    // }
}
