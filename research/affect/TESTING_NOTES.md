# Validation log interpretation

The final full-suite evidence is `tests.log` (225 passing tests); `tests-social-final.log` covers the original15 social tests. `cost-tests.log` adds the later research-only cost audit test. Do not sum repeated targeted executions as different tests.

`clippy-final.log` validates the primary implementation. `clippy-verified.log` validates the final cost-harness revision and all targets; formatting passed in the same command. `build.log` builds the primary probes and `cost-build.log` rebuilds the final cost probe. Early lint/compile logs are preserved rather than overwritten. Their ordinary type/format/style errors were corrected before the affected dispatch. The final cost lint correction changes only a boolean test assertion's idiom, not simulation behavior.

The offline analysis initially excluded a nonexistent `SocialDeathLearned` label rather than the actual `DeathObserved` observer record. An exact-trace assertion identified this; inspecting the diff showed only the newly recorded local death observation. The filter was corrected to the real event type. No simulation rule, output, data or decision was altered. The final analyzer requires exact physical/action traces after removing the four documented new observer kinds and the appended social score from explanations.

No experiment output was replaced, no historical expected result changed, and no behavioral coefficient was tuned after the primary results. Cost audit was registered separately after the primary matrix, explicitly to test the physical meaning of the scarcity proxy.
