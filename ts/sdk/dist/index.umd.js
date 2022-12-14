/*!
 * icns-sdk v 0.0.1
 * (c) mattverse <mattpark1028@gmail.com>
 * Released under the MIT OR Apache-2.0 License.
 */

(function (global, factory) {
    typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports) :
    typeof define === 'function' && define.amd ? define(['exports'], factory) :
    (global = typeof globalThis !== 'undefined' ? globalThis : global || self, factory(global["counter-sdk"] = {}));
})(this, (function (exports) { 'use strict';

    /******************************************************************************
    Copyright (c) Microsoft Corporation.

    Permission to use, copy, modify, and/or distribute this software for any
    purpose with or without fee is hereby granted.

    THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
    REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
    AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
    INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
    LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
    OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
    PERFORMANCE OF THIS SOFTWARE.
    ***************************************************************************** */
    /* global Reflect, Promise */

    var extendStatics = function(d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };

    function __extends(d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    }

    var __assign = function() {
        __assign = Object.assign || function __assign(t) {
            for (var s, i = 1, n = arguments.length; i < n; i++) {
                s = arguments[i];
                for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p)) t[p] = s[p];
            }
            return t;
        };
        return __assign.apply(this, arguments);
    };

    function __awaiter(thisArg, _arguments, P, generator) {
        function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
        return new (P || (P = Promise))(function (resolve, reject) {
            function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
            function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
            function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
            step((generator = generator.apply(thisArg, _arguments || [])).next());
        });
    }

    function __generator(thisArg, body) {
        var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
        return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
        function verb(n) { return function (v) { return step([n, v]); }; }
        function step(op) {
            if (f) throw new TypeError("Generator is already executing.");
            while (g && (g = 0, op[0] && (_ = 0)), _) try {
                if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
                if (y = 0, t) op = [op[0] & 2, t.value];
                switch (op[0]) {
                    case 0: case 1: t = op; break;
                    case 4: _.label++; return { value: op[1], done: false };
                    case 5: _.label++; y = op[1]; op = [0]; continue;
                    case 7: op = _.ops.pop(); _.trys.pop(); continue;
                    default:
                        if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                        if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                        if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                        if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                        if (t[2]) _.ops.pop();
                        _.trys.pop(); continue;
                }
                op = body.call(thisArg, _);
            } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
            if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
        }
    }

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */

    var _0 = /*#__PURE__*/Object.freeze({
        __proto__: null
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var IcnsNameNftQueryClient = /** @class */ (function () {
        function IcnsNameNftQueryClient(client, contractAddress) {
            var _this = this;
            this.admin = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            admin: {}
                        })];
                });
            }); };
            this.isAdmin = function (_a) {
                var address = _a.address;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                is_admin: {
                                    address: address
                                }
                            })];
                    });
                });
            };
            this.transferrable = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            transferrable: {}
                        })];
                });
            }); };
            this.ownerOf = function (_a) {
                var includeExpired = _a.includeExpired, tokenId = _a.tokenId;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                owner_of: {
                                    include_expired: includeExpired,
                                    token_id: tokenId
                                }
                            })];
                    });
                });
            };
            this.approval = function (_a) {
                var includeExpired = _a.includeExpired, spender = _a.spender, tokenId = _a.tokenId;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                approval: {
                                    include_expired: includeExpired,
                                    spender: spender,
                                    token_id: tokenId
                                }
                            })];
                    });
                });
            };
            this.approvals = function (_a) {
                var includeExpired = _a.includeExpired, tokenId = _a.tokenId;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                approvals: {
                                    include_expired: includeExpired,
                                    token_id: tokenId
                                }
                            })];
                    });
                });
            };
            this.allOperators = function (_a) {
                var includeExpired = _a.includeExpired, limit = _a.limit, owner = _a.owner, startAfter = _a.startAfter;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                all_operators: {
                                    include_expired: includeExpired,
                                    limit: limit,
                                    owner: owner,
                                    start_after: startAfter
                                }
                            })];
                    });
                });
            };
            this.numTokens = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            num_tokens: {}
                        })];
                });
            }); };
            this.contractInfo = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            contract_info: {}
                        })];
                });
            }); };
            this.nftInfo = function (_a) {
                var tokenId = _a.tokenId;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                nft_info: {
                                    token_id: tokenId
                                }
                            })];
                    });
                });
            };
            this.allNftInfo = function (_a) {
                var includeExpired = _a.includeExpired, tokenId = _a.tokenId;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                all_nft_info: {
                                    include_expired: includeExpired,
                                    token_id: tokenId
                                }
                            })];
                    });
                });
            };
            this.tokens = function (_a) {
                var limit = _a.limit, owner = _a.owner, startAfter = _a.startAfter;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                tokens: {
                                    limit: limit,
                                    owner: owner,
                                    start_after: startAfter
                                }
                            })];
                    });
                });
            };
            this.allTokens = function (_a) {
                var limit = _a.limit, startAfter = _a.startAfter;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                all_tokens: {
                                    limit: limit,
                                    start_after: startAfter
                                }
                            })];
                    });
                });
            };
            this.minter = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            minter: {}
                        })];
                });
            }); };
            this.client = client;
            this.contractAddress = contractAddress;
            this.admin = this.admin.bind(this);
            this.isAdmin = this.isAdmin.bind(this);
            this.transferrable = this.transferrable.bind(this);
            this.ownerOf = this.ownerOf.bind(this);
            this.approval = this.approval.bind(this);
            this.approvals = this.approvals.bind(this);
            this.allOperators = this.allOperators.bind(this);
            this.numTokens = this.numTokens.bind(this);
            this.contractInfo = this.contractInfo.bind(this);
            this.nftInfo = this.nftInfo.bind(this);
            this.allNftInfo = this.allNftInfo.bind(this);
            this.tokens = this.tokens.bind(this);
            this.allTokens = this.allTokens.bind(this);
            this.minter = this.minter.bind(this);
        }
        return IcnsNameNftQueryClient;
    }());
    var IcnsNameNftClient = /** @class */ (function (_super) {
        __extends(IcnsNameNftClient, _super);
        function IcnsNameNftClient(client, sender, contractAddress) {
            var _this = _super.call(this, client, contractAddress) || this;
            _this.transferNft = function (_a, fee, memo, funds) {
                var recipient = _a.recipient, tokenId = _a.tokenId;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    transfer_nft: {
                                        recipient: recipient,
                                        token_id: tokenId
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.sendNft = function (_a, fee, memo, funds) {
                var contract = _a.contract, msg = _a.msg, tokenId = _a.tokenId;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    send_nft: {
                                        contract: contract,
                                        msg: msg,
                                        token_id: tokenId
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.approve = function (_a, fee, memo, funds) {
                var expires = _a.expires, spender = _a.spender, tokenId = _a.tokenId;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    approve: {
                                        expires: expires,
                                        spender: spender,
                                        token_id: tokenId
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.revoke = function (_a, fee, memo, funds) {
                var spender = _a.spender, tokenId = _a.tokenId;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    revoke: {
                                        spender: spender,
                                        token_id: tokenId
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.approveAll = function (_a, fee, memo, funds) {
                var expires = _a.expires, operator = _a.operator;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    approve_all: {
                                        expires: expires,
                                        operator: operator
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.revokeAll = function (_a, fee, memo, funds) {
                var operator = _a.operator;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    revoke_all: {
                                        operator: operator
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.mint = function (_a, fee, memo, funds) {
                var extension = _a.extension, owner = _a.owner, tokenId = _a.tokenId, tokenUri = _a.tokenUri;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    mint: {
                                        extension: extension,
                                        owner: owner,
                                        token_id: tokenId,
                                        token_uri: tokenUri
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.burn = function (_a, fee, memo, funds) {
                var tokenId = _a.tokenId;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    burn: {
                                        token_id: tokenId
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.extension = function (_a, fee, memo, funds) {
                var msg = _a.msg;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    extension: {
                                        msg: msg
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.client = client;
            _this.sender = sender;
            _this.contractAddress = contractAddress;
            _this.transferNft = _this.transferNft.bind(_this);
            _this.sendNft = _this.sendNft.bind(_this);
            _this.approve = _this.approve.bind(_this);
            _this.revoke = _this.revoke.bind(_this);
            _this.approveAll = _this.approveAll.bind(_this);
            _this.revokeAll = _this.revokeAll.bind(_this);
            _this.mint = _this.mint.bind(_this);
            _this.burn = _this.burn.bind(_this);
            _this.extension = _this.extension.bind(_this);
            return _this;
        }
        return IcnsNameNftClient;
    }(IcnsNameNftQueryClient));

    var _1 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        IcnsNameNftQueryClient: IcnsNameNftQueryClient,
        IcnsNameNftClient: IcnsNameNftClient
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */

    var _2 = /*#__PURE__*/Object.freeze({
        __proto__: null
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var IcnsRegistrarQueryClient = /** @class */ (function () {
        function IcnsRegistrarQueryClient(client, contractAddress) {
            var _this = this;
            this.verifierPubKeys = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            verifier_pub_keys: {}
                        })];
                });
            }); };
            this.verificationThreshold = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            verification_threshold: {}
                        })];
                });
            }); };
            this.nameNftAddress = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            name_nft_address: {}
                        })];
                });
            }); };
            this.referralCount = function (_a) {
                var name = _a.name;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                referral_count: {
                                    name: name
                                }
                            })];
                    });
                });
            };
            this.fee = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            fee: {}
                        })];
                });
            }); };
            this.nameByTwitterId = function (_a) {
                var twitterId = _a.twitterId;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                name_by_twitter_id: {
                                    twitter_id: twitterId
                                }
                            })];
                    });
                });
            };
            this.client = client;
            this.contractAddress = contractAddress;
            this.verifierPubKeys = this.verifierPubKeys.bind(this);
            this.verificationThreshold = this.verificationThreshold.bind(this);
            this.nameNftAddress = this.nameNftAddress.bind(this);
            this.referralCount = this.referralCount.bind(this);
            this.fee = this.fee.bind(this);
            this.nameByTwitterId = this.nameByTwitterId.bind(this);
        }
        return IcnsRegistrarQueryClient;
    }());
    var IcnsRegistrarClient = /** @class */ (function (_super) {
        __extends(IcnsRegistrarClient, _super);
        function IcnsRegistrarClient(client, sender, contractAddress) {
            var _this = _super.call(this, client, contractAddress) || this;
            _this.claim = function (_a, fee, memo, funds) {
                var name = _a.name, referral = _a.referral, verifications = _a.verifications, verifyingMsg = _a.verifyingMsg;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    claim: {
                                        name: name,
                                        referral: referral,
                                        verifications: verifications,
                                        verifying_msg: verifyingMsg
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.updateVerifierPubkeys = function (_a, fee, memo, funds) {
                var add = _a.add, remove = _a.remove;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    update_verifier_pubkeys: {
                                        add: add,
                                        remove: remove
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.setVerificationThreshold = function (_a, fee, memo, funds) {
                var threshold = _a.threshold;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    set_verification_threshold: {
                                        threshold: threshold
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.setNameNftAddress = function (_a, fee, memo, funds) {
                var nameNftAddress = _a.nameNftAddress;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    set_name_nft_address: {
                                        name_nft_address: nameNftAddress
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.setMintingFee = function (_a, fee, memo, funds) {
                var mintingFee = _a.mintingFee;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    set_minting_fee: {
                                        minting_fee: mintingFee
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.withdrawFunds = function (_a, fee, memo, funds) {
                var amount = _a.amount, toAddress = _a.toAddress;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    withdraw_funds: {
                                        amount: amount,
                                        to_address: toAddress
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.client = client;
            _this.sender = sender;
            _this.contractAddress = contractAddress;
            _this.claim = _this.claim.bind(_this);
            _this.updateVerifierPubkeys = _this.updateVerifierPubkeys.bind(_this);
            _this.setVerificationThreshold = _this.setVerificationThreshold.bind(_this);
            _this.setNameNftAddress = _this.setNameNftAddress.bind(_this);
            _this.setMintingFee = _this.setMintingFee.bind(_this);
            _this.withdrawFunds = _this.withdrawFunds.bind(_this);
            return _this;
        }
        return IcnsRegistrarClient;
    }(IcnsRegistrarQueryClient));

    var _3 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        IcnsRegistrarQueryClient: IcnsRegistrarQueryClient,
        IcnsRegistrarClient: IcnsRegistrarClient
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */

    var _4 = /*#__PURE__*/Object.freeze({
        __proto__: null
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var IcnsResolverQueryClient = /** @class */ (function () {
        function IcnsResolverQueryClient(client, contractAddress) {
            var _this = this;
            this.config = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            config: {}
                        })];
                });
            }); };
            this.addresses = function (_a) {
                var name = _a.name;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                addresses: {
                                    name: name
                                }
                            })];
                    });
                });
            };
            this.address = function (_a) {
                var bech32Prefix = _a.bech32Prefix, name = _a.name;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                address: {
                                    bech32_prefix: bech32Prefix,
                                    name: name
                                }
                            })];
                    });
                });
            };
            this.names = function (_a) {
                var address = _a.address;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                names: {
                                    address: address
                                }
                            })];
                    });
                });
            };
            this.icnsNames = function (_a) {
                var address = _a.address;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                icns_names: {
                                    address: address
                                }
                            })];
                    });
                });
            };
            this.primaryName = function (_a) {
                var address = _a.address;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                primary_name: {
                                    address: address
                                }
                            })];
                    });
                });
            };
            this.admin = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            admin: {}
                        })];
                });
            }); };
            this.addressByIcns = function (_a) {
                var icns = _a.icns;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                address_by_icns: {
                                    icns: icns
                                }
                            })];
                    });
                });
            };
            this.client = client;
            this.contractAddress = contractAddress;
            this.config = this.config.bind(this);
            this.addresses = this.addresses.bind(this);
            this.address = this.address.bind(this);
            this.names = this.names.bind(this);
            this.icnsNames = this.icnsNames.bind(this);
            this.primaryName = this.primaryName.bind(this);
            this.admin = this.admin.bind(this);
            this.addressByIcns = this.addressByIcns.bind(this);
        }
        return IcnsResolverQueryClient;
    }());
    var IcnsResolverClient = /** @class */ (function (_super) {
        __extends(IcnsResolverClient, _super);
        function IcnsResolverClient(client, sender, contractAddress) {
            var _this = _super.call(this, client, contractAddress) || this;
            _this.setRecord = function (_a, fee, memo, funds) {
                var adr36Info = _a.adr36Info, bech32Prefix = _a.bech32Prefix, name = _a.name;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    set_record: {
                                        adr36_info: adr36Info,
                                        bech32_prefix: bech32Prefix,
                                        name: name
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.setPrimary = function (_a, fee, memo, funds) {
                var bech32Address = _a.bech32Address, name = _a.name;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    set_primary: {
                                        bech32_address: bech32Address,
                                        name: name
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.removeRecord = function (_a, fee, memo, funds) {
                var bech32Address = _a.bech32Address, name = _a.name;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    remove_record: {
                                        bech32_address: bech32Address,
                                        name: name
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.client = client;
            _this.sender = sender;
            _this.contractAddress = contractAddress;
            _this.setRecord = _this.setRecord.bind(_this);
            _this.setPrimary = _this.setPrimary.bind(_this);
            _this.removeRecord = _this.removeRecord.bind(_this);
            return _this;
        }
        return IcnsResolverClient;
    }(IcnsResolverQueryClient));

    var _5 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        IcnsResolverQueryClient: IcnsResolverQueryClient,
        IcnsResolverClient: IcnsResolverClient
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    exports.contracts = void 0;
    (function (contracts) {
        contracts.IcnsNameNft = __assign(__assign({}, _0), _1);
        contracts.IcnsRegistrar = __assign(__assign({}, _2), _3);
        contracts.IcnsResolver = __assign(__assign({}, _4), _5);
    })(exports.contracts || (exports.contracts = {}));

    Object.defineProperty(exports, '__esModule', { value: true });

}));
//# sourceMappingURL=index.umd.js.map
