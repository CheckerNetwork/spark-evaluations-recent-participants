pub use recent_participants::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod recent_participants {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_owner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("daysRing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("daysRing"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("get"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("set"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("set"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("date"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("participants"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static RECENTPARTICIPANTS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x08\x188\x03\x80a\x08\x18\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\xA0V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0`WPP\x80Qa\0\x88\x91`\0\x91` \x90\x91\x01\x90a\0\x8FV[PPa\x01\xD0V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\0\xDCW\x91` \x02\x82\x01[\x82\x81\x11\x15a\0\xDCW\x82Q\x80Qa\0\xCC\x91\x84\x91` \x90\x91\x01\x90a\0\xECV[P\x91` \x01\x91\x90`\x01\x01\x90a\0\xAFV[Pa\0\xE8\x92\x91Pa\x01MV[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x01AW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x01AW\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x01\x0CV[Pa\0\xE8\x92\x91Pa\x01jV[\x80\x82\x11\x15a\0\xE8W`\0a\x01a\x82\x82a\x01\x7FV[P`\x01\x01a\x01MV[[\x80\x82\x11\x15a\0\xE8W`\0\x81U`\x01\x01a\x01kV[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x01\x9D\x91\x90a\x01jV[PV[`\0` \x82\x84\x03\x12\x15a\x01\xB2W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xC9W`\0\x80\xFD[\x93\x92PPPV[a\x069\x80a\x01\xDF`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cmL\xE6<\x14a\0FW\x80c\x84lY\xE4\x14a\0dW\x80c\xDA\xA79\x04\x14a\0yW[`\0\x80\xFD[a\0Na\0\xB1V[`@Qa\0[\x91\x90a\x03\xEFV[`@Q\x80\x91\x03\x90\xF3[a\0wa\0r6`\x04a\x04IV[a\x02=V[\0[a\0\x8Ca\0\x876`\x04a\x04\xC8V[a\x03\x01V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0[V[```\0\x80[`\0T\x81\x10\x15a\0\xFEW`\0\x81\x81T\x81\x10a\0\xD4Wa\0\xD4a\x04\xEAV[`\0\x91\x82R` \x90\x91 \x01Ta\0\xEA\x90\x83a\x05HV[\x91P\x80a\0\xF6\x81a\x05aV[\x91PPa\0\xB7V[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x1AWa\x01\x1Aa\x05\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01CW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[`\0T\x81\x10\x15a\x024W`\0[`\0\x82\x81T\x81\x10a\x01jWa\x01ja\x04\xEAV[`\0\x91\x82R` \x90\x91 \x01T\x81\x10\x15a\x02!W`\0\x82\x81T\x81\x10a\x01\x90Wa\x01\x90a\x04\xEAV[\x90`\0R` `\0 \x01\x81\x81T\x81\x10a\x01\xABWa\x01\xABa\x04\xEAV[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x84a\x01\xD8\x81a\x05aV[\x95P\x81Q\x81\x10a\x01\xEAWa\x01\xEAa\x04\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x02\x19\x81a\x05aV[\x91PPa\x01WV[P\x80a\x02,\x81a\x05aV[\x91PPa\x01JV[P\x90\x93\x92PPPV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x02\xC2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUnauthorized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x83\x91\x83\x91a\x02\xD4\x90\x87a\x05\xC8V[\x81T\x81\x10a\x02\xE4Wa\x02\xE4a\x04\xEAV[\x90`\0R` `\0 \x01\x91\x90a\x02\xFB\x92\x91\x90a\x03RV[PPPPV[`\0\x82\x81T\x81\x10a\x03\x11W`\0\x80\xFD[\x90`\0R` `\0 \x01\x81\x81T\x81\x10a\x03)W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P\x82\x90PV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x03\xCAW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x03\xCAW\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x845\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x03rV[Pa\x03\xD6\x92\x91Pa\x03\xDAV[P\x90V[[\x80\x82\x11\x15a\x03\xD6W`\0\x81U`\x01\x01a\x03\xDBV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x04=W\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x04\x0BV[P\x90\x96\x95PPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x04^W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04}W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x04\x91W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04\xA0W`\0\x80\xFD[\x87` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x04\xB5W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x04\xDBW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x05[Wa\x05[a\x05\x19V[\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x05\x92Wa\x05\x92a\x05\x19V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82a\x05\xFEW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V\xFE\xA2dipfsX\"\x12 \xE6M4\xF3\xFE\xDD\x15rZ\x8D\xB7\x0Ft\xA8\x89\xC5\xB0\x8D\xF5\x837r\xA8\x9EN+\x87\x0F\xE7(\"\xD9dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static RECENTPARTICIPANTS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cmL\xE6<\x14a\0FW\x80c\x84lY\xE4\x14a\0dW\x80c\xDA\xA79\x04\x14a\0yW[`\0\x80\xFD[a\0Na\0\xB1V[`@Qa\0[\x91\x90a\x03\xEFV[`@Q\x80\x91\x03\x90\xF3[a\0wa\0r6`\x04a\x04IV[a\x02=V[\0[a\0\x8Ca\0\x876`\x04a\x04\xC8V[a\x03\x01V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0[V[```\0\x80[`\0T\x81\x10\x15a\0\xFEW`\0\x81\x81T\x81\x10a\0\xD4Wa\0\xD4a\x04\xEAV[`\0\x91\x82R` \x90\x91 \x01Ta\0\xEA\x90\x83a\x05HV[\x91P\x80a\0\xF6\x81a\x05aV[\x91PPa\0\xB7V[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x1AWa\x01\x1Aa\x05\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01CW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[`\0T\x81\x10\x15a\x024W`\0[`\0\x82\x81T\x81\x10a\x01jWa\x01ja\x04\xEAV[`\0\x91\x82R` \x90\x91 \x01T\x81\x10\x15a\x02!W`\0\x82\x81T\x81\x10a\x01\x90Wa\x01\x90a\x04\xEAV[\x90`\0R` `\0 \x01\x81\x81T\x81\x10a\x01\xABWa\x01\xABa\x04\xEAV[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x84a\x01\xD8\x81a\x05aV[\x95P\x81Q\x81\x10a\x01\xEAWa\x01\xEAa\x04\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x02\x19\x81a\x05aV[\x91PPa\x01WV[P\x80a\x02,\x81a\x05aV[\x91PPa\x01JV[P\x90\x93\x92PPPV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x02\xC2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUnauthorized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x83\x91\x83\x91a\x02\xD4\x90\x87a\x05\xC8V[\x81T\x81\x10a\x02\xE4Wa\x02\xE4a\x04\xEAV[\x90`\0R` `\0 \x01\x91\x90a\x02\xFB\x92\x91\x90a\x03RV[PPPPV[`\0\x82\x81T\x81\x10a\x03\x11W`\0\x80\xFD[\x90`\0R` `\0 \x01\x81\x81T\x81\x10a\x03)W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P\x82\x90PV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x03\xCAW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x03\xCAW\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x845\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x03rV[Pa\x03\xD6\x92\x91Pa\x03\xDAV[P\x90V[[\x80\x82\x11\x15a\x03\xD6W`\0\x81U`\x01\x01a\x03\xDBV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x04=W\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x04\x0BV[P\x90\x96\x95PPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x04^W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04}W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x04\x91W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04\xA0W`\0\x80\xFD[\x87` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x04\xB5W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x04\xDBW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x05[Wa\x05[a\x05\x19V[\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x05\x92Wa\x05\x92a\x05\x19V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82a\x05\xFEW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V\xFE\xA2dipfsX\"\x12 \xE6M4\xF3\xFE\xDD\x15rZ\x8D\xB7\x0Ft\xA8\x89\xC5\xB0\x8D\xF5\x837r\xA8\x9EN+\x87\x0F\xE7(\"\xD9dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static RECENTPARTICIPANTS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RecentParticipants<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RecentParticipants<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RecentParticipants<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RecentParticipants<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RecentParticipants<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RecentParticipants))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RecentParticipants<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RECENTPARTICIPANTS_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                RECENTPARTICIPANTS_ABI.clone(),
                RECENTPARTICIPANTS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `daysRing` (0xdaa73904) function
        pub fn days_ring(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([218, 167, 57, 4], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get` (0x6d4ce63c) function
        pub fn get(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([109, 76, 230, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `set` (0x846c59e4) function
        pub fn set(
            &self,
            date: ::ethers::core::types::U256,
            participants: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 108, 89, 228], (date, participants))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RecentParticipants<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `daysRing` function with signature `daysRing(uint256,uint256)` and selector `0xdaa73904`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "daysRing", abi = "daysRing(uint256,uint256)")]
    pub struct DaysRingCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `get` function with signature `get()` and selector `0x6d4ce63c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "get", abi = "get()")]
    pub struct GetCall;
    ///Container type for all input parameters for the `set` function with signature `set(uint256,address[])` and selector `0x846c59e4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "set", abi = "set(uint256,address[])")]
    pub struct SetCall {
        pub date: ::ethers::core::types::U256,
        pub participants: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum RecentParticipantsCalls {
        DaysRing(DaysRingCall),
        Get(GetCall),
        Set(SetCall),
    }
    impl ::ethers::core::abi::AbiDecode for RecentParticipantsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DaysRingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DaysRing(decoded));
            }
            if let Ok(decoded)
                = <GetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Get(decoded));
            }
            if let Ok(decoded)
                = <SetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Set(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RecentParticipantsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DaysRing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Get(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Set(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for RecentParticipantsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DaysRing(element) => ::core::fmt::Display::fmt(element, f),
                Self::Get(element) => ::core::fmt::Display::fmt(element, f),
                Self::Set(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DaysRingCall> for RecentParticipantsCalls {
        fn from(value: DaysRingCall) -> Self {
            Self::DaysRing(value)
        }
    }
    impl ::core::convert::From<GetCall> for RecentParticipantsCalls {
        fn from(value: GetCall) -> Self {
            Self::Get(value)
        }
    }
    impl ::core::convert::From<SetCall> for RecentParticipantsCalls {
        fn from(value: SetCall) -> Self {
            Self::Set(value)
        }
    }
    ///Container type for all return fields from the `daysRing` function with signature `daysRing(uint256,uint256)` and selector `0xdaa73904`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DaysRingReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `get` function with signature `get()` and selector `0x6d4ce63c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
}
