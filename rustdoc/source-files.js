var sourcesIndex = JSON.parse('{\
"ahash":["",[],["convert.rs","fallback_hash.rs","hash_map.rs","hash_set.rs","lib.rs","operations.rs","random_state.rs","specialize.rs"]],\
"aoc":["",[],["lib.rs"]],\
"aoc_2015":["",[],["lib.rs"]],\
"aoc_2015_day_01":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_02":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_03":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_04":["",[],["cli.rs","input.rs","lib.rs","logic.rs"]],\
"aoc_2015_day_05":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2015_day_06":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_07":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_08":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2015_day_09":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_10":["",[],["cli.rs","cycles.rs","input.rs","lib.rs","logic.rs","model.rs","tracking.rs"]],\
"aoc_2015_day_11":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2015_day_12":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_13":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_14":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_15":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_16":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_17":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2015_day_18":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_19":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2015_day_20":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2015_day_21":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_22":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_23":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2015_day_24":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2015_day_25":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016":["",[],["lib.rs"]],\
"aoc_2016_cpu":["",[],["cpu.rs","instr.rs","lib.rs"]],\
"aoc_2016_day_01":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2016_day_02":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2016_day_03":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2016_day_04":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2016_day_05":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_06":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_07":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_08":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2016_day_09":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_10":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2016_day_11":["",[],["input.rs","lib.rs","logic.rs","model.rs","tools.rs"]],\
"aoc_2016_day_12":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_13":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2016_day_14":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_15":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_16":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_17":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_18":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_19":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_20":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_21":["",[],["input.rs","lib.rs","logic.rs","ops.rs"]],\
"aoc_2016_day_22":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_23":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2016_day_24":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2016_day_25":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017":["",[],["lib.rs"]],\
"aoc_2017_cpu":["",[],["cpu.rs"]],\
"aoc_2017_day_01":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_02":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_03":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_04":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_05":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_06":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_07":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2017_day_08":["",[],["cpu.rs","input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_09":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_10":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_11":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2017_day_12":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2017_day_13":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_14":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_15":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_16":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2017_day_17":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_18":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_19":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2017_day_20":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2017_day_21":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2017_day_22":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2017_day_23":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_24":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_day_25":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2017_knot":["",[],["knot.rs"]],\
"aoc_2018":["",[],["lib.rs"]],\
"aoc_2018_cpu":["",[],["cpu.rs"]],\
"aoc_2018_day_01":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2018_day_02":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2018_day_03":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_04":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_05":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2018_day_06":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2018_day_07":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2018_day_08":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2018_day_09":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2018_day_10":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_11":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2018_day_12":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_13":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_14":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2018_day_15":["",[],["input.rs","lib.rs","logic.rs","model.rs","state.rs"]],\
"aoc_2018_day_16":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_17":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_18":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_19":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_20":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_21":["",[],["analyser.rs","input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_22":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_23":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_24":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2018_day_25":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019":["",[],["lib.rs"]],\
"aoc_2019_day_01":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2019_day_02":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2019_day_03":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_04":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2019_day_05":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_06":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_07":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_08":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_09":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_10":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_11":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_12":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_13":["",[],["input.rs","lib.rs","logic.rs","model.rs","run.rs"]],\
"aoc_2019_day_14":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_15":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_16":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2019_day_17":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_18":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_19":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_20":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_21":["",[],["emul.rs","input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_22":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_23":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2019_day_24":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2019_day_25":["",[],["game.rs","input.rs","lib.rs","logic.rs","model.rs","run.rs"]],\
"aoc_2019_intcode":["",[],["intcode.rs"]],\
"aoc_2020":["",[],["lib.rs"]],\
"aoc_2020_day_01":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2020_day_02":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_03":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_04":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_05":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_06":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2020_day_07":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2020_day_08":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_09":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2020_day_10":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2020_day_11":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_12":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_13":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2020_day_14":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_15":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2020_day_16":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_17":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_18":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2020_day_19":["",[],["input.rs","lib.rs","logic.rs","matcher.rs","model.rs"]],\
"aoc_2020_day_20":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_21":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_22":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_23":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_24":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2020_day_25":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2021":["",[],["lib.rs"]],\
"aoc_2021_day_01":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2021_day_02":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2021_day_03":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2021_day_04":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2021_day_05":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2021_day_06":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2021_day_07":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2021_day_08":["",[],["input.rs","lib.rs","logic.rs","model.rs","solver.rs"]],\
"aoc_2021_day_09":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2021_day_10":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2021_day_11":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2021_day_12":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2021_day_13":["",[],["input.rs","lib.rs","logic.rs","model.rs","tool.rs"]],\
"aoc_2021_day_14":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2021_day_15":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2021_day_16":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2021_day_17":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2021_day_18":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2021_day_19":["",[],["input.rs","lib.rs","logic.rs","model.rs","rotation.rs"]],\
"aoc_2021_day_20":["",[],["input.rs","lib.rs","logic.rs","model.rs","tool.rs"]],\
"aoc_2021_day_21":["",[],["input.rs","lib.rs","logic.rs"]],\
"aoc_2021_day_22":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_2021_day_23":["",[],["input.rs","lib.rs","logic.rs","model.rs","tools.rs"]],\
"aoc_2021_day_24":["",[],["input.rs","lib.rs","logic.rs","machine.rs","model.rs","quick.rs","solver.rs","tool.rs"]],\
"aoc_2021_day_25":["",[],["input.rs","lib.rs","logic.rs","model.rs"]],\
"aoc_bithash":["",[],["bithash.rs"]],\
"aoc_bitvec":["",[],["bitvec.rs","encode.rs","iter.rs","lib.rs"]],\
"aoc_checked":["",[],["checked.rs"]],\
"aoc_codegen":["",[],["lib.rs"]],\
"aoc_common":["",[],["lib.rs","puzzle.rs"]],\
"aoc_grid":["",[],["buf.rs","cursor.rs","display.rs","extend.rs","grid.rs","iter.rs","parse.rs","pos.rs","storage.rs","transform.rs","view.rs"]],\
"aoc_inpstr":["",[],["inpstr.rs"]],\
"aoc_list":["",[],["list.rs"]],\
"aoc_md5":["",[],["md5.rs"]],\
"aoc_misc":["",[],["collections.rs","iter.rs","misc.rs","prelude.rs"]],\
"aoc_nums":["",[],["bits.rs","conv.rs","int.rs","iter.rs","nums.rs","ops.rs"]],\
"aoc_ocr":["",[],["ocr.rs"]],\
"aoc_parallel":["",[],["parallel.rs"]],\
"aoc_parser":["",[],["delim.rs","display.rs","enums.rs","from_parser.rs","parse.rs","parser.rs","structs.rs"]],\
"aoc_pos":["",[],["pos.rs"]],\
"aoc_search":["",[],["pairs_map.rs","permutations.rs","priority.rs","search.rs"]],\
"arrayvec":["",[],["array_string.rs","arrayvec.rs","arrayvec_impl.rs","char.rs","errors.rs","lib.rs","utils.rs"]],\
"atty":["",[],["lib.rs"]],\
"bitflags":["",[],["lib.rs"]],\
"cfg_if":["",[],["lib.rs"]],\
"clap":["",[["builder",[],["action.rs","app_settings.rs","arg.rs","arg_group.rs","arg_predicate.rs","arg_settings.rs","command.rs","debug_asserts.rs","macros.rs","mod.rs","possible_value.rs","usage_parser.rs","value_hint.rs","value_parser.rs"]],["error",[],["context.rs","kind.rs","mod.rs"]],["output",[],["fmt.rs","help.rs","mod.rs","usage.rs"]],["parser",[["features",[],["mod.rs","suggestions.rs"]],["matches",[],["any_value.rs","arg_matches.rs","matched_arg.rs","mod.rs","value_source.rs"]]],["arg_matcher.rs","error.rs","mod.rs","parser.rs","validator.rs"]],["util",[],["color.rs","fnv.rs","graph.rs","id.rs","mod.rs","str_to_bool.rs"]]],["derive.rs","lib.rs","macros.rs","mkeymap.rs"]],\
"clap_derive":["",[["derives",[],["args.rs","into_app.rs","mod.rs","parser.rs","subcommand.rs","value_enum.rs"]],["utils",[],["doc_comments.rs","mod.rs","spanned.rs","ty.rs"]]],["attrs.rs","dummies.rs","lib.rs","parse.rs"]],\
"clap_lex":["",[],["lib.rs"]],\
"either":["",[],["lib.rs"]],\
"getrandom":["",[],["error.rs","lib.rs","linux_android.rs","use_file.rs","util.rs","util_libc.rs"]],\
"hashbrown":["",[["external_trait_impls",[],["mod.rs"]],["raw",[],["alloc.rs","bitmask.rs","mod.rs","sse2.rs"]]],["lib.rs","macros.rs","map.rs","scopeguard.rs","set.rs"]],\
"heck":["",[],["kebab.rs","lib.rs","lower_camel.rs","shouty_kebab.rs","shouty_snake.rs","snake.rs","title.rs","upper_camel.rs"]],\
"indexmap":["",[["map",[["core",[],["raw.rs"]]],["core.rs"]]],["equivalent.rs","lib.rs","macros.rs","map.rs","mutable_keys.rs","set.rs","util.rs"]],\
"itertools":["",[["adaptors",[],["coalesce.rs","map.rs","mod.rs","multi_product.rs"]]],["combinations.rs","combinations_with_replacement.rs","concat_impl.rs","cons_tuples_impl.rs","diff.rs","duplicates_impl.rs","either_or_both.rs","exactly_one_err.rs","extrema_set.rs","flatten_ok.rs","format.rs","free.rs","group_map.rs","groupbylazy.rs","grouping_map.rs","impl_macros.rs","intersperse.rs","k_smallest.rs","kmerge_impl.rs","lazy_buffer.rs","lib.rs","merge_join.rs","minmax.rs","multipeek_impl.rs","pad_tail.rs","peek_nth.rs","peeking_take_while.rs","permutations.rs","powerset.rs","process_results_impl.rs","put_back_n_impl.rs","rciter_impl.rs","repeatn.rs","size_hint.rs","sources.rs","tee.rs","tuple_impl.rs","unique_impl.rs","unziptuple.rs","with_position.rs","zip_eq_impl.rs","zip_longest.rs","ziptuple.rs"]],\
"libc":["",[["unix",[["linux_like",[["linux",[["arch",[["generic",[],["mod.rs"]]],["mod.rs"]],["gnu",[["b64",[["x86_64",[],["align.rs","mod.rs","not_x32.rs"]]],["mod.rs"]]],["align.rs","mod.rs"]]],["align.rs","mod.rs","non_exhaustive.rs"]]],["mod.rs"]]],["align.rs","mod.rs"]]],["fixed_width_ints.rs","lib.rs","macros.rs"]],\
"once_cell":["",[],["imp_std.rs","lib.rs","race.rs"]],\
"os_str_bytes":["",[["common",[],["mod.rs","raw.rs"]]],["iter.rs","lib.rs","pattern.rs","raw_str.rs"]],\
"proc_macro2":["",[],["detection.rs","fallback.rs","lib.rs","marker.rs","parse.rs","rcvec.rs","wrapper.rs"]],\
"proc_macro_error":["",[["imp",[],["delegate.rs"]]],["diagnostic.rs","dummy.rs","lib.rs","macros.rs","sealed.rs"]],\
"proc_macro_error_attr":["",[],["lib.rs","parse.rs","settings.rs"]],\
"quote":["",[],["ext.rs","format.rs","ident_fragment.rs","lib.rs","runtime.rs","spanned.rs","to_tokens.rs"]],\
"strsim":["",[],["lib.rs"]],\
"syn":["",[["gen",[],["clone.rs","gen_helper.rs"]]],["attr.rs","await.rs","bigint.rs","buffer.rs","custom_keyword.rs","custom_punctuation.rs","data.rs","derive.rs","discouraged.rs","error.rs","export.rs","expr.rs","ext.rs","file.rs","generics.rs","group.rs","ident.rs","item.rs","lib.rs","lifetime.rs","lit.rs","lookahead.rs","mac.rs","macros.rs","op.rs","parse.rs","parse_macro_input.rs","parse_quote.rs","pat.rs","path.rs","print.rs","punctuated.rs","reserved.rs","sealed.rs","span.rs","spanned.rs","stmt.rs","thread.rs","token.rs","ty.rs","verbatim.rs","whitespace.rs"]],\
"termcolor":["",[],["lib.rs"]],\
"textwrap":["",[],["core.rs","indentation.rs","lib.rs","line_ending.rs","word_separators.rs","word_splitters.rs","wrap_algorithms.rs"]],\
"unicode_ident":["",[],["lib.rs","tables.rs"]]\
}');
createSourceSidebar();
