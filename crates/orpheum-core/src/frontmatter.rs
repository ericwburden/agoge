use serde::de::DeserializeOwned;

use crate::error::{OrpheumError, OrpheumErrorCode};

pub fn split_frontmatter(text: &str) -> Result<(&str, &str), OrpheumError> {
    let opening_len = if text.starts_with("---\r\n") {
        5
    } else if text.starts_with("---\n") {
        4
    } else {
        0
    };

    if opening_len == 0 {
        return Err(OrpheumError::coded(
            OrpheumErrorCode::InvalidMetadata,
            "missing YAML frontmatter",
        ));
    }

    let remainder = &text[opening_len..];
    let mut yaml_len = 0usize;
    let mut body_start = None;

    for segment in remainder.split_inclusive(['\n']) {
        if segment == "---\n" || segment == "---\r\n" || segment == "---" {
            body_start = Some(opening_len + yaml_len + segment.len());
            break;
        }
        yaml_len += segment.len();
    }

    let (yaml_end, body_start) = match body_start {
        Some(body_start) => (opening_len + yaml_len, body_start),
        None => {
            return Err(OrpheumError::coded(
                OrpheumErrorCode::InvalidMetadata,
                "frontmatter terminator not found",
            ));
        }
    };

    if body_start <= opening_len || yaml_end < opening_len {
        return Err(OrpheumError::coded(
            OrpheumErrorCode::InvalidMetadata,
            "frontmatter boundaries are invalid",
        ));
    }

    let yaml = &text[opening_len..yaml_end];
    let body = text.get(body_start..).ok_or_else(|| {
        OrpheumError::coded(
            OrpheumErrorCode::InvalidMetadata,
            "frontmatter body boundary is invalid",
        )
    })?;
    Ok((yaml, body))
}

pub fn parse_frontmatter<T>(text: &str) -> Result<(T, String), OrpheumError>
where
    T: DeserializeOwned,
{
    let (yaml, body) = split_frontmatter(text)?;
    let value = serde_yaml::from_str::<T>(yaml).map_err(|err| {
        OrpheumError::coded(
            OrpheumErrorCode::InvalidMetadata,
            format!("invalid frontmatter: {err}"),
        )
    })?;
    Ok((value, body.to_string()))
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::parse_frontmatter;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Example {
        id: String,
    }

    #[test]
    fn parses_crlf_frontmatter() {
        let text = "---\r\nid: test\r\n---\r\n\r\n# Heading\r\n";
        let (value, body) = parse_frontmatter::<Example>(text).expect("frontmatter should parse");
        assert_eq!(value, Example { id: "test".into() });
        assert!(body.contains("# Heading"));
    }
}
