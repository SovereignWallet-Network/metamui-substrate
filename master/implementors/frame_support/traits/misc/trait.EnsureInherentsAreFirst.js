(function() {var implementors = {
"frame_support_test_compile_pass":[["impl <a class=\"trait\" href=\"frame_support/traits/misc/trait.EnsureInherentsAreFirst.html\" title=\"trait frame_support::traits::misc::EnsureInherentsAreFirst\">EnsureInherentsAreFirst</a>&lt;<a class=\"struct\" href=\"sp_runtime/generic/block/struct.Block.html\" title=\"struct sp_runtime::generic::block::Block\">Block</a>&lt;<a class=\"struct\" href=\"sp_runtime/generic/header/struct.Header.html\" title=\"struct sp_runtime::generic::header::Header\">Header</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>, <a class=\"struct\" href=\"sp_runtime/traits/struct.BlakeTwo256.html\" title=\"struct sp_runtime::traits::BlakeTwo256\">BlakeTwo256</a>&gt;, <a class=\"struct\" href=\"sp_runtime/generic/unchecked_extrinsic/struct.UncheckedExtrinsic.html\" title=\"struct sp_runtime::generic::unchecked_extrinsic::UncheckedExtrinsic\">UncheckedExtrinsic</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>, <a class=\"enum\" href=\"frame_support_test_compile_pass/enum.RuntimeCall.html\" title=\"enum frame_support_test_compile_pass::RuntimeCall\">RuntimeCall</a>, <a class=\"struct\" href=\"sp_core/sr25519/struct.Signature.html\" title=\"struct sp_core::sr25519::Signature\">Signature</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;&gt;&gt; for <a class=\"struct\" href=\"frame_support_test_compile_pass/struct.Runtime.html\" title=\"struct frame_support_test_compile_pass::Runtime\">Runtime</a>"]],
"kitchensink_runtime":[["impl <a class=\"trait\" href=\"frame_support/traits/misc/trait.EnsureInherentsAreFirst.html\" title=\"trait frame_support::traits::misc::EnsureInherentsAreFirst\">EnsureInherentsAreFirst</a>&lt;<a class=\"struct\" href=\"sp_runtime/generic/block/struct.Block.html\" title=\"struct sp_runtime::generic::block::Block\">Block</a>&lt;<a class=\"struct\" href=\"sp_runtime/generic/header/struct.Header.html\" title=\"struct sp_runtime::generic::header::Header\">Header</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>, <a class=\"struct\" href=\"sp_runtime/traits/struct.BlakeTwo256.html\" title=\"struct sp_runtime::traits::BlakeTwo256\">BlakeTwo256</a>&gt;, <a class=\"struct\" href=\"sp_runtime/generic/unchecked_extrinsic/struct.UncheckedExtrinsic.html\" title=\"struct sp_runtime::generic::unchecked_extrinsic::UncheckedExtrinsic\">UncheckedExtrinsic</a>&lt;<a class=\"enum\" href=\"sp_runtime/multiaddress/enum.MultiAddress.html\" title=\"enum sp_runtime::multiaddress::MultiAddress\">MultiAddress</a>&lt;&lt;&lt;<a class=\"enum\" href=\"sp_runtime/enum.MultiSignature.html\" title=\"enum sp_runtime::MultiSignature\">MultiSignature</a> as <a class=\"trait\" href=\"sp_runtime/traits/trait.Verify.html\" title=\"trait sp_runtime::traits::Verify\">Verify</a>&gt;::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.Verify.html#associatedtype.Signer\" title=\"type sp_runtime::traits::Verify::Signer\">Signer</a> as <a class=\"trait\" href=\"sp_runtime/traits/trait.IdentifyAccount.html\" title=\"trait sp_runtime::traits::IdentifyAccount\">IdentifyAccount</a>&gt;::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.IdentifyAccount.html#associatedtype.AccountId\" title=\"type sp_runtime::traits::IdentifyAccount::AccountId\">AccountId</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt;, <a class=\"enum\" href=\"kitchensink_runtime/enum.RuntimeCall.html\" title=\"enum kitchensink_runtime::RuntimeCall\">RuntimeCall</a>, <a class=\"enum\" href=\"sp_runtime/enum.MultiSignature.html\" title=\"enum sp_runtime::MultiSignature\">MultiSignature</a>, (<a class=\"struct\" href=\"frame_system/extensions/check_non_zero_sender/struct.CheckNonZeroSender.html\" title=\"struct frame_system::extensions::check_non_zero_sender::CheckNonZeroSender\">CheckNonZeroSender</a>&lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"frame_system/extensions/check_spec_version/struct.CheckSpecVersion.html\" title=\"struct frame_system::extensions::check_spec_version::CheckSpecVersion\">CheckSpecVersion</a>&lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"frame_system/extensions/check_tx_version/struct.CheckTxVersion.html\" title=\"struct frame_system::extensions::check_tx_version::CheckTxVersion\">CheckTxVersion</a>&lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"frame_system/extensions/check_genesis/struct.CheckGenesis.html\" title=\"struct frame_system::extensions::check_genesis::CheckGenesis\">CheckGenesis</a>&lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"frame_system/extensions/check_mortality/struct.CheckMortality.html\" title=\"struct frame_system::extensions::check_mortality::CheckMortality\">CheckMortality</a>&lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"frame_system/extensions/check_nonce/struct.CheckNonce.html\" title=\"struct frame_system::extensions::check_nonce::CheckNonce\">CheckNonce</a>&lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"frame_system/extensions/check_weight/struct.CheckWeight.html\" title=\"struct frame_system::extensions::check_weight::CheckWeight\">CheckWeight</a>&lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"pallet_asset_tx_payment/struct.ChargeAssetTxPayment.html\" title=\"struct pallet_asset_tx_payment::ChargeAssetTxPayment\">ChargeAssetTxPayment</a>&lt;<a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a>&gt;)&gt;&gt;&gt; for <a class=\"struct\" href=\"kitchensink_runtime/struct.Runtime.html\" title=\"struct kitchensink_runtime::Runtime\">Runtime</a>"]],
"node_template_runtime":[["impl <a class=\"trait\" href=\"frame_support/traits/misc/trait.EnsureInherentsAreFirst.html\" title=\"trait frame_support::traits::misc::EnsureInherentsAreFirst\">EnsureInherentsAreFirst</a>&lt;<a class=\"struct\" href=\"sp_runtime/generic/block/struct.Block.html\" title=\"struct sp_runtime::generic::block::Block\">Block</a>&lt;<a class=\"struct\" href=\"sp_runtime/generic/header/struct.Header.html\" title=\"struct sp_runtime::generic::header::Header\">Header</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>, <a class=\"struct\" href=\"sp_runtime/traits/struct.BlakeTwo256.html\" title=\"struct sp_runtime::traits::BlakeTwo256\">BlakeTwo256</a>&gt;, <a class=\"struct\" href=\"sp_runtime/generic/unchecked_extrinsic/struct.UncheckedExtrinsic.html\" title=\"struct sp_runtime::generic::unchecked_extrinsic::UncheckedExtrinsic\">UncheckedExtrinsic</a>&lt;<a class=\"enum\" href=\"sp_runtime/multiaddress/enum.MultiAddress.html\" title=\"enum sp_runtime::multiaddress::MultiAddress\">MultiAddress</a>&lt;&lt;&lt;<a class=\"enum\" href=\"sp_runtime/enum.MultiSignature.html\" title=\"enum sp_runtime::MultiSignature\">MultiSignature</a> as <a class=\"trait\" href=\"sp_runtime/traits/trait.Verify.html\" title=\"trait sp_runtime::traits::Verify\">Verify</a>&gt;::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.Verify.html#associatedtype.Signer\" title=\"type sp_runtime::traits::Verify::Signer\">Signer</a> as <a class=\"trait\" href=\"sp_runtime/traits/trait.IdentifyAccount.html\" title=\"trait sp_runtime::traits::IdentifyAccount\">IdentifyAccount</a>&gt;::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.IdentifyAccount.html#associatedtype.AccountId\" title=\"type sp_runtime::traits::IdentifyAccount::AccountId\">AccountId</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;, <a class=\"enum\" href=\"node_template_runtime/enum.RuntimeCall.html\" title=\"enum node_template_runtime::RuntimeCall\">RuntimeCall</a>, <a class=\"enum\" href=\"sp_runtime/enum.MultiSignature.html\" title=\"enum sp_runtime::MultiSignature\">MultiSignature</a>, (<a class=\"struct\" href=\"frame_system/extensions/check_non_zero_sender/struct.CheckNonZeroSender.html\" title=\"struct frame_system::extensions::check_non_zero_sender::CheckNonZeroSender\">CheckNonZeroSender</a>&lt;<a class=\"struct\" href=\"node_template_runtime/struct.Runtime.html\" title=\"struct node_template_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"frame_system/extensions/check_spec_version/struct.CheckSpecVersion.html\" title=\"struct frame_system::extensions::check_spec_version::CheckSpecVersion\">CheckSpecVersion</a>&lt;<a class=\"struct\" href=\"node_template_runtime/struct.Runtime.html\" title=\"struct node_template_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"frame_system/extensions/check_tx_version/struct.CheckTxVersion.html\" title=\"struct frame_system::extensions::check_tx_version::CheckTxVersion\">CheckTxVersion</a>&lt;<a class=\"struct\" href=\"node_template_runtime/struct.Runtime.html\" title=\"struct node_template_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"frame_system/extensions/check_genesis/struct.CheckGenesis.html\" title=\"struct frame_system::extensions::check_genesis::CheckGenesis\">CheckGenesis</a>&lt;<a class=\"struct\" href=\"node_template_runtime/struct.Runtime.html\" title=\"struct node_template_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"frame_system/extensions/check_mortality/struct.CheckMortality.html\" title=\"struct frame_system::extensions::check_mortality::CheckMortality\">CheckMortality</a>&lt;<a class=\"struct\" href=\"node_template_runtime/struct.Runtime.html\" title=\"struct node_template_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"frame_system/extensions/check_nonce/struct.CheckNonce.html\" title=\"struct frame_system::extensions::check_nonce::CheckNonce\">CheckNonce</a>&lt;<a class=\"struct\" href=\"node_template_runtime/struct.Runtime.html\" title=\"struct node_template_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"frame_system/extensions/check_weight/struct.CheckWeight.html\" title=\"struct frame_system::extensions::check_weight::CheckWeight\">CheckWeight</a>&lt;<a class=\"struct\" href=\"node_template_runtime/struct.Runtime.html\" title=\"struct node_template_runtime::Runtime\">Runtime</a>&gt;, <a class=\"struct\" href=\"pallet_transaction_payment/struct.ChargeTransactionPayment.html\" title=\"struct pallet_transaction_payment::ChargeTransactionPayment\">ChargeTransactionPayment</a>&lt;<a class=\"struct\" href=\"node_template_runtime/struct.Runtime.html\" title=\"struct node_template_runtime::Runtime\">Runtime</a>&gt;)&gt;&gt;&gt; for <a class=\"struct\" href=\"node_template_runtime/struct.Runtime.html\" title=\"struct node_template_runtime::Runtime\">Runtime</a>"]],
"pallet_bags_list":[["impl EnsureInherentsAreFirst&lt;<a class=\"struct\" href=\"sp_runtime/generic/block/struct.Block.html\" title=\"struct sp_runtime::generic::block::Block\">Block</a>&lt;<a class=\"struct\" href=\"sp_runtime/generic/header/struct.Header.html\" title=\"struct sp_runtime::generic::header::Header\">Header</a>&lt;&lt;<a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.BlockNumber\" title=\"type frame_system::pallet::Config::BlockNumber\">BlockNumber</a>, <a class=\"struct\" href=\"sp_runtime/traits/struct.BlakeTwo256.html\" title=\"struct sp_runtime::traits::BlakeTwo256\">BlakeTwo256</a>&gt;, <a class=\"struct\" href=\"sp_runtime/generic/unchecked_extrinsic/struct.UncheckedExtrinsic.html\" title=\"struct sp_runtime::generic::unchecked_extrinsic::UncheckedExtrinsic\">UncheckedExtrinsic</a>&lt;&lt;<a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.AccountId\" title=\"type frame_system::pallet::Config::AccountId\">AccountId</a>, &lt;<a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.RuntimeCall\" title=\"type frame_system::pallet::Config::RuntimeCall\">RuntimeCall</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;&gt;&gt; for <a class=\"struct\" href=\"pallet_bags_list/mock/struct.Runtime.html\" title=\"struct pallet_bags_list::mock::Runtime\">Runtime</a>"]],
"pallet_nomination_pools":[["impl <a class=\"trait\" href=\"frame_support/traits/misc/trait.EnsureInherentsAreFirst.html\" title=\"trait frame_support::traits::misc::EnsureInherentsAreFirst\">EnsureInherentsAreFirst</a>&lt;<a class=\"struct\" href=\"sp_runtime/generic/block/struct.Block.html\" title=\"struct sp_runtime::generic::block::Block\">Block</a>&lt;<a class=\"struct\" href=\"sp_runtime/generic/header/struct.Header.html\" title=\"struct sp_runtime::generic::header::Header\">Header</a>&lt;&lt;<a class=\"struct\" href=\"pallet_nomination_pools/mock/struct.Runtime.html\" title=\"struct pallet_nomination_pools::mock::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.BlockNumber\" title=\"type frame_system::pallet::Config::BlockNumber\">BlockNumber</a>, <a class=\"struct\" href=\"sp_runtime/traits/struct.BlakeTwo256.html\" title=\"struct sp_runtime::traits::BlakeTwo256\">BlakeTwo256</a>&gt;, <a class=\"struct\" href=\"sp_runtime/generic/unchecked_extrinsic/struct.UncheckedExtrinsic.html\" title=\"struct sp_runtime::generic::unchecked_extrinsic::UncheckedExtrinsic\">UncheckedExtrinsic</a>&lt;&lt;<a class=\"struct\" href=\"pallet_nomination_pools/mock/struct.Runtime.html\" title=\"struct pallet_nomination_pools::mock::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.AccountId\" title=\"type frame_system::pallet::Config::AccountId\">AccountId</a>, &lt;<a class=\"struct\" href=\"pallet_nomination_pools/mock/struct.Runtime.html\" title=\"struct pallet_nomination_pools::mock::Runtime\">Runtime</a> as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.RuntimeCall\" title=\"type frame_system::pallet::Config::RuntimeCall\">RuntimeCall</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;&gt;&gt; for <a class=\"struct\" href=\"pallet_nomination_pools/mock/struct.Runtime.html\" title=\"struct pallet_nomination_pools::mock::Runtime\">Runtime</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()