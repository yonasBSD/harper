use super::PatternLinter;
use super::{Lint, LintKind, Suggestion};
use crate::make_title_case;
use crate::patterns::{EitherPattern, IsNotTitleCase, Pattern, SequencePattern};
use crate::FstDictionary;
use crate::{Token, TokenStringExt};
use std::sync::Arc;

/// A macro that will generate a linter to enforce capitalization of a multi-token proper noun.
macro_rules! create_linter_for {
    ($name:ident, $pattern:expr, $message:literal) => {
        create_linter_for!($name, $pattern, $message, $message);
    };
    ($name:ident, $pattern:expr, $message:literal, $description:literal) => {
        #[doc = $description]
        pub struct $name {
            pattern: Box<dyn Pattern>,
            dict: Arc<FstDictionary>,
        }

        impl $name {
            pub fn new() -> Self {
                let dict = FstDictionary::curated();

                Self {
                    pattern: Box::new(IsNotTitleCase::new(Box::new($pattern), dict.clone())),
                    dict,
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl PatternLinter for $name {
            fn pattern(&self) -> &dyn Pattern {
                self.pattern.as_ref()
            }

            fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Lint {
                let proper = make_title_case(matched_tokens, source, &self.dict);

                Lint {
                    span: matched_tokens.span().unwrap(),
                    lint_kind: LintKind::Capitalization,
                    suggestions: vec![Suggestion::ReplaceWith(proper)],
                    message: $message.to_string(),
                    priority: 31,
                }
            }

            fn description(&self) -> &'static str {
                $description
            }
        }
    };
}

create_linter_for!(
    Americas,
    SequencePattern::default()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::aco("South")),
            Box::new(SequencePattern::aco("North"))
        ])))
        .then_whitespace()
        .t_aco("America"),
    "When referring to the continents, make sure to treat them as a proper noun."
);

create_linter_for!(
    Koreas,
    SequencePattern::default()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::aco("South")),
            Box::new(SequencePattern::aco("North"))
        ])))
        .then_whitespace()
        .t_aco("Korea"),
    "When referring to the nations, make sure to treat them as a proper noun."
);

create_linter_for!(
    ChineseCommunistParty,
    SequencePattern::aco("Chinese")
        .then_whitespace()
        .t_aco("Communist")
        .then_whitespace()
        .t_aco("Party"),
    "When referring to the political party, make sure to treat them as a proper noun."
);

create_linter_for!(
    UnitedOrganizations,
    SequencePattern::default()
        .t_aco("United")
        .then_whitespace()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::aco("Nations")),
            Box::new(SequencePattern::aco("States")),
            Box::new(SequencePattern::aco("Kingdom")),
            Box::new(SequencePattern::aco("Airlines")),
            Box::new(
                SequencePattern::default()
                    .t_aco("Arab")
                    .then_whitespace()
                    .t_aco("Emirates")
            )
        ]))),
    "When referring to national or international organizations, make sure to treat them as a proper noun."
);

create_linter_for!(
    Holidays,
    EitherPattern::new(vec![
        Box::new(
            SequencePattern::default()
                .then(Box::new(EitherPattern::new(vec![
                    Box::new(SequencePattern::aco("Presidents'")),
                    Box::new(SequencePattern::aco("Valentines")),
                    Box::new(SequencePattern::aco("Christmas")),
                    Box::new(SequencePattern::aco("Easter")),
                    Box::new(SequencePattern::aco("Flag")),
                    Box::new(SequencePattern::aco("Independence")),
                    Box::new(SequencePattern::aco("Mothers'")),
                    Box::new(SequencePattern::aco("New").t_aco("Years")),
                    Box::new(SequencePattern::aco("Fathers'")),
                    Box::new(SequencePattern::aco("Columbus")),
                    Box::new(SequencePattern::aco("Thanksgiving")),
                    Box::new(SequencePattern::aco("Memorial")),
                    Box::new(SequencePattern::aco("May")),
                    Box::new(SequencePattern::aco("Halloween")),
                    Box::new(SequencePattern::aco("Tax")),
                    Box::new(SequencePattern::aco("Parents")),
                    Box::new(SequencePattern::aco("Veterans")),
                    Box::new(SequencePattern::aco("Armistice")),
                    Box::new(SequencePattern::aco("Groundhog")),
                    Box::new(
                        SequencePattern::default()
                            .t_aco("National")
                            .then_whitespace()
                            .t_aco("Freedom")
                    ),
                    Box::new(
                        SequencePattern::default()
                            .t_aco("All")
                            .then_whitespace()
                            .t_aco("Saints")
                    ),
                    Box::new(
                        SequencePattern::default()
                            .t_aco("All")
                            .then_whitespace()
                            .t_aco("Souls")
                    )
                ])))
                .then_whitespace()
                .t_aco("Day")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("Black")
                .then_whitespace()
                .t_aco("Friday")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("Cyber")
                .then_whitespace()
                .t_aco("Monday")
        )
    ]),
    "When referring to holidays, make sure to treat them as a proper noun."
);

create_linter_for!(
    AmazonNames,
    SequencePattern::default()
    .t_aco("Amazon")
    .then_whitespace()
    .then(Box::new(EitherPattern::new(vec![
        Box::new(
            SequencePattern::default()
                .t_aco("Shopping")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("Web")
                    .then_whitespace()
                .t_aco("Services")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("Lambda")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("RDS")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("DynamoDB")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("SageMaker")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("Rekognition")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("CloudFront")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("ECS")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("EKS")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("CloudWatch")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("IAM")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("Prime")
        ),
        Box::new(
            SequencePattern::default()
                .t_aco("Kindle")
        )
    ]))),
    "When referring to the various products of Amazon.com, make sure to treat them as a proper noun."
);

create_linter_for!(
    GoogleNames,
    SequencePattern::default()
        .t_aco("Google")
        .then_whitespace()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::aco("Search")),
            Box::new(SequencePattern::aco("Cloud")),
            Box::new(SequencePattern::aco("Maps")),
            Box::new(SequencePattern::aco("Docs")),
            Box::new(SequencePattern::aco("Sheets")),
            Box::new(SequencePattern::aco("Slides")),
            Box::new(SequencePattern::aco("Drive")),
            Box::new(SequencePattern::aco("Meet")),
            Box::new(SequencePattern::aco("Gmail")),
            Box::new(SequencePattern::aco("Calendar")),
            Box::new(SequencePattern::aco("Chrome")),
            Box::new(SequencePattern::aco("ChromeOS")),
            Box::new(SequencePattern::aco("Android")),
            Box::new(SequencePattern::aco("Play")),
            Box::new(SequencePattern::aco("Bard")),
            Box::new(SequencePattern::aco("Gemini")),
            Box::new(SequencePattern::aco("YouTube")),
            Box::new(SequencePattern::aco("Photos")),
            Box::new(SequencePattern::aco("Analytics")),
            Box::new(SequencePattern::aco("AdSense")),
            Box::new(SequencePattern::aco("Pixel")),
            Box::new(SequencePattern::aco("Nest")),
            Box::new(SequencePattern::aco("Workspace"))
        ]))),
    "When referring to Google products and services, make sure to treat them as proper nouns."
);

create_linter_for!(
    AzureNames,
    SequencePattern::default()
        .t_aco("Azure")
        .then_whitespace()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::aco("DevOps")),
            Box::new(SequencePattern::aco("Functions")),
            Box::new(
                SequencePattern::default()
                    .t_aco("Cosmos")
                    .then_whitespace()
                    .t_aco("DB")
            ),
            Box::new(
                SequencePattern::default()
                    .t_aco("SQL")
                    .then_whitespace()
                    .t_aco("Database")
            ),
            Box::new(
                SequencePattern::default()
                    .t_aco("Kubernetes")
                    .then_whitespace()
                    .t_aco("Service")
            ),
            Box::new(
                SequencePattern::default()
                    .t_aco("Virtual")
                    .then_whitespace()
                    .t_aco("Machines")
            ),
            Box::new(SequencePattern::aco("Monitor")),
            Box::new(SequencePattern::aco("Storage")),
            Box::new(
                SequencePattern::default()
                    .t_aco("Active")
                    .then_whitespace()
                    .t_aco("Directory")
            ),
            Box::new(
                SequencePattern::default()
                    .t_aco("App")
                    .then_whitespace()
                    .t_aco("Service")
            ),
            Box::new(
                SequencePattern::default()
                    .t_aco("Key")
                    .then_whitespace()
                    .t_aco("Vault")
            ),
            Box::new(
                SequencePattern::default()
                    .t_aco("Cognitive")
                    .then_whitespace()
                    .t_aco("Services")
            ),
            Box::new(
                SequencePattern::default()
                    .t_aco("Service")
                    .then_whitespace()
                    .t_aco("Bus")
            ),
            Box::new(
                SequencePattern::default()
                    .t_aco("Event")
                    .then_whitespace()
                    .t_aco("Hub")
            )
        ]))),
    "When referring to Azure cloud services, make sure to treat them as proper nouns."
);

create_linter_for!(
    MicrosoftNames,
    SequencePattern::default()
        .t_aco("Microsoft")
        .then_whitespace()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::aco("Windows")),
            Box::new(SequencePattern::aco("Office")),
            Box::new(SequencePattern::aco("Teams")),
            Box::new(SequencePattern::aco("Excel")),
            Box::new(SequencePattern::aco("PowerPoint")),
            Box::new(SequencePattern::aco("Word")),
            Box::new(SequencePattern::aco("Outlook")),
            Box::new(SequencePattern::aco("OneDrive")),
            Box::new(SequencePattern::aco("SharePoint")),
            Box::new(SequencePattern::aco("Xbox")),
            Box::new(SequencePattern::aco("Surface")),
            Box::new(SequencePattern::aco("Edge")),
            Box::new(SequencePattern::aco("Bing")),
            Box::new(SequencePattern::aco("Dynamics")),
            Box::new(
                SequencePattern::default()
                    .t_aco("Visual")
                    .then_whitespace()
                    .t_aco("Studio")
            )
        ]))),
    "When referring to Microsoft products and services, make sure to treat them as proper nouns."
);

create_linter_for!(
    AppleNames,
    SequencePattern::default()
        .t_aco("Apple")
        .then_whitespace()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::aco("iPhone")),
            Box::new(SequencePattern::aco("iPad")),
            Box::new(SequencePattern::aco("iMac")),
            Box::new(SequencePattern::aco("MacBook")),
            Box::new(
                SequencePattern::aco("MacBook")
                    .then_whitespace()
                    .t_aco("Pro")
            ),
            Box::new(
                SequencePattern::aco("MacBook")
                    .then_whitespace()
                    .t_aco("Air")
            ),
            Box::new(SequencePattern::aco("Mac").then_whitespace().t_aco("Pro")),
            Box::new(SequencePattern::aco("Mac").then_whitespace().t_aco("Mini")),
            Box::new(SequencePattern::aco("AirPods")),
            Box::new(
                SequencePattern::aco("AirPods")
                    .then_whitespace()
                    .t_aco("Pro")
            ),
            Box::new(
                SequencePattern::aco("AirPods")
                    .then_whitespace()
                    .t_aco("Max")
            ),
            Box::new(SequencePattern::aco("Watch")),
            Box::new(SequencePattern::aco("TV")),
            Box::new(SequencePattern::aco("Music")),
            Box::new(SequencePattern::aco("Arcade")),
            Box::new(SequencePattern::aco("iCloud")),
            Box::new(SequencePattern::aco("Safari")),
            Box::new(SequencePattern::aco("HomeKit")),
            Box::new(SequencePattern::aco("CarPlay")),
            Box::new(
                SequencePattern::default()
                    .t_aco("Vision")
                    .then_whitespace()
                    .t_aco("Pro")
            )
        ]))),
    "When referring to Apple products and services, make sure to treat them as proper nouns."
);

create_linter_for!(
    MetaNames,
    SequencePattern::aco("Meta")
        .then_whitespace()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::aco("Oculus")),
            Box::new(SequencePattern::aco("Portals")),
            Box::new(SequencePattern::aco("Quest")),
            Box::new(SequencePattern::aco("Gaming")),
            Box::new(SequencePattern::aco("Horizon")),
            Box::new(
                SequencePattern::default()
                    .t_aco("Reality")
                    .then_whitespace()
                    .t_aco("Labs")
            ),
        ]))),
    "When referring to Meta products and services, make sure to treat them as proper nouns."
);

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    use super::{Americas, MetaNames, MicrosoftNames, UnitedOrganizations};

    #[test]
    fn americas_lowercase() {
        assert_suggestion_result("south america", Americas::default(), "South America");
        assert_suggestion_result("north america", Americas::default(), "North America");
    }

    #[test]
    fn americas_uppercase() {
        assert_suggestion_result("SOUTH AMERICA", Americas::default(), "South America");
        assert_suggestion_result("NORTH AMERICA", Americas::default(), "North America");
    }

    #[test]
    fn americas_allow_correct() {
        assert_lint_count("South America", Americas::default(), 0);
        assert_lint_count("North America", Americas::default(), 0);
    }

    #[test]
    fn united_nations_uppercase() {
        assert_suggestion_result(
            "UNITED NATIONS",
            UnitedOrganizations::default(),
            "United Nations",
        );
    }

    #[test]
    fn united_arab_emirates_lowercase() {
        assert_suggestion_result(
            "UNITED ARAB EMIRATES",
            UnitedOrganizations::default(),
            "United Arab Emirates",
        );
    }

    #[test]
    fn united_nations_allow_correct() {
        assert_lint_count("United Nations", UnitedOrganizations::default(), 0);
    }

    #[test]
    fn meta_allow_correct() {
        assert_lint_count("Meta Quest", MetaNames::default(), 0);
    }

    #[test]
    fn microsoft_lowercase() {
        assert_suggestion_result(
            "microsoft visual studio",
            MicrosoftNames::default(),
            "Microsoft Visual Studio",
        );
    }

    #[test]
    fn microsoft_first_word_is_correct() {
        assert_suggestion_result(
            "Microsoft visual studio",
            MicrosoftNames::default(),
            "Microsoft Visual Studio",
        );
    }
}