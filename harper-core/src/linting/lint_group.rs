use paste::paste;
use serde::{Deserialize, Serialize};

use super::an_a::AnA;
use super::avoid_curses::AvoidCurses;
use super::boring_words::BoringWords;
use super::capitalize_personal_pronouns::CapitalizePersonalPronouns;
use super::correct_number_suffix::CorrectNumberSuffix;
use super::dot_initialisms::DotInitialisms;
use super::ellipsis_length::EllipsisLength;
use super::linking_verbs::LinkingVerbs;
use super::long_sentences::LongSentences;
use super::matcher::Matcher;
use super::multiple_sequential_pronouns::MultipleSequentialPronouns;
use super::number_suffix_capitalization::NumberSuffixCapitalization;
use super::proper_noun_capitalization_linters::{
    AmazonNames, Americas, AppleNames, AzureNames, ChineseCommunistParty, GoogleNames, Holidays,
    MetaNames, MicrosoftNames, UnitedOrganizations,
};
use super::repeated_words::RepeatedWords;
use super::sentence_capitalization::SentenceCapitalization;
use super::spaces::Spaces;
use super::spell_check::SpellCheck;
use super::spelled_numbers::SpelledNumbers;
use super::terminating_conjunctions::TerminatingConjunctions;
use super::that_which::ThatWhich;
use super::unclosed_quotes::UnclosedQuotes;
use super::use_genitive::UseGenitive;
use super::wrong_quotes::WrongQuotes;
use super::{Lint, Linter};
use crate::{Dictionary, Document};

macro_rules! create_lint_group_config {
    ($($linter:ident => $default:expr),*) => {
        paste! {
            /// A collection of all the descriptions from the composing linters.
            #[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
            pub struct LintGroupDescriptions<'a> {
                $(
                    #[doc = "The description for the [`" $linter "`] linter."]
                    pub [<$linter:snake>]: &'a str,
                )*
                pub spell_check: &'a str
            }

            #[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
            pub struct LintGroupConfig {
                $(
                    #[doc = "Configures the use of the [`" $linter "`] linter.
                    If set to [`None`], the default configuration will be used."]
                    pub [<$linter:snake>]: Option<bool>,
                )*
                pub spell_check: Option<bool>
            }

            impl LintGroupConfig {
                /// Creates a config with all lints disabled.
                pub fn none() -> Self{
                    Self {
                        $(
                            [<$linter:snake>]: Some(false),
                        )*
                        spell_check: Some(false)
                    }
                }

                /// Fills the [`None`] values in the configuration with the default values.
                pub fn fill_default_values(&mut self){
                    $(
                        if self.[<$linter:snake>].is_none() {
                            self.[<$linter:snake>] = Some($default);
                        }
                    )*

                    if self.spell_check.is_none() {
                        self.spell_check = Some(true);
                    }
                }
            }

            /// A wrapper that combines all built-in Harper linters
            /// into a single, configurable [`Linter`].
            pub struct LintGroup<T: Dictionary> {
                $(
                    [<$linter:snake>]: $linter,
                )*
                spell_check: SpellCheck<T>,
                pub config: LintGroupConfig
            }


            impl<T: Dictionary> LintGroup<T> {
                pub fn new(config: LintGroupConfig, dictionary: T) -> Self {
                    Self {
                        $(
                            [<$linter:snake>]: $linter::default(),
                        )*
                        spell_check: SpellCheck::new(dictionary),
                        config,
                    }
                }

                pub fn all_descriptions(&self) -> LintGroupDescriptions<'_> {
                    LintGroupDescriptions {
                        $(
                            [<$linter:snake>]: self.[<$linter:snake>].description(),
                        )*
                        spell_check: self.spell_check.description(),
                    }
                }
            }

            impl<T: Dictionary> Linter for LintGroup<T> {
                fn lint(&mut self, document: &Document) -> Vec<Lint> {
                    let mut lints = Vec::new();

                    let mut config = self.config.clone();
                    config.fill_default_values();

                    $(
                        if config.[<$linter:snake>].unwrap() {
                            lints.append(&mut self.[<$linter:snake>].lint(document));
                        }
                    )*

                    if config.spell_check.unwrap() {
                        lints.append(&mut self.spell_check.lint(document));
                    }


                    lints
                }

                fn description(&self) -> &'static str {
                    "A collection of linters that can be run as one."
                }
            }
        }
    };
}

create_lint_group_config!(
    SpelledNumbers => false,
    AnA => true,
    SentenceCapitalization => true,
    UnclosedQuotes => true,
    WrongQuotes => false,
    LongSentences => true,
    RepeatedWords => true,
    Spaces => true,
    Matcher => true,
    CorrectNumberSuffix => true,
    NumberSuffixCapitalization => true,
    MultipleSequentialPronouns => true,
    LinkingVerbs => false,
    AvoidCurses => true,
    TerminatingConjunctions => true,
    EllipsisLength => true,
    DotInitialisms => true,
    BoringWords => false,
    UseGenitive => false,
    ThatWhich => true,
    CapitalizePersonalPronouns => true,
    Americas => true,
    ChineseCommunistParty => true,
    UnitedOrganizations => true,
    Holidays => true,
    AmazonNames => true,
    GoogleNames => true,
    MetaNames => true,
    MicrosoftNames => true,
    AppleNames => true,
    AzureNames => true
);

impl<T: Dictionary + Default> Default for LintGroup<T> {
    fn default() -> Self {
        Self::new(LintGroupConfig::default(), T::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::FullDictionary;

    use super::LintGroup;

    #[test]
    fn can_get_all_descriptions() {
        let group = LintGroup::<FullDictionary>::default();
        group.all_descriptions();
    }
}
