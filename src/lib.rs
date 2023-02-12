use either::Either;
use lazy_static::lazy_static;
use regex::Regex;

pub struct LocalVersion {
    pub parsed: Vec<Either<String, u32>>,
    pub local_version_string: String,
}

#[derive(Debug)]
pub struct Version {
    pub epoch: Option<u32>,
    pub release: Vec<u32>,
    pub pre: Option<(String, Vec<u32>)>,
    pub post: Option<u32>,
    pub dev: Option<u32>,
    pub local: Option<String>,
    pub version_string: String,
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.epoch == other.epoch
        && self.release == other.release
        && self.pre == other.pre
        && self.post == other.post
        && self.dev == other.dev
        && self.local == other.local
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.epoch != other.epoch {
            return Some(self.epoch.cmp(&other.epoch));
        }
        if self.release != other.release {
            return Some(self.release.cmp(&other.release));
        }
        if self.pre != other.pre {
            match (self.pre.clone(), other.pre.clone()) {
                (Some(_), None) => return Some(std::cmp::Ordering::Greater),
                (None, Some(_)) => return Some(std::cmp::Ordering::Less),
                (None, None) => return Some(std::cmp::Ordering::Equal),
                (Some((this_pre, this_version)), Some((other_pre, other_version))) => {
                    if this_pre != other_pre {
                        return Some(this_pre.cmp(&other_pre));
                    }
                    return Some(this_version.cmp(&other_version));
                }
            }
        }
        if self.post != other.post {
            return Some(self.post.cmp(&other.post));
        }
        if self.dev != other.dev {
            return Some(self.dev.cmp(&other.dev));
        }
        if self.local != other.local {
            return Some(self.local.cmp(&other.local));
        }
        return None;
    }
}

type Operator = String;
type Extra = String;
type VersionConstraint = (Operator, Version);
type Requirement = (String, Vec<VersionConstraint>, Extra);

type Error = String;

#[derive(Debug)]
pub struct Metadata {
    pub metadata_version: Version,
    pub name: String,
    pub version: Version,
    pub dynamic: Option<String>,
    pub platforms: Option<Vec<String>>,
    pub suported_platforms: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub description_content_type: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub homepage: Option<String>,
    pub download_url: Option<String>,
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub maintainer: Option<String>,
    pub maintainer_email: Option<String>,
    pub license: Option<String>,
    pub classifiers: Option<Vec<String>>,
    pub requires_dist: Option<Vec<Requirement>>,
    pub requires_python: Option<Vec<VersionConstraint>>,
    pub requires_externals: Option<Vec<String>>,
    pub project_urls: Option<Vec<String>>,
    pub provides_extras: Option<Vec<Extra>>,
}

pub fn get_metadata_attribute(metadata_string: &String, attribute: String) -> Result<String, Error> {
    let attribute = metadata_string.lines()
                                   .find(|line| line.starts_with(&attribute))
                                   .map(|line| line.split(": ").nth(1).unwrap().to_string())
                                   .ok_or(format!("{} not found", attribute))?;
    Ok(attribute)
}

/// Parsing PEP 440 conformant version strings
///
/// https://peps.python.org/pep-0440/
///
/// Args:
/// - `version_string` (String): A version string
///
/// Returns `Result<Version, Error>`: A Version object
pub fn parse_version(version_string: String) -> Result<Version, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^\s*v?(?:(?:(?P<epoch>[0-9]+)!)?(?P<release>[0-9]+(?:\.[0-9]+)*)(?P<pre>[-_\.]?(?P<pre_l>(a|b|c|rc|alpha|beta|pre|preview))[-_\.]?(?P<pre_n>[0-9]+)?)?(?P<post>(?:-(?P<post_n1>[0-9]+))|(?:[-_\.]?(?P<post_l>post|rev|r)[-_\.]?(?P<post_n2>[0-9]+)?))?(?P<dev>[-_\.]?(?P<dev_l>dev)[-_\.]?(?P<dev_n>[0-9]+)?)?)(?:\+(?P<local>[a-z0-9]+(?:[-_\.][a-z0-9]+)*))?\s*$"
        ).unwrap();
    }

    let captures = RE.captures(&version_string)
                     .ok_or(format!("{} is not a valid version", version_string))?;

    let epoch = captures.name("epoch")
                        .map(|epoch| epoch.as_str().parse::<u32>().unwrap());
    let release = captures.name("release").map(|release| {
                                              release.as_str()
                                                     .split(".")
                                                     .map(|n| n.parse::<u32>().unwrap())
                                                     .collect::<Vec<u32>>()
                                          });
    let pre = captures.name("pre").map(|pre| {
                                      (pre.as_str().to_string(),
                                       captures.name("pre_l").map(|pre_l| pre_l.as_str().to_string()).unwrap(),
                                       captures.name("pre_n")
                                               .map(|pre_n| pre_n.as_str().parse::<u32>().unwrap())
                                               .unwrap())
                                  });
    let post = captures.name("post").map(|post| post.as_str().parse::<u32>().unwrap());
    let dev = captures.name("dev").map(|dev| dev.as_str().parse::<u32>().unwrap());
    let local = captures.name("local").map(|local| local.as_str().to_string());

    Ok(Version { epoch,
                 release: release.unwrap(),
                 pre: pre.map(|pre| (pre.1, vec![pre.2])),
                 post,
                 dev,
                 local,
                 version_string })
}

pub fn parse_metadata(metadata_string: String) -> Result<Metadata, Error> {
    let metadata_version =
        parse_version(get_metadata_attribute(&metadata_string, "Metadata-Version:".to_string())?).unwrap();

    let name = get_metadata_attribute(&metadata_string, "Name:".to_string())?;

    let version = parse_version(get_metadata_attribute(&metadata_string, "Version:".to_string())?).unwrap();

    Ok(Metadata { metadata_version,
                  name,
                  version,
                  dynamic: None,
                  platforms: None,
                  suported_platforms: None,
                  summary: None,
                  description: None,
                  description_content_type: None,
                  keywords: None,
                  homepage: None,
                  download_url: None,
                  author: None,
                  author_email: None,
                  maintainer: None,
                  maintainer_email: None,
                  license: None,
                  classifiers: None,
                  requires_dist: None,
                  requires_python: None,
                  requires_externals: None,
                  project_urls: None,
                  provides_extras: None })
}
