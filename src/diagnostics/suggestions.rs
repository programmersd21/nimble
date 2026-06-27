use crate::diagnostics::span::DiagnosticSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Applicability {
    MachineApplicable,
    MaybeIncorrect,
    HasPlaceholders,
    Unspecified,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FixIt {
    pub span: DiagnosticSpan,
    pub replacement: String,
}

impl FixIt {
    pub fn new(span: DiagnosticSpan, replacement: impl Into<String>) -> Self {
        FixIt {
            span: span,
            replacement: replacement.into(),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Suggestion {
    pub msg: String,
    pub applicability: Applicability,
    pub substitutions: Vec<FixIt>,
}

impl Suggestion {
    pub fn new(
        msg: impl Into<String>,
        applicability: Applicability,
        substitutions: Vec<FixIt>,
    ) -> Self {
        Suggestion {
            msg: msg.into(),
            applicability,
            substitutions,
        }
    }
}

/// Damerau-Levenshtein distance handles insertions, deletions, substitutions, and transpositions.
pub fn damerau_levenshtein(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut dp = vec![vec![0; len2 + 2]; len1 + 2];
    let max_dist = len1 + len2;
    dp[0][0] = max_dist;

    for i in 0..=len1 {
        dp[i + 1][0] = max_dist;
        dp[i + 1][1] = i;
    }
    for j in 0..=len2 {
        dp[0][j + 1] = max_dist;
        dp[1][j + 1] = j;
    }

    let mut da = std::collections::HashMap::new();

    for i in 1..=len1 {
        let mut db = 0;
        for j in 1..=len2 {
            let k = *da.get(&v2[j - 1]).unwrap_or(&0);
            let l = db;

            let cost = if v1[i - 1] == v2[j - 1] {
                db = j;
                0
            } else {
                1
            };

            dp[i + 1][j + 1] = std::cmp::min(
                dp[i][j] + cost, // substitution
                std::cmp::min(
                    dp[i + 1][j] + 1, // insertion
                    std::cmp::min(
                        dp[i][j + 1] + 1, // deletion
                        if k > 0 && l > 0 {
                            dp[k][l] + (i - k - 1) + 1 + (j - l - 1) // transposition
                        } else {
                            max_dist
                        },
                    ),
                ),
            );
        }
        da.insert(v1[i - 1], i);
    }

    dp[len1 + 1][len2 + 1]
}

/// Find similar names among candidates to suggest as spelling corrections.
pub fn get_spelling_suggestions(
    query: &str,
    candidates: &[&str],
    limit: usize,
) -> Vec<(String, usize)> {
    let mut results: Vec<(String, usize)> = candidates
        .iter()
        .map(|c| (c.to_string(), damerau_levenshtein(query, c)))
        // Filter out candidates that are too distant.
        .filter(|(name, dist)| *dist <= 3 && *dist < query.len().max(name.len()) / 2 + 1)
        .collect();

    results.sort_by(|a, b| {
        a.1.cmp(&b.1)
            .then_with(|| {
                a.0.len()
                    .abs_diff(query.len())
                    .cmp(&b.0.len().abs_diff(query.len()))
            })
            .then_with(|| a.0.cmp(&b.0))
    });

    results.truncate(limit);
    results
}
