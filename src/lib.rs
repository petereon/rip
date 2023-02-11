type Version = Vec<char>;
type Operator = String;
type Extra = String;
type VersionConstraint = (Operator, Version);
type Requirement = (String, Vec<VersionConstraint>, Extra);

type Error = String;

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

pub fn parse_metadata() -> Result<Metadata, Error>{
    unimplemented!()
}