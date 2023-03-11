//! # yad_semver (Yet Another Damn Semver)
//! 
//! This crate provides a simple [SemVer 2.0](https://semver.org/spec/v2.0.0.html) implementation.
//! 
//! SemVer structs can be converted to/from strings, and can be compared.
//! 
//! This crate exists because the "semver" crate is "for Cargo's flavor of Semantic Versioning",
//! whereas this crate structly follows the semver 2.0 specification.
//! 
//! ## Usage
//! 
//! ```rust
//! use yad_semver::SemVer;
//! 
//! // You can create SemVer structs in place
//! let v1 = SemVer::new(1, 0, 0, None, None);
//! 
//! // Or from strings
//! let v2 = "2.0.0-alpha".parse::<SemVer>().unwrap();
//! 
//! // SemVers can be compared and displayed
//! use std::cmp::max;
//! println!("The newest version is {}", max(v1, v2));
//! ```

use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::min;
use std::fmt::Display;


#[derive(PartialEq, Eq, Ord)]
pub struct SemVer
{
    pub major: u128,
    pub minor: u128,
    pub patch: u128,
    pub pre_release: Option<String>,
    pub build_meta: Option<String>,
}


impl SemVer
{
    pub fn new(major: u128, minor: u128, patch: u128, pre_release: Option<String>, build_meta: Option<String>) -> Self
    {
        Self { major, minor, patch, pre_release, build_meta }
    }
}


impl Clone for SemVer
{
    fn clone(&self) -> Self
    {
        Self
        {
            major: self.major,
            minor: self.minor,
            patch: self.patch,
            pre_release: self.pre_release.clone(),
            build_meta: self.build_meta.clone(),
        }
    }
}


#[derive(Debug)]
pub struct SemVerParseError
{
    pub version: String,
}


impl Display for SemVer
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match (&self.pre_release, &self.build_meta)
        {
            (Some(pre_release), Some(build_meta)) =>
            {
                f.write_fmt(format_args!("{}.{}.{}-{}+{}", self.major, self.minor, self.patch, pre_release, build_meta))
            }
            (Some(pre_release), None) =>
            {
                f.write_fmt(format_args!("{}.{}.{}-{}", self.major, self.minor, self.patch, pre_release))
            }
            (None, Some(build_meta)) =>
            {
                f.write_fmt(format_args!("{}.{}.{}+{}", self.major, self.minor, self.patch, build_meta))
            }
            (None, None) =>
            {
                f.write_fmt(format_args!("{}.{}.{}", self.major, self.minor, self.patch))
            }
        }
    }
}


impl FromStr for SemVer
{
    type Err = SemVerParseError;


    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let error = Self::Err { version: s.to_owned() };
        
        lazy_static!
        {
            // See https://semver.org/spec/v2.0.0.html#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
            static ref RE: Regex = Regex::new(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();
        }

        let Some(captures) = RE.captures(s) else { return Err(error); };

        let Some(major) = captures.name("major") else { return Err(error); };
        let Some(minor) = captures.name("minor") else { return Err(error); };
        let Some(patch) = captures.name("patch") else { return Err(error); };

        let Ok(major) = major.as_str().parse() else { return Err(error); };
        let Ok(minor) = minor.as_str().parse() else { return Err(error); };
        let Ok(patch) = patch.as_str().parse() else { return Err(error); };

        let pre_release = captures.name("prerelease").and_then(|m| Some(m.as_str().to_owned()));
        let build_meta = captures.name("buildmetadata").and_then(|m| Some(m.as_str().to_owned()));

        Ok(Self { major, minor, patch, pre_release, build_meta })
    }
}


impl PartialOrd for SemVer
{
    fn ge(&self, other: &Self) -> bool
    {
        // Simple check
        if
            (self.major > other.major) ||
            (self.minor > other.minor) ||
            (self.patch > other.patch) ||
            (self.pre_release.is_none() && other.pre_release.is_some())
        {
            return true;
        }

        // More complicated checks
        let ours = &self.pre_release;
        let theirs = &other.pre_release;

        if ours.is_some() && theirs.is_some()
        {
            let ours = ours.as_ref().unwrap().split(".").collect::<Vec<&str>>();
            let theirs = theirs.as_ref().unwrap().split(".").collect::<Vec<&str>>();

            for i in 0..min(ours.len(), theirs.len())
            {
                let ours = ours[i];
                let theirs = theirs[i];

                if ours == theirs { continue; }

                let ours_num = ours.parse::<u128>();
                let theirs_num = theirs.parse::<u128>();

                return match (ours_num.is_ok(), theirs_num.is_ok())
                {
                    (true, true) =>
                    {
                        let ours_num = ours_num.unwrap();
                        let theirs_num = theirs_num.unwrap();

                        ours_num > theirs_num
                    }
                    (true, false) => false,
                    (false, true) => true,
                    (false, false) => ours > theirs,
                };
            }

            // If we are at this point, then all the prerelease fields are equal
            return ours.len() > theirs.len();
        }
        else if ours.is_none() && theirs.is_some()
        {
            return true;
        }

        return false;
    }


    fn gt(&self, other: &Self) -> bool
    {
        self == other && self.ge(other)
    }


    fn le(&self, other: &Self) -> bool
    {
        other.gt(self)
    }


    fn lt(&self, other: &Self) -> bool
    {
        other.ge(self)
    }


    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        if self == other
        {
            Some(std::cmp::Ordering::Equal)
        }
        else if self < other
        {
            Some(std::cmp::Ordering::Less)
        }
        else
        {
            Some(std::cmp::Ordering::Greater)
        }
    }
}


#[cfg(test)]
mod tests;
