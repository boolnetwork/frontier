(function() {var implementors = {};
implementors["fc_consensus"] = [{"text":"impl&lt;B, I, C&gt; Sync for FrontierBlockImport&lt;B, I, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for Error","synthetic":true,"types":[]}];
implementors["fc_rpc"] = [{"text":"impl&lt;B, C, P, CT, BE, H&gt; Sync for EthApi&lt;B, C, P, CT, BE, H&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BE: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;CT: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;B as Block&gt;::Hash: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;B as Block&gt;::Header as Header&gt;::Number: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;B, C&gt; Sync for EthFilterApi&lt;B, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;B, BE, C, H&gt; Sync for NetApi&lt;B, BE, C, H&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BE: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;B as Block&gt;::Hash: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;B as Block&gt;::Header as Header&gt;::Number: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;B, C&gt; Sync for Web3Api&lt;B, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;B, P, C, BE, H&gt; Sync for EthPubSubApi&lt;B, P, C, BE, H&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BE: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;B as Block&gt;::Hash: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;B as Block&gt;::Header as Header&gt;::Number: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for HexEncodedIdProvider","synthetic":true,"types":[]},{"text":"impl Sync for EthDevSigner","synthetic":true,"types":[]}];
implementors["fc_rpc_core"] = [{"text":"impl Sync for AccountInfo","synthetic":true,"types":[]},{"text":"impl Sync for ExtAccountInfo","synthetic":true,"types":[]},{"text":"impl Sync for EthAccount","synthetic":true,"types":[]},{"text":"impl Sync for StorageProof","synthetic":true,"types":[]},{"text":"impl Sync for RecoveredAccount","synthetic":true,"types":[]},{"text":"impl Sync for Bytes","synthetic":true,"types":[]},{"text":"impl Sync for Block","synthetic":true,"types":[]},{"text":"impl Sync for Header","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for Rich&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for CallRequest","synthetic":true,"types":[]},{"text":"impl Sync for Filter","synthetic":true,"types":[]},{"text":"impl Sync for FilterPoolItem","synthetic":true,"types":[]},{"text":"impl Sync for FilteredParams","synthetic":true,"types":[]},{"text":"impl Sync for Index","synthetic":true,"types":[]},{"text":"impl Sync for Log","synthetic":true,"types":[]},{"text":"impl Sync for Receipt","synthetic":true,"types":[]},{"text":"impl Sync for SyncInfo","synthetic":true,"types":[]},{"text":"impl Sync for Peers","synthetic":true,"types":[]},{"text":"impl Sync for PeerInfo","synthetic":true,"types":[]},{"text":"impl Sync for PeerNetworkInfo","synthetic":true,"types":[]},{"text":"impl Sync for PeerProtocolsInfo","synthetic":true,"types":[]},{"text":"impl Sync for TransactionStats","synthetic":true,"types":[]},{"text":"impl Sync for ChainStatus","synthetic":true,"types":[]},{"text":"impl Sync for EthProtocolInfo","synthetic":true,"types":[]},{"text":"impl Sync for PipProtocolInfo","synthetic":true,"types":[]},{"text":"impl Sync for Transaction","synthetic":true,"types":[]},{"text":"impl Sync for RichRawTransaction","synthetic":true,"types":[]},{"text":"impl Sync for PendingTransaction","synthetic":true,"types":[]},{"text":"impl Sync for TransactionRequest","synthetic":true,"types":[]},{"text":"impl Sync for Work","synthetic":true,"types":[]},{"text":"impl Sync for BlockTransactions","synthetic":true,"types":[]},{"text":"impl Sync for BlockNumber","synthetic":true,"types":[]},{"text":"impl Sync for FilterChanges","synthetic":true,"types":[]},{"text":"impl Sync for FilterType","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for VariadicValue&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for SyncStatus","synthetic":true,"types":[]},{"text":"impl Sync for LocalTransactionStatus","synthetic":true,"types":[]},{"text":"impl Sync for PubSubSyncStatus","synthetic":true,"types":[]},{"text":"impl Sync for Result","synthetic":true,"types":[]},{"text":"impl Sync for Kind","synthetic":true,"types":[]},{"text":"impl Sync for Params","synthetic":true,"types":[]}];
implementors["fp_consensus"] = [{"text":"impl Sync for ConsensusLog","synthetic":true,"types":[]}];
implementors["fp_evm"] = [{"text":"impl Sync for Vicinity","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for ExecutionInfo&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for CallOrCreateInfo","synthetic":true,"types":[]}];
implementors["fp_rpc"] = [{"text":"impl Sync for TransactionStatus","synthetic":true,"types":[]}];
implementors["frontier_template_node"] = [{"text":"impl Sync for Executor","synthetic":true,"types":[]},{"text":"impl Sync for MockTimestampInherentDataProvider","synthetic":true,"types":[]},{"text":"impl Sync for ConsensusResult","synthetic":true,"types":[]},{"text":"impl Sync for RunCmd","synthetic":true,"types":[]},{"text":"impl Sync for Cli","synthetic":true,"types":[]},{"text":"impl Sync for Sealing","synthetic":true,"types":[]},{"text":"impl Sync for Subcommand","synthetic":true,"types":[]},{"text":"impl&lt;C, F, P&gt; Sync for LightDeps&lt;C, F, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;C, P&gt; Sync for FullDeps&lt;C, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["frontier_template_runtime"] = [{"text":"impl Sync for Version","synthetic":true,"types":[]},{"text":"impl Sync for BlockHashCount","synthetic":true,"types":[]},{"text":"impl Sync for BlockWeights","synthetic":true,"types":[]},{"text":"impl Sync for BlockLength","synthetic":true,"types":[]},{"text":"impl Sync for SS58Prefix","synthetic":true,"types":[]},{"text":"impl Sync for MinimumPeriod","synthetic":true,"types":[]},{"text":"impl Sync for ExistentialDeposit","synthetic":true,"types":[]},{"text":"impl Sync for MaxLocks","synthetic":true,"types":[]},{"text":"impl Sync for TransactionByteFee","synthetic":true,"types":[]},{"text":"impl Sync for FixedGasPrice","synthetic":true,"types":[]},{"text":"impl Sync for ChainId","synthetic":true,"types":[]},{"text":"impl&lt;F&gt; Sync for EthereumFindAuthor&lt;F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for BlockGasLimit","synthetic":true,"types":[]},{"text":"impl Sync for Runtime","synthetic":true,"types":[]},{"text":"impl !Sync for Origin","synthetic":true,"types":[]},{"text":"impl Sync for PalletInfo","synthetic":true,"types":[]},{"text":"impl Sync for GenesisConfig","synthetic":true,"types":[]},{"text":"impl Sync for TransactionConverter","synthetic":true,"types":[]},{"text":"impl Sync for RuntimeApi","synthetic":true,"types":[]},{"text":"impl Sync for Event","synthetic":true,"types":[]},{"text":"impl Sync for OriginCaller","synthetic":true,"types":[]},{"text":"impl Sync for Call","synthetic":true,"types":[]},{"text":"impl Sync for SessionKeys","synthetic":true,"types":[]},{"text":"impl&lt;Block:&nbsp;BlockT, C:&nbsp;CallApiAt&lt;Block&gt;&gt; Sync for RuntimeApiImpl&lt;Block, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C::StateBackend: StateBackend&lt;HashFor&lt;Block&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["frontier_template_test_client"] = [{"text":"impl Sync for LocalExecutor","synthetic":true,"types":[]},{"text":"impl Sync for GenesisParameters","synthetic":true,"types":[]}];
implementors["pallet_dynamic_fee"] = [{"text":"impl Sync for GenesisConfig","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for Module&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for InherentDataProvider","synthetic":true,"types":[]},{"text":"impl Sync for Event","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for InherentError","synthetic":true,"types":[]}];
implementors["pallet_ethereum"] = [{"text":"impl Sync for IntermediateStateRoot","synthetic":true,"types":[]},{"text":"impl Sync for GenesisConfig","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for Module&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for ReturnValue","synthetic":true,"types":[]},{"text":"impl Sync for Event","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for Error&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["pallet_evm"] = [{"text":"impl Sync for EnsureAddressSame","synthetic":true,"types":[]},{"text":"impl&lt;AccountId&gt; Sync for EnsureAddressRoot&lt;AccountId&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;AccountId&gt; Sync for EnsureAddressNever&lt;AccountId&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for EnsureAddressTruncated","synthetic":true,"types":[]},{"text":"impl Sync for IdentityAddressMapping","synthetic":true,"types":[]},{"text":"impl&lt;H&gt; Sync for HashedAddressMapping&lt;H&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;H: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for GenesisAccount","synthetic":true,"types":[]},{"text":"impl Sync for GenesisConfig","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for Module&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;AccountId&gt; Sync for RawEvent&lt;AccountId&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for Error&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Config&gt;::Currency as Currency&lt;&lt;T as Config&gt;::AccountId&gt;&gt;::Balance: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for Runner&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'vicinity, 'config, T&gt; Sync for SubstrateStackState&lt;'vicinity, 'config, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["pallet_evm_precompile_blake2"] = [{"text":"impl Sync for Blake2F","synthetic":true,"types":[]}];
implementors["pallet_evm_precompile_bn128"] = [{"text":"impl Sync for Bn128Add","synthetic":true,"types":[]},{"text":"impl Sync for Bn128Mul","synthetic":true,"types":[]},{"text":"impl Sync for Bn128Pairing","synthetic":true,"types":[]}];
implementors["pallet_evm_precompile_dispatch"] = [{"text":"impl&lt;T&gt; Sync for Dispatch&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["pallet_evm_precompile_ed25519"] = [{"text":"impl Sync for Ed25519Verify","synthetic":true,"types":[]}];
implementors["pallet_evm_precompile_modexp"] = [{"text":"impl Sync for Modexp","synthetic":true,"types":[]}];
implementors["pallet_evm_precompile_simple"] = [{"text":"impl Sync for Identity","synthetic":true,"types":[]},{"text":"impl Sync for ECRecover","synthetic":true,"types":[]},{"text":"impl Sync for Ripemd160","synthetic":true,"types":[]},{"text":"impl Sync for Sha256","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()